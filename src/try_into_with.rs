//! try

use anyhow::Result;

use crate::{
    parser::parse_str_with_options,
    re::{regex_to_path_regex, string_to_path_regex},
    ParserOptions, PathRegex, PathRegexOptions, Token,
};

///
pub trait TryIntoWith<T, O>: Clone {
    ///
    fn try_into_with(self, options: &O) -> Result<T>;
}

impl TryIntoWith<Vec<Token>, ParserOptions> for Vec<Token> {
    fn try_into_with(self, _: &ParserOptions) -> Result<Vec<Token>> {
        Ok(self)
    }
}

impl TryIntoWith<Vec<Token>, ParserOptions> for String
{
    fn try_into_with(self, options: &ParserOptions) -> Result<Vec<Token>> {
        (&*self).try_into_with(options)
    }
}

impl TryIntoWith<Vec<Token>, ParserOptions> for &str {
    fn try_into_with(self, options: &ParserOptions) -> Result<Vec<Token>> {
        parse_str_with_options(self, options)
    }
}

impl TryIntoWith<PathRegex, PathRegexOptions> for regex::Regex {
    fn try_into_with(self, _: &PathRegexOptions) -> Result<PathRegex> {
        let mut keys = vec![];
        let re = regex_to_path_regex(self, &mut keys)?;
        Ok(PathRegex { re, keys })
    }
}

impl TryIntoWith<PathRegex, PathRegexOptions> for String {
    fn try_into_with(self, options: &PathRegexOptions) -> Result<PathRegex> {
        (&*self).try_into_with(options)
    }
}

impl<'a> TryIntoWith<PathRegex, PathRegexOptions> for &'a str {
    fn try_into_with(self, options: &PathRegexOptions) -> Result<PathRegex> {
        string_to_path_regex(self, options)
    }
}

impl<T> TryIntoWith<PathRegex, PathRegexOptions> for Vec<T>
where
    T: TryIntoWith<PathRegex, PathRegexOptions>,
{
    fn try_into_with(self, options: &PathRegexOptions) -> Result<PathRegex> {
        let mut keys = vec![];
        let mut parts = vec![];
        for source in self.into_iter() {
            let mut re = source.try_into_with(options)?;
            keys.append(&mut re.keys);
            parts.push(re.to_string());
        }
        let re = regex::Regex::new(&format!("(?:{})", parts.join("|")))?;
        Ok(PathRegex { re, keys })
    }
}
