//! Parse the path to the lexical

use anyhow::{anyhow, Result};
use std::{
    cell::Cell,
    cmp::{Eq, PartialEq},
};

use crate::escape_string;

macro_rules! lex_token_kind {
    ($($ty:tt $name:tt)+) => {
        #[derive(PartialEq,Eq,Copy,Clone)]
        pub(crate) enum LexTokenKind {
            $($ty,)+
        }

        impl std::fmt::Display for LexTokenKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(LexTokenKind::$ty => $name,)+
                };
                f.write_str(name)
            }
        }

        impl std::fmt::Debug for LexTokenKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(LexTokenKind::$ty => $name,)+
                };
                f.write_str(name)
            }
        }
    };
}

lex_token_kind! {
    Open "OPEN"
    Close "CLOSE"
    Pattern "PATTERN"
    Name "NAME"
    Char "CHAR"
    EscapedChar "ESCAPEDCHAR"
    Modifier "MODIFIER"
    End "END"
}

struct LexToken<'a> {
    kind: LexTokenKind,
    index: usize,
    value: &'a str,
}

impl<'a> std::fmt::Display for LexToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("type", &self.kind)
            .field("index", &self.index)
            .field("value", &self.value)
            .finish()
    }
}

impl<'a> std::fmt::Debug for LexToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("type", &self.kind)
            .field("index", &self.index)
            .field("value", &self.value)
            .finish()
    }
}

fn lexer<'a>(input: &'a str) -> Result<Vec<LexToken<'a>>> {
    use LexTokenKind::*;

    let mut tokens = vec![];
    let mut i = 0;
    let char_vec: Vec<_> = input.chars().collect();
    while i < input.len() {
        let char = char_vec[i];
        if matches!(char, '*' | '+' | '?') {
            tokens.push(LexToken {
                kind: Modifier,
                index: i,
                value: &input[i..i + 1],
            });
            i += 1;
            continue;
        }

        if char == '\\' {
            tokens.push(LexToken {
                kind: EscapedChar,
                index: i,
                value: &input[i + 1..i + 2],
            });
            i += 2;
            continue;
        }

        if char == '{' {
            tokens.push(LexToken {
                kind: Open,
                index: i,
                value: &input[i..i + 1],
            });
            i += 1;
            continue;
        }

        if char == '}' {
            tokens.push(LexToken {
                kind: Close,
                index: i,
                value: &input[i..i + 1],
            });
            i += 1;
            continue;
        }

        if char == ':' {
            let mut j = i + 1;
            while j < input.len() {
                let char = char_vec[j];
                if matches!(char,'0'..='9' | 'A'..='Z' | 'a'..='z' |'_') {
                    j += 1;
                    continue;
                }
                break;
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
            continue;
        }

        if char == '(' {
            let mut count = 1;
            let mut pattern = "";
            let mut j = i + 1;

            if char_vec[j] == '?' {
                return Err(anyhow!("Pattern cannot start with \"?\" at {j}"));
            }

            while j < input.len() {
                let char = char_vec[j];

                if char == '\\' {
                    j += 2;
                    continue;
                }

                if char == ')' {
                    count -= 1;
                    if count == 0 {
                        j += 1;
                        break;
                    }
                } else if char == '(' {
                    count += 1;
                    if char_vec[j + 1] != '?' {
                        return Err(anyhow!("Capturing groups are not allowed at {j}"));
                    }
                }
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
            continue;
        }

        tokens.push(LexToken {
            kind: Char,
            index: i,
            value: &input[i..i + 1],
        });
        i += 1;
    }

    tokens.push(LexToken {
        kind: End,
        index: i,
        value: "",
    });

    Ok(tokens)
}

/// Token
pub enum Token {
    ///
    Static(String),
    ///
    Key {
        ///
        name: String,
        ///
        prefix: String,
        ///
        suffix: String,
        ///
        pattern: String,
        ///
        modifier: String,
    },
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Static(s) => f.write_str(s),
            Token::Key {
                name,
                prefix,
                suffix,
                pattern,
                modifier,
            } => f
                .debug_struct("")
                .field("name", name)
                .field("prefix", prefix)
                .field("suffix", suffix)
                .field("pattern", pattern)
                .field("modifier", modifier)
                .finish(),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Static(s) => f.write_str(s),
            Token::Key {
                name,
                prefix,
                suffix,
                pattern,
                modifier,
            } => f
                .debug_struct("")
                .field("name", name)
                .field("prefix", prefix)
                .field("suffix", suffix)
                .field("pattern", pattern)
                .field("modifier", modifier)
                .finish(),
        }
    }
}

/// Path parser
pub struct Parser<'a> {
    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    delimiter: &'a str,
    /// List of characters to automatically consider prefixes when parsing.
    prefixes: &'a str,
}

