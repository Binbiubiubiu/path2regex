//! 2323

use eyre::{eyre, Result};
use regex::Regex;

use crate::{parser::ParserOptions, try_pipe::TryPipe, type_of, Key, Token, Value};

type EncodeFn = for<'a> fn(&'a String, &'a Key) -> String;

/// The Option of the Parser
#[derive(Clone)]
pub(crate) struct CompilerOptions {
    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub(crate) delimiter: String,
    /// List of characters to automatically consider prefixes when parsing.
    pub(crate) prefixes: String,

    /// When `true` the regexp will be case sensitive. (default: `false`)
    pub(crate) sensitive: bool,
    // Function for encoding input strings for output.
    pub(crate) encode: EncodeFn,
    // When `false` the function can produce an invalid (unmatched) path. (default: `true`)
    pub(crate) validate: bool,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        let po = ParserOptions::default();
        Self {
            delimiter: po.delimiter,
            prefixes: po.prefixes,
            sensitive: false,
            encode: |x, _| x.to_owned(),
            validate: true,
        }
    }
}

impl std::fmt::Display for CompilerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::fmt::Debug for CompilerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompilerOptions")
            .field("delimiter", &self.delimiter)
            .field("prefixes", &self.prefixes)
            .field("sensitive", &self.sensitive)
            .field("encode", &type_of(self.encode))
            .field("validate", &self.validate)
            .finish()
    }
}

///
#[derive(Clone)]
pub struct CompilerBuilder<I> {
    source: I,
    options: CompilerOptions,
}

impl<I> CompilerBuilder<I>
where
    I: TryPipe<Vec<Token>, ParserOptions>,
{
    ///
    pub fn new(source: I) -> Self {
        Self {
            source,
            options: Default::default(),
        }
    }

    /// Finish to build a Compiler
    pub fn build(&self) -> Result<Compiler> {
        let tokens = self
            .source
            .try_pipe(&ParserOptions::from(self.options.clone()))?;
        let matches = tokens
            .iter()
            .map(|token| match token {
                Token::Static(_) => None,
                Token::Key(Key { pattern, .. }) => {
                    let pattern = &format!("^(?:{pattern})$");
                    let re = regex::RegexBuilder::new(pattern)
                        .case_insensitive(self.options.sensitive)
                        .build();
                    re.ok()
                }
            })
            .collect::<Vec<_>>();
        Ok(Compiler {
            tokens,
            matches,
            options: self.options.clone(),
        })
    }

    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub fn delimiter<S>(&mut self, delimiter: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.options.delimiter = delimiter.as_ref().to_owned();
        self
    }

    /// List of characters to automatically consider prefixes when parsing.
    pub fn prefixes<S>(&mut self, prefixes: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.options.prefixes = prefixes.as_ref().to_owned();
        self
    }

    ///
    pub fn sensitive(&mut self, yes: bool) -> &mut Self {
        self.options.sensitive = yes;
        self
    }

    ///
    pub fn encode(&mut self, encode: EncodeFn) -> &mut Self {
        self.options.encode = encode;
        self
    }

    ///
    pub fn validate(&mut self, validate: bool) -> &mut Self {
        self.options.validate = validate;
        self
    }
}

///
pub struct Compiler {
    tokens: Vec<Token>,
    matches: Vec<Option<Regex>>,
    options: CompilerOptions,
}

impl Compiler {
    ///
    pub fn new<I>(path: I) -> Result<Compiler>
    where
        I: TryPipe<Vec<Token>, ParserOptions>,
    {
        CompilerBuilder::new(path).build()
    }

    ///
    pub fn from(tokens: Vec<Token>) -> Result<Compiler> {
        CompilerBuilder::new(tokens).build()
    }

    ///
    pub fn render(&self, data: &Value) -> Result<String> {
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
                            return Err(eyre!("Expected all \"{name}\" to match \"{pattern}\", but got \"{segment}\""));
                        }
                        path = format!("{path}{prefix}{segment}{suffix}");
                        Ok(())
                    };

                    if let Some(value) = value {
                        match value {
                            Value::Array(value) => {
                                if !repeat {
                                    return Err(eyre!(
                                        "Expected \"{name}\" to not repeat, but got an array",
                                    ));
                                }

                                if value.is_empty() {
                                    if optional {
                                        continue;
                                    }

                                    return Err(eyre!("Expected \"{name}\" to not be empty",));
                                }

                                for value in value.iter() {
                                    match value {
                                        Value::Number(value) => {
                                            resolve_string(&value.to_string())?;
                                        }
                                        Value::String(value) => {
                                            resolve_string(value)?;
                                        }
                                        _ => {
                                            return Err(eyre!(
                                                "Expected \"{name}\" to be {array_type_name}"
                                            ))
                                        }
                                    }
                                }
                                continue;
                            }
                            Value::Number(value) => {
                                resolve_string(&value.to_string())?;
                                continue;
                            }
                            Value::String(value) => {
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
                    return Err(eyre!("Expected \"{name}\" to be {type_of_message}"));
                }
            }
        }
        Ok(path)
    }
}
