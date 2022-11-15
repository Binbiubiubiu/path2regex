use regex::Regex;

use crate::ast::Token;

struct PathRegex(Regex);

impl From<Vec<Token>> for PathRegex {
    fn from(_: Vec<Token>) -> Self {
        Self(Regex::new("").unwrap())
    }
}
