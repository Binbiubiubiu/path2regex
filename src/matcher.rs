//! 122

use anyhow::Result;
use std::collections::HashMap;

use crate::{
    builder::Builder, internal::DataValue, try_into_with::TryIntoWith, Key, MatcherBuilder,
    MatcherOptions, PathRegex, PathRegexOptions,
};

type ParamsType = std::collections::HashMap<String, DataValue>;

///
#[derive(Debug)]
pub struct Matcher {
    pub(crate) re: PathRegex,
    pub(crate) keys: Vec<Key>,
    pub(crate) options: MatcherOptions,
}

impl Matcher {
    ///
    pub fn new<S>(path: S) -> Result<Self>
    where
        S: TryIntoWith<PathRegex, PathRegexOptions>,
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
                let value: Vec<_> = m
                    .split(sp)
                    .map(|x| DataValue::String(decode(x, key)))
                    .collect();
                params.insert(name.to_owned(), DataValue::Array(value));
            } else {
                params.insert(name.to_owned(), DataValue::String(decode(m, key)));
            }
        }

        Some(params)
    }
}
