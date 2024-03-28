use core::fmt::Display;
use std::clone;

use thiserror::Error;

use crate::error::SpannedError;

use self::lexer::{LineInfo, Token};

pub mod ast;
pub mod lexer;
pub mod parser;

#[derive(Debug, Clone)]
pub struct LineAnnotated<T> {
    pub line: LineInfo,
    pub value: T,
}

#[derive(Error, Debug, Clone)]
pub enum SyntaxErrorKind {
    #[error("Unexpected token: {token:?}")]
    UnexpectedToken { token: Token },
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Reserved word: {word}")]
    ReservedWord { word: String },
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // TODO: allow until parser is done
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
}

impl SyntaxError {
    pub fn new(line: usize, column: usize, kind: SyntaxErrorKind) -> Self {
        Self { kind }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SyntaxError: {}", self.kind)
    }
}