impl<'a> Parser<'a> {
    /// Create a path Parser
    pub fn new() -> Self {
        Self {
            delimiter: "/#?",
            prefixes: "./",
        }
    }

    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub fn delimiter(self, delimiter: &'a str) -> Self {
        Self { delimiter, ..self }
    }

    /// List of characters to automatically consider prefixes when parsing.
    pub fn prefixes(self, prefixes: &'a str) -> Self {
        Self { prefixes, ..self }
    }

    /// Parse the path to the lexical
    pub fn parse<'b, I>(&self, input: I) -> Result<Vec<Token>>
    where
        I: AsRef<str> + 'b,
    {
        use LexTokenKind::*;
        let input = input.as_ref();
        let tokens = lexer(input)?;
        let mut result = vec![];
        let default_pattern = format!(
            "[^{}]+?",
            escape_string(if self.delimiter.is_empty() {
                "/#?"
            } else {
                self.delimiter
            })?
        );
        let prefixes = self.prefixes;
        let mut key: usize = 0;
        let i: Cell<usize> = Cell::new(0);
        let mut path = String::new();

        let try_consume = |ty: LexTokenKind| {
            if i.get() < tokens.len() && tokens[i.get()].kind == ty {
                let value = tokens[i.get()].value;
                i.set(i.get() + 1);
                return Some(value);
            }
            None
        };

        let must_consume = |ty: LexTokenKind| {
            let value = try_consume(ty);
            if value.is_some() {
                return Ok(value);
            }
            let LexToken { kind, index, .. } = &tokens[i.get()];
            Err(anyhow!("Unexpected {kind} at {index}, expected {ty}"))
        };

        let consume_text = || {
            let mut result = String::new();
            while let Some(t) = try_consume(Char).or(try_consume(EscapedChar)) {
                result += t;
            }
            result
        };

        while i.get() < tokens.len() {
            let char = try_consume(Char);
            let name = try_consume(Name);
            let pattern = try_consume(Pattern);

            if name.or(pattern).is_some() {
                let mut prefix = char.unwrap_or("");

                if prefixes.find(prefix).is_none() {
                    path += prefix;
                    prefix = ""
                }

                if !path.is_empty() {
                    result.push(Token::Static(path));
                    path = String::new();
                }

                result.push(Token::Key {
                    name: name.map_or_else(
                        || {
                            let k = key;
                            key += 1;
                            format!("{k}")
                        },
                        |x| x.to_owned(),
                    ),
                    prefix: prefix.to_owned(),
                    suffix: String::new(),
                    pattern: pattern.map_or_else(|| default_pattern.clone(), |x| x.to_owned()),
                    modifier: try_consume(Modifier).unwrap_or("").to_owned(),
                });
                continue;
            }

            let value = char.or(try_consume(EscapedChar));
            if let Some(value) = value {
                path += value;
                continue;
            }

            if !path.is_empty() {
                result.push(Token::Static(path));
                path = String::new();
            }

            let open = try_consume(Open);
            if open.is_some() {
                let prefix = consume_text();
                let name = try_consume(Name);
                let pattern = try_consume(Pattern);
                let suffix = consume_text();

                must_consume(Close)?;

                result.push(Token::Key {
                    name: name.map_or_else(
                        || {
                            if pattern.is_some() {
                                let k = key;
                                key += 1;
                                format!("{k}")
                            } else {
                                "".to_owned()
                            }
                        },
                        |x| x.to_owned(),
                    ),
                    pattern: if name.is_some() && pattern.is_none() {
                        default_pattern.clone()
                    } else {
                        pattern.unwrap_or("").to_owned()
                    },
                    prefix,
                    suffix,
                    modifier: try_consume(Modifier).unwrap_or("").to_owned(),
                });

                continue;
            }

            must_consume(End)?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let arr = lexer("/\\segment+").unwrap();
        assert_eq!(format!("{:?}",arr),"[ { type: CHAR, index: 0, value: \"/\" },  { type: ESCAPEDCHAR, index: 1, value: \"s\" },  { type: CHAR, index: 3, value: \"e\" },  { type: CHAR, index: 4, value: \"g\" },  { type: CHAR, index: 5, value: \"m\" },  { type: CHAR, index: 6, value: \"e\" },  { type: CHAR, index: 7, value: \"n\" },  { type: CHAR, index: 8, value: \"t\" },  { type: MODIFIER, index: 9, value: \"+\" },  { type: END, index: 10, value: \"\" }]");
    }

    #[test]
    fn test_parse() {
        let p = Parser::new();
        // println!("{:#?}", lexer("{:test/}+").unwrap());
        println!("{:#?}", p.parse("packages/").unwrap());
        assert!(true);
    }
}
