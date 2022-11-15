//! 2323

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub, private_in_public)]

mod ast;
pub mod compiler;
mod into_regex;
mod parser;

pub use ast::{Key, Token};
pub use parser::{parse,parse_with_options, Parser,ParserBuilder,ParserOptions};
