//! 2323

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(private_in_public, unreachable_pub, missing_docs, rust_2018_idioms)]

mod ast;
mod builder;
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
#[cfg(feature = "compile")]
pub use builder::{CompilerBuilder, CompilerOptions};
#[cfg(feature = "match")]
pub use builder::{MatcherBuilder, MatcherOptions};
pub use builder::{ParserBuilder, ParserOptions, PathRegexBuilder, PathRegexOptions};

#[cfg(feature = "compile")]
pub use compiler::Compiler;
#[cfg(feature = "match")]
pub use matcher::Matcher;
pub use parser::Parser;
pub use re::PathRegex;
pub use try_into_with::TryIntoWith;

mod internal {
    pub(crate) type FnStr = for<'a> fn(&'a str) -> String;
    #[cfg(any(feature = "compile", feature = "match"))]
    pub(crate) type FnStrWithKey = for<'a> fn(&'a str, &'a crate::Key) -> String;
    pub(crate) use regex::escape as escape_string;
    #[cfg(any(feature = "compile", feature = "match"))]
    pub(crate) use serde_json::Value as DataValue;

    pub(crate) fn type_of<T>(_: T) -> String {
        std::any::type_name::<T>().to_string()
    }
}
