use crate::{
    parser::{parse_with_options, ParserOptions},
    re::{regex_to_path_regex, string_to_path_regex},
    re_builder::PathRegexOptions,
    PathRegex, Token,
};

///
pub trait TryPipe<T, O>: Sized {
    fn try_pipe(&self, options: &O) -> eyre::Result<T>;
}

impl TryPipe<Vec<Token>, ParserOptions> for Vec<Token> {
    fn try_pipe(&self, _: &ParserOptions) -> eyre::Result<Vec<Token>> {
        Ok(self.clone())
    }
}

impl TryPipe<Vec<Token>, ParserOptions> for String {
    fn try_pipe(&self, options: &ParserOptions) -> eyre::Result<Vec<Token>> {
        parse_with_options(self, options)
    }
}

impl TryPipe<Vec<Token>, ParserOptions> for &str {
    fn try_pipe(&self, options: &ParserOptions) -> eyre::Result<Vec<Token>> {
        parse_with_options(self, options)
    }
}

impl TryPipe<PathRegex, PathRegexOptions> for regex::Regex {
    fn try_pipe(&self, _: &PathRegexOptions) -> eyre::Result<PathRegex> {
        let mut keys = vec![];
        let re = regex_to_path_regex(self.clone(), &mut keys)?;
        Ok(PathRegex { re, keys })
    }
}

impl TryPipe<PathRegex, PathRegexOptions> for String {
    fn try_pipe(&self, options: &PathRegexOptions) -> eyre::Result<PathRegex> {
        string_to_path_regex(self, options)
    }
}

impl<'a> TryPipe<PathRegex, PathRegexOptions> for &'a str {
    fn try_pipe(&self, options: &PathRegexOptions) -> eyre::Result<PathRegex> {
        string_to_path_regex(self, options)
    }
}

impl<T> TryPipe<PathRegex, PathRegexOptions> for Vec<T>
where
    T: TryPipe<PathRegex, PathRegexOptions>,
{
    fn try_pipe(&self, options: &PathRegexOptions) -> eyre::Result<PathRegex> {
        let mut keys = vec![];
        let mut parts = vec![];
        for path in self.iter() {
            let mut re = path.try_pipe(options)?;
            keys.append(&mut re.keys);
            parts.push(re.to_string());
        }
        let re = regex::Regex::new(&format!("(?:{})", parts.join("|")))?;
        Ok(PathRegex { re, keys })
    }
}
