//! The Builder of the [`Parser`](struct.Parser.html)

#[cfg(feature = "compile")]
use crate::CompilerOptions;
use crate::{Parser, PathRegexOptions, DEFUALT_DELIMITER};

/// The Configuration of the [`Parser`](struct.Parser.html)
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
            delimiter: DEFUALT_DELIMITER.to_owned(),
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
    #[inline]
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
    #[inline]
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

/// The Builder of the [`Parser`](struct.Parser.html)
#[derive(Debug, Clone)]
pub struct ParserBuilder(ParserOptions);

impl ParserBuilder {
    /// Create a [`Parser`](struct.Parser.html) Builder
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Finish to build a [`Parser`](struct.Parser.html)
    pub fn build(&self) -> Parser {
        Parser(self.0.clone())
    }

    /// Set the default delimiter for repeat parameters. (default: `'/'`)
    pub fn set_delimiter<S>(&mut self, delimiter: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.0.delimiter = delimiter.as_ref().to_owned();
        self
    }

    /// List of characters to automatically consider prefixes when parsing.
    pub fn set_prefixes<S>(&mut self, prefixes: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.0.prefixes = prefixes.as_ref().to_owned();
        self
    }
}

impl Default for ParserBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
