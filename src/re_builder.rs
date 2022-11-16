use eyre::Result;

use crate::{
    matcher::MatcherOptions, parser::ParserOptions, try_pipe::TryPipe, type_of, PathRegex,
};

///
pub(crate) type EncodeFn = for<'a> fn(&'a String) -> String;
///
#[derive(Clone)]
pub struct PathRegexOptions {
    /// Set the default delimiter for repeat parameters. (default: `'/#?'`)
    pub(crate) delimiter: String,
    /// List of characters to automatically consider prefixes when parsing.
    pub(crate) prefixes: String,
    /// When `true` the regexp will be case sensitive. (default: `false`)
    pub(crate) sensitive: bool,
    /// When `true` the regexp won't allow an optional trailing delimiter to match. (default: `false`)
    pub(crate) strict: bool,
    /// When `true` the regexp will match to the end of the string. (default: `true`)
    pub(crate) end: bool,
    /// When `true` the regexp will match from the beginning of the string. (default: `true`)
    pub(crate) start: bool,
    /// List of characters that can also be "end" characters.
    pub(crate) ends_with: String,
    /// Encode path tokens for use in the `Regex`.
    pub(crate) encode: EncodeFn,
}

impl PathRegexOptions {
    ///
    pub fn new() -> Self {
        Default::default()
    }
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

impl From<MatcherOptions> for PathRegexOptions {
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

///
pub struct PathRegexBuilder<T> {
    source: T,
    options: PathRegexOptions,
}

impl<T> PathRegexBuilder<T>
where
    T: TryPipe<PathRegex, PathRegexOptions>,
{
    ///
    pub fn new(source: T) -> Self {
        Self {
            source,
            options: Default::default(),
        }
    }

    ///
    pub fn build(&self) -> Result<PathRegex> {
        self.source.try_pipe(&self.options)
    }

    ///
    pub fn replace_options(&mut self, options: PathRegexOptions) -> &mut Self {
        self.options = options;
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
    pub fn strict(&mut self, yes: bool) -> &mut Self {
        self.options.strict = yes;
        self
    }

    ///
    pub fn end(&mut self, yes: bool) -> &mut Self {
        self.options.end = yes;
        self
    }

    ///
    pub fn start(&mut self, yes: bool) -> &mut Self {
        self.options.start = yes;
        self
    }

    ///
    pub fn delimiter<S>(&mut self, de: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.options.delimiter = de.as_ref().to_owned();
        self
    }

    ///
    pub fn ends_with<S>(&mut self, end: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.options.ends_with = end.as_ref().to_owned();
        self
    }

    ///
    pub fn encode(&mut self, encode: EncodeFn) -> &mut Self {
        self.options.encode = encode;
        self
    }
}
