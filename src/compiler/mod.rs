//! Path compiler
mod builder;

use anyhow::{anyhow, Result};
pub use builder::{CompilerBuilder, CompilerOptions};
use regex::Regex;

use crate::{internal::DataValue, try_into_with::TryIntoWith, Key, ParserOptions, Token};

/// Path compiler
pub struct Compiler {
    pub(crate) tokens: Vec<Token>,
    pub(crate) matches: Vec<Option<Regex>>,
    pub(crate) options: CompilerOptions,
}

impl Compiler {
    ///
    #[inline]
    pub fn new<I>(path: I) -> Result<Compiler>
    where
        I: TryIntoWith<Vec<Token>, ParserOptions>,
    {
        CompilerBuilder::new(path).build()
    }

    ///
    pub fn render(&self, data: &DataValue) -> Result<String> {
        let mut path = String::new();
        let CompilerOptions {
            validate, encode, ..
        } = self.options;

        let array_type_name = "an array containing only strings or numbers";
        let item_type_name = "a string or a number";

        for (i, token) in self.tokens.iter().enumerate() {
            match token {
                Token::Static(token) => {
                    path += token;
                    continue;
                }
                Token::Key(token) => {
                    let Key {
                        name,
                        prefix,
                        suffix,
                        pattern,
                        modifier,
                    } = token;
                    let value = data.get(name);
                    let modifier = modifier.as_str();
                    let optional = matches!(modifier, "?" | "*");
                    let repeat = matches!(modifier, "+" | "*");

                    let mut resolve_string = |value: &String| {
                        let segment = encode(value, token);

                        if validate
                            && self.matches[i]
                                .as_ref()
                                .map(|m| m.is_match(segment.as_str()))
                                .is_none()
                        {
                            return Err(anyhow!("Expected all \"{name}\" to match \"{pattern}\", but got \"{segment}\""));
                        }
                        path = format!("{path}{prefix}{segment}{suffix}");
                        Ok(())
                    };

                    if let Some(value) = value {
                        match value {
                            DataValue::Array(value) => {
                                if !repeat {
                                    return Err(anyhow!(
                                        "Expected \"{name}\" to not repeat, but got an array",
                                    ));
                                }

                                if value.is_empty() {
                                    if optional {
                                        continue;
                                    }

                                    return Err(anyhow!("Expected \"{name}\" to not be empty",));
                                }

                                for value in value.iter() {
                                    match value {
                                        DataValue::Number(value) => {
                                            resolve_string(&value.to_string())?;
                                        }
                                        DataValue::String(value) => {
                                            resolve_string(value)?;
                                        }
                                        _ => {
                                            return Err(anyhow!(
                                                "Expected \"{name}\" to be {array_type_name}"
                                            ))
                                        }
                                    }
                                }
                                continue;
                            }
                            DataValue::Number(value) => {
                                resolve_string(&value.to_string())?;
                                continue;
                            }
                            DataValue::String(value) => {
                                resolve_string(value)?;
                                continue;
                            }
                            _ => (),
                        }
                    }

                    if optional {
                        continue;
                    }

                    let type_of_message = if repeat {
                        array_type_name
                    } else {
                        item_type_name
                    };
                    return Err(anyhow!("Expected \"{name}\" to be {type_of_message}"));
                }
            }
        }
        Ok(path)
    }
}
