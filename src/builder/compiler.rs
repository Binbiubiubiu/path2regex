//!
use crate::{
    internal::{type_of, FnStrWithKey},
    try_into_with::TryIntoWith,
    Compiler, Key, ParserOptions, Token,
};
use anyhow::Result;

use super::Builder;

/// The Option of the Parser
#[derive(Clone)]
pub struct CompilerOptions {
    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub delimiter: String,
    /// List of characters to automatically consider prefixes when parsing.
    pub prefixes: String,

    /// When `true` the regexp will be case sensitive. (default: `false`)
    pub sensitive: bool,
    /// Function for encoding input strings for output.
    pub encode: FnStrWithKey,
    /// When `false` the function can produce an invalid (unmatched) path. (default: `true`)
    pub validate: bool,
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

impl<I> Builder<Result<Compiler>> for CompilerBuilder<I>
where
    I: TryIntoWith<Vec<Token>, ParserOptions>,
{
    /// Finish to build a Compiler
    fn build(self) -> Result<Compiler> {
        let tokens = self
            .source
            .try_into_with(&ParserOptions::from(self.options.clone()))?;
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
            options: self.options,
        })
    }
}

impl<I> CompilerBuilder<I>
where
    I: TryIntoWith<Vec<Token>, ParserOptions>,
{
    ///
    pub fn new(source: I) -> Self {
        Self {
            source,
            options: Default::default(),
        }
    }

    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub fn set_delimiter<S>(&mut self, delimiter: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.options.delimiter = delimiter.as_ref().to_owned();
        self
    }

    /// List of characters to automatically consider prefixes when parsing.
    pub fn set_prefixes<S>(&mut self, prefixes: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.options.prefixes = prefixes.as_ref().to_owned();
        self
    }

    ///
    pub fn set_sensitive(&mut self, yes: bool) -> &mut Self {
        self.options.sensitive = yes;
        self
    }

    ///
    pub fn set_encode(&mut self, encode: FnStrWithKey) -> &mut Self {
        self.options.encode = encode;
        self
    }

    ///
    pub fn set_validate(&mut self, validate: bool) -> &mut Self {
        self.options.validate = validate;
        self
    }
}
