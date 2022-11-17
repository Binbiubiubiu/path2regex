#[cfg(feature = "compile")]
mod compiler;
#[cfg(feature = "match")]
mod matcher;
mod parser;
mod re;

pub(crate) trait Builder<T> {
    fn build(self) -> T;
}

#[cfg(feature = "compile")]
pub use compiler::{CompilerBuilder, CompilerOptions};
#[cfg(feature = "match")]
pub use matcher::{MatcherBuilder, MatcherOptions};
pub use parser::{ParserBuilder, ParserOptions};
pub use re::{PathRegexBuilder, PathRegexOptions};
