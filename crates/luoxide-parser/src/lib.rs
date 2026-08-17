/// The AST is always available; the lexer/parser live behind the `parse`
/// feature, so `default-features = false` gives an AST-only dependency.
pub mod ast;

#[cfg(feature = "parse")]
#[macro_use]
pub mod macros;
#[cfg(feature = "parse")]
pub mod diagnostic;
#[cfg(feature = "parse")]
pub mod error;
#[cfg(feature = "parse")]
pub mod lexer;
#[cfg(feature = "parse")]
pub mod outcome;
#[cfg(feature = "parse")]
pub mod parser;
#[cfg(feature = "parse")]
pub mod token;
#[cfg(feature = "parse")]
pub mod token_set;
