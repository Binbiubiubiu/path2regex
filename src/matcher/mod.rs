//! Path matcher
mod builder;

use anyhow::Result;

use crate::{
    internal::{DataValue, END_WITH_DELIMITER},
    Key, PathRegex, PathRegexOptions, TryIntoWith,
};

pub use builder::{MatcherBuilder, MatcherOptions};

/// Path matcher
#[derive(Debug)]
pub struct Matcher {
    pub(crate) re: PathRegex,
    pub(crate) keys: Vec<Key>,
    pub(crate) options: MatcherOptions,
}

impl Matcher {
    /// Create a [`Matcher`](struct.Matcher.html)
    #[inline]
    pub fn new<S>(path: S) -> Result<Self>
    where
        S: TryIntoWith<PathRegex, PathRegexOptions>,
    {
        MatcherBuilder::new(path).build()
    }

    /// Create a [`Matcher`](struct.Matcher.html) with the options
    #[inline]
    pub fn new_with_options<S>(path: S, options: MatcherOptions) -> Result<Self>
    where
        S: TryIntoWith<PathRegex, PathRegexOptions>,
    {
        MatcherBuilder::new_with_options(path, options).build()
    }

    /// matching parameters in the path
    pub fn find<S>(&self, path: S) -> Option<MatchResult>
    where
        S: AsRef<str>,
    {
        let path = path.as_ref();
        let MatcherOptions { decode, .. } = &self.options;

        let captures = self.re.captures(path)?;
        let m = captures.get(0)?;

        let params = captures
            .iter()
            .skip(1)
            .map(|x| x.map_or("", |x| x.as_str()))
            .zip(self.keys.iter())
            .map(|(value, key)| {
                let Key {
                    name,
                    prefix,
                    suffix,
                    ..
                } = key;

                if matches!(name.as_str(), "*" | "+") {
                    let sp = if prefix.is_empty() { suffix } else { prefix };
                    let value = value
                        .split(sp)
                        .map(|x| DataValue::String(decode(x, key)))
                        .collect();
                    return (name.to_owned(), DataValue::Array(value));
                }

                (name.to_owned(), DataValue::String(decode(value, key)))
            })
            .collect::<DataValue>();

        let mut path = m.as_str();
        if captures.name(END_WITH_DELIMITER).is_some() {
            path = &path[..path.len() - 1];
        }

        Some(MatchResult {
            index: m.start(),
            path: path.to_owned(),
            params,
        })
    }
}

/// Regular matching results
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MatchResult {
    /// The path of the match
    pub path: String,
    /// The index of the match
    pub index: usize,
    /// Matching parameters
    pub params: DataValue,
}

// impl MatchResult {
//     pub fn path(&self) -> &String {
//         &self.path
//     }

//     pub fn index(&self) -> usize {
//         self.index
//     }

//     pub fn params(&self) -> &ParamsType {
//         &self.params
//     }
// }
