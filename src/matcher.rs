//! 122

use eyre::Result;
use std::collections::HashMap;

use crate::{
    re_builder::{EncodeFn, PathRegexOptions},
    try_pipe::TryPipe,
    type_of, Key, PathRegex, Value,
};
type DecodeFn = for<'a> fn(&'a str, &'a Key) -> String;
type ParamsType = HashMap<String, Value>;

///
#[derive(Clone)]
pub(crate) struct MatcherOptions {
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
    /// Function for decoding strings for params.
    pub(crate) decode: DecodeFn,
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
    I: TryPipe<PathRegex, PathRegexOptions>,
{
    ///
    pub fn new(source: I) -> Self {
        Self {
            source,
            options: Default::default(),
        }
    }

    ///
    pub fn build(&self) -> Result<Matcher> {
        let re = self
            .source
            .try_pipe(&PathRegexOptions::from(self.options.clone()))?;

        Ok(Matcher {
            re: re.clone(),
            keys: re.keys,
            options: self.options.clone(),
        })
    }
}

///
#[derive(Debug)]
pub struct Matcher {
    re: PathRegex,
    keys: Vec<Key>,
    options: MatcherOptions,
}

impl Matcher {
    ///
    pub fn new<S>(path: S) -> Result<Self>
    where
        S: TryPipe<PathRegex, PathRegexOptions>,
    {
        MatcherBuilder::new(path).build()
    }

    ///
    pub fn find<S>(&self, path: S) -> Option<ParamsType>
    where
        S: AsRef<str>,
    {
        let path = path.as_ref();
        let MatcherOptions { decode, .. } = &self.options;

        self.re.find(path)?;

        let mut params = HashMap::new();

        for (i, m) in self.re.find_iter(path).enumerate() {
            let m = m.as_str();
            let key = &self.keys[i - 1];
            let Key {
                name,
                prefix,
                suffix,
                ..
            } = key;
            if matches!(name.as_str(), "*" | "+") {
                let sp = if prefix.is_empty() { suffix } else { prefix };
                let value: Vec<_> = m.split(sp).map(|x| Value::String(decode(x, key))).collect();
                params.insert(name.to_owned(), Value::Array(value));
            } else {
                params.insert(name.to_owned(), Value::String(decode(m, key)));
            }
        }

        Some(params)
    }
}
