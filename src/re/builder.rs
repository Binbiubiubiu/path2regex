//! The Builder of the [`PathRegex`](struct.PathRegex.html)

use anyhow::Result;

use crate::{
    internal::{type_of, FnStr},
    ParserOptions, PathRegex, TryIntoWith,
};

#[cfg(feature = "match")]
use crate::MatcherOptions;

/// The Configuration of the [`PathRegex`](struct.PathRegex.html)
#[derive(Clone)]
pub struct PathRegexOptions {
    /// Set the default delimiter for repeat parameters. (default: `'/#?'`)
    pub delimiter: String,
    /// List of characters to automatically consider prefixes when parsing.
    pub prefixes: String,
    /// When `true` the regexp will be case sensitive. (default: `false`)
    pub sensitive: bool,
    /// When `true` the regexp won't allow an optional trailing delimiter to match. (default: `false`)
    pub strict: bool,
    /// When `true` the regexp will match to the end of the string. (default: `true`)
    pub end: bool,
    /// When `true` the regexp will match from the beginning of the string. (default: `true`)
    pub start: bool,
    /// List of characters that can also be "end" characters.
    pub ends_with: String,
    /// Encode path tokens for use in the `Regex`.
    pub encode: FnStr,
}

impl Default for PathRegexOptions {
    fn default() -> Self {
        let ParserOptions {
            delimiter,
            prefixes,
        } = ParserOptions::default();
        Self {
            delimiter,
            prefixes,
            sensitive: false,
            strict: false,
            end: true,
            start: true,
            ends_with: "".to_owned(),
            encode: |x| x.to_owned(),
        }
    }
}

#[cfg(feature = "match")]
impl From<MatcherOptions> for PathRegexOptions {
    #[inline]
    fn from(options: MatcherOptions) -> Self {
        let MatcherOptions {
            delimiter,
            prefixes,
            sensitive,
            strict,
            end,
            start,
            ends_with,
            encode,
            ..
        } = options;
        Self {
            delimiter,
            prefixes,
            sensitive,
            strict,
            end,
            start,
            ends_with,
            encode,
        }
    }
}

impl std::fmt::Display for PathRegexOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::fmt::Debug for PathRegexOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathRegexOptions")
            .field("delimiter", &self.delimiter)
            .field("prefixes", &self.prefixes)
            .field("sensitive", &self.sensitive)
            .field("strict", &self.strict)
            .field("end", &self.end)
            .field("start", &self.start)
            .field("ends_with", &self.ends_with)
            .field("encode", &type_of(self.encode))
            .finish()
    }
}

/// The Builder of the [`PathRegex`](struct.PathRegex.html)
pub struct PathRegexBuilder<S> {
    source: S,
    options: PathRegexOptions,
}

impl<S> PathRegexBuilder<S>
where
    S: TryIntoWith<PathRegex, PathRegexOptions>,
{
    /// Create a [`PathRegex`](struct.PathRegex.html) Builder
    pub fn new(source: S) -> Self {
        Self {
            source,
            options: Default::default(),
        }
    }

    /// Create a builder of the [`PathRegex`](struct.PathRegex.html) with the options
    pub fn new_with_options(source: S, options: PathRegexOptions) -> Self {
        Self { source, options }
    }

    /// build a builder of the [`PathRegex`](struct.PathRegex.html)
    pub fn build(&self) -> Result<PathRegex> {
        self.source.clone().try_into_with(&self.options)
    }

    /// List of characters to automatically consider prefixes when parsing.
    pub fn set_prefixes(&mut self, prefixes: impl AsRef<str>) -> &mut Self {
        self.options.prefixes = prefixes.as_ref().to_owned();
        self
    }

    /// When `true` the regexp will be case sensitive. (default: `false`)
    pub fn set_sensitive(&mut self, yes: bool) -> &mut Self {
        self.options.sensitive = yes;
        self
    }

    /// When `true` the regexp won't allow an optional trailing delimiter to match. (default: `false`)
    pub fn set_strict(&mut self, yes: bool) -> &mut Self {
        self.options.strict = yes;
        self
    }

    /// When `true` the regexp will match to the end of the string. (default: `true`)
    pub fn set_end(&mut self, yes: bool) -> &mut Self {
        self.options.end = yes;
        self
    }

    /// When `true` the regexp will match from the beginning of the string. (default: `true`)
    pub fn set_start(&mut self, yes: bool) -> &mut Self {
        self.options.start = yes;
        self
    }

    /// Set the default delimiter for repeat parameters. (default: `'/#?'`)
    pub fn set_delimiter(&mut self, de: impl AsRef<str>) -> &mut Self {
        self.options.delimiter = de.as_ref().to_owned();
        self
    }

    /// List of characters that can also be "end" characters.
    pub fn set_ends_with(&mut self, end: impl AsRef<str>) -> &mut Self {
        self.options.ends_with = end.as_ref().to_owned();
        self
    }

    /// Function for encoding input strings for output.
    pub fn set_encode(&mut self, encode: FnStr) -> &mut Self {
        self.options.encode = encode;
        self
    }
}
