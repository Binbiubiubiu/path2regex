//! Path parser
mod builder;

use anyhow::{anyhow, Result};
use std::cell::Cell;

use crate::{
    ast::{LexToken, LexTokenKind},
    internal::escape_string,
    Key, Token, TryIntoWith,
};

pub use builder::{ParserBuilder, ParserOptions};

/// Path parser
#[derive(Debug, Clone)]
pub struct Parser(pub(crate) ParserOptions);

impl Parser {
    /// Create a [Parser](struct.Parser.html)
    #[inline]
    pub fn new() -> Self {
        ParserBuilder::new().build()
    }

    /// Create a [Parser](struct.Parser.html) with the options
    #[inline]
    pub fn new_with_options(options: ParserOptions) -> Self {
        Self(options)
    }

    /// Parse the path to the lexical
    pub fn parse_str(&self, input: impl AsRef<str>) -> Result<Vec<Token>> {
        input.as_ref().try_into_with(&self.0)
    }
}

impl Default for Parser {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// lex word parser
#[inline]
fn lexer(input: &str) -> Result<Vec<LexToken<'_>>> {
    use LexTokenKind::*;

    let mut tokens = vec![];
    let mut i = 0;
    let char_vec: Vec<_> = input.chars().collect();
    while i < char_vec.len() {
        match char_vec[i] {
            '*' | '+' | '?' => {
                tokens.push(LexToken {
                    kind: Modifier,
                    index: i,
                    value: &input[i..i + 1],
                });
                i += 1;
            }
            '\\' => {
                tokens.push(LexToken {
                    kind: EscapedChar,
                    index: i,
                    value: &input[i + 1..i + 2],
                });
                i += 2;
            }
            '{' => {
                tokens.push(LexToken {
                    kind: Open,
                    index: i,
                    value: &input[i..i + 1],
                });
                i += 1;
            }
            '}' => {
                tokens.push(LexToken {
                    kind: Close,
                    index: i,
                    value: &input[i..i + 1],
                });
                i += 1;
            }
            ':' => {
                let mut j = i + 1;
                while j < input.len() {
                    match char_vec[j] {
                        '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' => {
                            j += 1;
                            continue;
                        }
                        _ => break,
                    }
                }

                let name = &input[i + 1..j];

                if name.is_empty() {
                    return Err(anyhow!("Missing parameter name at {i}"));
                }
                tokens.push(LexToken {
                    kind: Name,
                    index: i,
                    value: name,
                });
                i = j;
            }
            '(' => {
                let mut count = 1;
                let mut pattern = "";
                let mut j = i + 1;

                if char_vec[j] == '?' {
                    return Err(anyhow!("Pattern cannot start with \"?\" at {j}"));
                }

                while j < input.len() {
                    match char_vec[j] {
                        '\\' => {
                            j += 2;
                            continue;
                        }
                        ')' => {
                            count -= 1;
                            if count == 0 {
                                j += 1;
                                break;
                            }
                        }
                        '(' => {
                            count += 1;
                            let it = char_vec.get(j + 1);
                            if it.is_none() || matches!(it, Some(&x) if x != '?') {
                                return Err(anyhow!("Capturing groups are not allowed at {j}"));
                            }
                        }
                        _ => {}
                    };

                    pattern = &input[i + 1..j + 1];
                    j += 1;
                }
                if count > 0 {
                    return Err(anyhow!("Unbalanced pattern at {i}"));
                }

                if pattern.is_empty() {
                    return Err(anyhow!("Missing pattern at {i}"));
                }

                tokens.push(LexToken {
                    kind: Pattern,
                    index: i,
                    value: pattern,
                });
                i = j;
            }
            _ => {
                tokens.push(LexToken {
                    kind: Char,
                    index: i,
                    value: &input[i..i + 1],
                });
                i += 1;
            }
        };
    }

    tokens.push(LexToken {
        kind: End,
        index: i,
        value: "",
    });

    Ok(tokens)
}

/// Parse the path to the lexical with Some options
#[inline]
pub(crate) fn parse_str_with_options(
    input: impl AsRef<str>,
    options: &ParserOptions,
) -> Result<Vec<Token>> {
    let ParserOptions {
        delimiter,
        prefixes,
    } = options;

    use LexTokenKind::*;
    let input = input.as_ref();
    let tokens = lexer(input)?;
    let mut result = vec![];
    let default_pattern = format!("[^{}]+?", escape_string(delimiter));

    let mut key: usize = 0;
    let i: Cell<usize> = Cell::new(0);
    let mut path = String::new();

    let try_consume = |ty: LexTokenKind| match i.get() {
        n if n < tokens.len() && tokens[n].kind == ty => {
            let value = tokens[n].value;
            i.set(n + 1);
            Some(value)
        }
        _ => None,
    };

    let must_consume = |ty: LexTokenKind| {
        let value = try_consume(ty);
        match value {
            Some(v) => Ok(v),
            None => {
                let LexToken { kind, index, .. } = &tokens[i.get()];
                Err(anyhow!("Unexpected {kind} at {index}, expected {ty}"))
            }
        }
    };

    let consume_text = || {
        let mut result = String::new();
        while let Some(t) = try_consume(Char).or_else(|| try_consume(EscapedChar)) {
            result += t;
        }
        result
    };

    while i.get() < tokens.len() {
        let char = try_consume(Char);
        let name = try_consume(Name);
        let pattern = try_consume(Pattern);

        if name.or(pattern).is_some() {
            let mut prefix = char.unwrap_or_default();

            if !prefixes.contains(prefix) {
                path += prefix;
                prefix = ""
            }

            if !path.is_empty() {
                result.push(Token::Static(path));
                path = String::new();
            }

            result.push(Token::Key(Key {
                name: name.map_or_else(
                    || {
                        let k = key;
                        key += 1;
                        k.to_string()
                    },
                    |x| x.to_owned(),
                ),
                prefix: prefix.to_owned(),
                suffix: String::new(),
                pattern: pattern.map_or_else(|| default_pattern.clone(), |x| x.to_owned()),
                modifier: try_consume(Modifier).unwrap_or_default().to_owned(),
            }));
            continue;
        }

        if let Some(value) = char.or_else(|| try_consume(EscapedChar)) {
            path += value;
            continue;
        }

        if !path.is_empty() {
            result.push(Token::Static(path));
            path = String::new();
        }

        if try_consume(Open).is_some() {
            let prefix = consume_text();
            let name = try_consume(Name);
            let pattern = try_consume(Pattern);
            let suffix = consume_text();

            must_consume(Close)?;

            result.push(Token::Key(Key {
                name: name.map_or_else(
                    || {
                        if pattern.is_some() {
                            let k = key;
                            key += 1;
                            k.to_string()
                        } else {
                            Default::default()
                        }
                    },
                    |x| x.to_owned(),
                ),
                pattern: if name.is_some() && pattern.is_none() {
                    default_pattern.clone()
                } else {
                    pattern.unwrap_or_default().to_owned()
                },
                prefix,
                suffix,
                modifier: try_consume(Modifier).unwrap_or_default().to_owned(),
            }));

            continue;
        }

        must_consume(End)?;
    }

    Ok(result)
}
