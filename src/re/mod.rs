//! Path regex
mod builder;

use anyhow::Result;

use regex::{Regex, RegexBuilder};

pub use builder::{PathRegexBuilder, PathRegexOptions};

use crate::{
    internal::{escape_string, END_WITH_DELIMITER},
    Key, Parser, ParserOptions, Token, TryIntoWith,
};

/// Path regex
#[derive(Clone)]
pub struct PathRegex {
    pub(crate) re: Regex,
    pub(crate) keys: Vec<Key>,
}

impl PathRegex {
    /// Create a [`PathRegex`](struct.PathRegex.html)
    #[inline]
    pub fn new<S>(source: S) -> Result<Self>
    where
        S: TryIntoWith<PathRegex, PathRegexOptions>,
    {
        PathRegexBuilder::new(source).build()
    }

    /// Create a [`PathRegex`](struct.PathRegex.html) with the options
    #[inline]
    pub fn new_with_options<S>(source: S, options: PathRegexOptions) -> Result<Self>
    where
        S: TryIntoWith<PathRegex, PathRegexOptions>,
    {
        PathRegexBuilder::new_with_options(source, options).build()
    }

    /// Get then parameter matches in the path
    pub fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}

impl std::fmt::Display for PathRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::fmt::Debug for PathRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.re.as_str())
    }
}

impl AsRef<Regex> for PathRegex {
    #[inline]
    fn as_ref(&self) -> &Regex {
        &self.re
    }
}

impl std::ops::Deref for PathRegex {
    type Target = Regex;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.re
    }
}

///
#[inline]
pub(crate) fn regex_to_path_regex(path: Regex, keys: &mut Vec<Key>) -> Result<Regex> {
    if keys.is_empty() {
        return Ok(path);
    }

    let groups_regex = RegexBuilder::new(r"\((?:\?<(.*?)>)?").build()?;

    let mut index: usize = 0;
    for name in groups_regex.captures_iter(path.as_str()) {
        keys.push(Key {
            name: name.get(1).map_or_else(
                || {
                    let p = index;
                    index += 1;
                    format!("{p}")
                },
                |m| m.as_str().to_owned(),
            ),
            prefix: Default::default(),
            suffix: Default::default(),
            pattern: Default::default(),
            modifier: Default::default(),
        });
    }

    Ok(path)
}

///
#[inline]
fn tokens_to_path_regex(
    tokens: Vec<Token>,
    keys: &mut Vec<Key>,
    options: &PathRegexOptions,
) -> Result<Regex, regex::Error> {
    let PathRegexOptions {
        sensitive,
        strict,
        end,
        start,
        delimiter,
        ends_with,
        encode,
        ..
    } = options;
    let ends_with_re = (!ends_with.is_empty())
        .then(|| format!("[{}]|$", escape_string(ends_with)))
        .unwrap_or_else(|| "$".to_string());
    let delimiter_re = (!delimiter.is_empty())
        .then(|| format!("[{}]", escape_string(delimiter)))
        .unwrap_or_default();
    let route = if *start { "^" } else { "" };
    let mut route = String::from(route);

    for token in tokens.iter() {
        match token {
            Token::Static(token) => route += &escape_string(&encode(token)),
            Token::Key(token) => {
                let Key {
                    prefix,
                    suffix,
                    pattern,
                    modifier,
                    ..
                } = token;
                let prefix = escape_string(&encode(prefix));
                let suffix = escape_string(&encode(suffix));

                if !pattern.is_empty() {
                    keys.push(token.clone());

                    if !prefix.is_empty() || !suffix.is_empty() {
                        let modifier = modifier.as_str();
                        if matches!(modifier, "+" | "*") {
                            let mo = if modifier == "*" { "?" } else { "" };
                            route += &format!(
                                "(?:{prefix}((?:{pattern})(?:{suffix}{prefix}(?:{pattern}))*){suffix}){mo}"
                            );
                        } else {
                            route += &format!("(?:{prefix}({pattern}){suffix}){modifier}");
                        }
                    } else {
                        let modifier = token.modifier.as_str();
                        if matches!(modifier, "+" | "*") {
                            route += &format!("((?:{pattern}){modifier})");
                        } else {
                            route += &format!("({pattern}){modifier}");
                        }
                    }
                } else {
                    route += &format!("(?:{prefix}{suffix}){modifier}");
                }
            }
        }
    }

    if *end {
        if !strict {
            route += &format!("{delimiter_re}?");
        }
        route += "$";
        if ends_with.is_empty() {
            route += "$";
        } else {
            route += &format!("(?P<{END_WITH_DELIMITER}>{ends_with_re})");
        };
    } else {
        let end_token = tokens.last();
        let is_end_delimited = match end_token {
            Some(token) => match token {
                Token::Static(end_token) if !end_token.is_empty() => {
                    delimiter_re.contains(end_token.chars().last().unwrap())
                }
                _ => false,
            },
            None => true,
        };

        if !strict {
            route += &format!("(?:{delimiter_re}{ends_with_re})?");
        }

        if !is_end_delimited {
            route += &format!("(?P<{END_WITH_DELIMITER}>{delimiter_re}|{ends_with_re})");
        }
    }

    RegexBuilder::new(&route)
        .case_insensitive(!sensitive)
        .build()
}

#[inline]
pub(crate) fn string_to_path_regex<S>(path: S, options: &PathRegexOptions) -> Result<PathRegex>
where
    S: AsRef<str>,
{
    let mut keys = vec![];
    let tokens = Parser::new_with_options(ParserOptions::from(options.clone())).parse_str(path)?;

    let re = tokens_to_path_regex(tokens, &mut keys, options)?;
    Ok(PathRegex { re, keys })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn test_compile_tokens_to_regexp() -> anyhow::Result<()> {
        let tokens = Parser::new().parse_str("/user/:id")?;
        let re = tokens_to_path_regex(tokens, &mut vec![], &Default::default())?;
        let matches = re
            .captures("/user/123")
            .unwrap()
            .iter()
            .map(|x| match x {
                Some(x) => x.as_str(),
                None => Default::default(),
            })
            .collect::<Vec<_>>();
        assert_eq!(matches, vec!["/user/123", "123"]);
        Ok(())
    }
}
