//! 2323

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(private_in_public, unreachable_pub)]
#![warn(missing_docs, rust_2018_idioms)]

mod ast;
mod compiler;
mod matcher;
mod parser;
mod re;
mod re_builder;
mod try_pipe;

pub use ast::{Key, Token};
pub use compiler::{Compiler, CompilerBuilder};
pub use matcher::{Matcher, MatcherBuilder};
pub use parser::{Parser, ParserBuilder};
pub use re::PathRegex;
pub use re_builder::PathRegexBuilder;
pub use serde_json::Value;

pub(crate) use regex::escape as escape_string;
pub(crate) fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
