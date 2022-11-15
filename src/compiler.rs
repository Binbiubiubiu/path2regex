//! 2323
use std::collections::HashMap;

use eyre::{eyre, Result};
use regex::Regex;

use crate::{ParserBuilder, Key, Token};

type EncodeFn = for<'a> fn(&'a String, &'a Key) -> String;

///
#[derive(Clone)]
pub struct Compiler {
    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    delimiter: String,
    /// List of characters to automatically consider prefixes when parsing.
    prefixes: String,

    /// When `true` the regexp will be case sensitive. (default: `false`)
    sensitive: bool,
    // Function for encoding input strings for output.
    encode: EncodeFn,
    // When `false` the function can produce an invalid (unmatched) path. (default: `true`)
    validate: bool,
}

impl Compiler {
    ///
    pub fn new() -> Self {
        Self {
            delimiter: String::from("/#?"),
            prefixes: String::from("./"),
            sensitive: false,
            encode: |x, _| x.to_owned(),
            validate: true,
        }
    }
}

impl Compiler {
    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub fn delimiter<S>(self, delimiter: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            delimiter: delimiter.as_ref().to_owned(),
            ..self
        }
    }

    /// List of characters to automatically consider prefixes when parsing.
    pub fn prefixes<S>(self, prefixes: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            prefixes: prefixes.as_ref().to_owned(),
            ..self
        }
    }

    ///
    pub fn sensitive(self, sensitive: bool) -> Self {
        Self { sensitive, ..self }
    }

    ///
    pub fn encode(self, encode: EncodeFn) -> Self {
        Self { encode, ..self }
    }

    ///
    pub fn validate(self, validate: bool) -> Self {
        Self { validate, ..self }
    }
}

impl Compiler {
    ///
    pub fn build<'b, I>(&self, input: I) -> Result<Matcher>
    where
        I: AsRef<str> + 'b,
    {
        let tokens = ParserBuilder::new()
            .delimiter(&self.delimiter)
            .prefixes(&self.prefixes)
            .build()
            .parse(input)?;
        let matches = tokens
            .iter()
            .map(|token| match token {
                Token::Static(_) => None,
                Token::Key(Key { pattern, .. }) => {
                    let pattern = &format!("^(?:{pattern})$");
                    let re = regex::RegexBuilder::new(pattern)
                        .case_insensitive(self.sensitive)
                        .build();
                    re.ok()
                }
            })
            .collect::<Vec<_>>();
        Ok(Matcher {
            tokens,
            matches,
            complier: self.clone(),
        })
    }
}

///
pub struct Matcher {
    tokens: Vec<Token>,
    matches: Vec<Option<Regex>>,
    complier: Compiler,
}

impl Matcher {
    ///
    pub fn complie(&self, data: HashMap<String, DataValue>) -> Result<String> {
        let mut path = String::new();
        let Compiler {
            validate, encode, ..
        } = self.complier;

        for (i, token) in self.tokens.iter().enumerate() {
            match token {
                Token::Static(token) => {
                    path += token;
                    continue;
                }
                Token::Key(token) => {
                    let value = data.get(&token.name);
                    let modifier = token.modifier.as_str();
                    let optional = matches!(modifier, "?" | "*");
                    let repeat = matches!(modifier, "+" | "*");

                    if let Some(value) = value {
                        match value {
                            DataValue::Array(value) => {
                                if !repeat {
                                    return Err(eyre!(
                                        "Expected \"{}\" to not repeat, but got an array",
                                        token.name
                                    ));
                                }

                                if value.is_empty() {
                                    if optional {
                                        continue;
                                    }

                                    return Err(eyre!(
                                        "Expected \"{}\" to not be empty",
                                        token.name
                                    ));
                                }

                                for value in value.iter() {
                                    let segment = encode(value, token);

                                    if validate
                                        && !self.matches[i]
                                            .as_ref()
                                            .map(|m| m.is_match(segment.as_str()))
                                            .is_some()
                                    {
                                        return Err(eyre!("Expected all \"{}\" to match \"{}\", but got \"{segment}\"",token.name,token.pattern));
                                    }
                                    path =
                                        format!("{path}{}{segment}{}", token.prefix, token.suffix);
                                }
                                continue;
                            }
                            DataValue::String(value) => {
                                let segment = encode(value, token);

                                if validate
                                    && !self.matches[i]
                                        .as_ref()
                                        .map(|m| m.is_match(segment.as_str()))
                                        .is_some()
                                {
                                    return Err(eyre!("Expected all \"{}\" to match \"{}\", but got \"{segment}\"",token.name,token.pattern));
                                }

                                path = format!("{path}{}{segment}{}", token.prefix, token.suffix);

                                continue;
                            }
                        }
                    }

                    if optional {
                        continue;
                    }

                    let type_of_message = if repeat { "an array" } else { "a string" };
                    return Err(eyre!(
                        "Expected \"{}\" to be ${type_of_message}",
                        token.name
                    ));
                }
            }
        }
        Ok(path)
    }
}

///  
pub enum DataValue {
    ///
    Array(Vec<String>),
    ///
    String(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complie() {
        let mut data = HashMap::new();
        data.insert("test".to_owned(), DataValue::String("a+b".to_string()));
        let re = Compiler::new()
            .encode(|s, _| urlencoding::encode(s).to_string())
            .build("/:test")
            .unwrap()
            .complie(data)
            .unwrap();
        println!("{}", re);
        assert!(true);
    }
}
