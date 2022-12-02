#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(private_in_public, unreachable_pub, missing_docs, rust_2018_idioms)]
#![doc = include_str!("../README.md")]

mod ast;
#[cfg(feature = "compile")]
#[cfg_attr(docsrs, doc(cfg(feature = "compile")))]
mod compiler;
#[cfg(feature = "match")]
#[cfg_attr(docsrs, doc(cfg(feature = "match")))]
mod matcher;
mod parser;
mod re;
mod try_into_with;

pub use ast::{Key, Token};
pub use parser::{Parser, ParserBuilder, ParserOptions};
pub use re::{PathRegex, PathRegexBuilder, PathRegexOptions};
pub use try_into_with::TryIntoWith;

#[cfg(feature = "compile")]
pub use compiler::{Compiler, CompilerBuilder, CompilerOptions};
#[cfg(feature = "match")]
pub use matcher::{MatchResult, Matcher, MatcherBuilder, MatcherOptions};
/// The matching trailing character is used for 'end' and 'ends_with' configuration item filtering
pub const DEFAULT_DELIMITER: &str = "/#?";

mod internal {
    pub(crate) use regex::escape as escape_string;
    #[cfg(any(feature = "compile", feature = "match"))]
    pub(crate) use serde_json::Value as DataValue;

    #[inline]
    pub(crate) fn type_of<T>(_: T) -> String {
        std::any::type_name::<T>().to_string()
    }

    pub(crate) type FnStr = for<'a> fn(&'a str) -> String;
    #[cfg(any(feature = "compile", feature = "match"))]
    pub(crate) type FnStrWithKey = for<'a> fn(&'a str, &'a crate::Key) -> String;

    pub(crate) const END_WITH_DELIMITER: &str = "END_WITH_DELIMITER";
}
