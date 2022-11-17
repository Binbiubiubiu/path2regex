//! matcher
use anyhow::Result;

use crate::{
    internal::{type_of, FnStr, FnStrWithKey},
    try_into_with::TryIntoWith,
    Matcher, PathRegex, PathRegexOptions,
};

use super::Builder;

///
#[derive(Clone)]
pub struct MatcherOptions {
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
    /// Function for decoding strings for params.
    pub decode: FnStrWithKey,
}

impl Default for MatcherOptions {
    fn default() -> Self {
        let PathRegexOptions {
            delimiter,
            prefixes,
            sensitive,
            strict,
            end,
            start,
            ends_with,
            encode,
        } = PathRegexOptions::default();
        Self {
            delimiter,
            prefixes,
            sensitive,
            strict,
            end,
            start,
            ends_with,
            encode,
            decode: |x, _| x.to_owned(),
        }
    }
}

impl std::fmt::Display for MatcherOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::fmt::Debug for MatcherOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatcherOptions")
            .field("delimiter", &self.delimiter)
            .field("prefixes", &self.prefixes)
            .field("sensitive", &self.sensitive)
            .field("strict", &self.strict)
            .field("end", &self.end)
            .field("start", &self.start)
            .field("ends_with", &self.ends_with)
            .field("encode", &type_of(self.encode))
            .field("decode", &type_of(self.decode))
            .finish()
    }
}

///
pub struct MatcherBuilder<I> {
    source: I,
    options: MatcherOptions,
}

impl<I> MatcherBuilder<I>
where
    I: TryIntoWith<PathRegex, PathRegexOptions>,
{
    ///
    pub fn new(source: I) -> Self {
        Self {
            source,
            options: Default::default(),
        }
    }
}

impl<I> Builder<Result<Matcher>> for MatcherBuilder<I>
where
    I: TryIntoWith<PathRegex, PathRegexOptions>,
{
    fn build(self) -> Result<Matcher> {
        let re = self
            .source
            .try_into_with(&PathRegexOptions::from(self.options.clone()))?;

        Ok(Matcher {
            re: re.clone(),
            keys: re.keys,
            options: self.options,
        })
    }
}
