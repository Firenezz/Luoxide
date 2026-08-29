//! Lua lexer, parser, and AST.
//!
//! [`ast`] is always available. Lexer and parser require the `parse` feature
//! (on by default). `default-features = false` is AST-only.

/// Abstract syntax tree. Available without the `parse` feature.
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
