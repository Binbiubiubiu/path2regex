use eyre::Result;

use regex::{Regex, RegexBuilder};

use crate::{
    escape_string,
    parser::{parse_with_options, ParserOptions},
    re_builder::PathRegexOptions,
    Key, PathRegexBuilder, Token,
};

///
#[derive(Clone)]
pub struct PathRegex {
    pub(crate) re: Regex,
    ///
    pub keys: Vec<Key>,
}

impl PathRegex {
    ///
    pub fn new<S>(path: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        PathRegexBuilder::new(path.as_ref()).build()
    }
}

impl From<Regex> for PathRegex {
    fn from(re: Regex) -> Self {
        PathRegexBuilder::new(re).build().unwrap()
    }
}

impl std::str::FromStr for PathRegex {
    type Err = eyre::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathRegexBuilder::new(s).build()
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
    fn as_ref(&self) -> &Regex {
        &self.re
    }
}

impl std::ops::Deref for PathRegex {
    type Target = Regex;

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
pub(crate) fn tokens_to_path_regex(
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
    let ends_with_re = format!("[{}]|$", escape_string(ends_with));
    let delimiter_re = format!("[{}]", escape_string(delimiter));
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
                                "(?:${prefix}((?:{pattern})(?:{suffix}{prefix}(?:{pattern}))*)${suffix})${mo}"
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
        if !*strict {
            route += &format!("{delimiter_re}?");
        }

        if ends_with.is_empty() {
            route += "$";
        } else {
            route += &format!("(?={ends_with_re})");
        };
    } else {
        let end_token = tokens.last();
        let is_end_delimited = match end_token {
            Some(token) => match token {
                Token::Static(end_token) if !end_token.is_empty() => delimiter_re
                    .find(end_token.chars().last().unwrap())
                    .is_some(),
                _ => false,
            },
            None => true,
        };

        if !strict {
            route += &format!("(?:${delimiter_re}(?=${ends_with_re}))?");
        }

        if !is_end_delimited {
            route += &format!("(?=${delimiter_re}|${ends_with_re})");
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
    let tokens = parse_with_options(path, &ParserOptions::from(options.clone()))?;

    let re = tokens_to_path_regex(tokens, &mut keys, options)?;
    Ok(PathRegex { re, keys })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_path_regex() {
        
    }
}