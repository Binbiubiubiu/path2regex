//! The Option of the Parser

#[cfg(feature = "compile")]
use crate::CompilerOptions;
use crate::{Parser, PathRegexOptions};

use super::Builder;

///
#[derive(Clone)]
pub struct ParserOptions {
    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub delimiter: String,
    /// List of characters to automatically consider prefixes when parsing.
    pub prefixes: String,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            delimiter: "/#?".to_owned(),
            prefixes: "./".to_owned(),
        }
    }
}

impl std::fmt::Debug for ParserOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserOptions")
            .field("delimiter", &self.delimiter)
            .field("prefixes", &self.prefixes)
            .finish()
    }
}

impl std::fmt::Display for ParserOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl From<PathRegexOptions> for ParserOptions {
    fn from(options: PathRegexOptions) -> Self {
        let PathRegexOptions {
            delimiter,
            prefixes,
            ..
        } = options;
        Self {
            delimiter,
            prefixes,
        }
    }
}

#[cfg(feature = "compile")]
impl From<CompilerOptions> for ParserOptions {
    fn from(options: CompilerOptions) -> Self {
        let CompilerOptions {
            delimiter,
            prefixes,
            ..
        } = options;
        Self {
            delimiter,
            prefixes,
        }
    }
}

///
#[derive(Debug, Clone)]
pub struct ParserBuilder(ParserOptions);

impl Builder<Parser> for ParserBuilder {
    /// Finish to build a Parser
    fn build(self) -> Parser {
        Parser(self.0)
    }
}

impl ParserBuilder {
    /// Create a Parser Builder
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub fn delimiter<S>(&mut self, delimiter: S) -> &mut ParserBuilder
    where
        S: AsRef<str>,
    {
        self.0.delimiter = delimiter.as_ref().to_owned();
        self
    }

    /// List of characters to automatically consider prefixes when parsing.
    pub fn prefixes<S>(&mut self, prefixes: S) -> &mut ParserBuilder
    where
        S: AsRef<str>,
    {
        self.0.prefixes = prefixes.as_ref().to_owned();
        self
    }
}

impl Default for ParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}
