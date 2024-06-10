use core::fmt::Display;

use thiserror::Error;

use self::lexer::{LineInfo, Token, TokenKind};

#[macro_use]
mod macros;
pub mod ast;
pub mod fold;
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
    #[error("Malformed short string")]
    MalformedShortString,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // TODO: allow until parser is done
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
}

impl SyntaxError {
    #[allow(unused_variables)] // TODO: allow until parser is done
    pub fn new(line: usize, column: usize, kind: SyntaxErrorKind) -> Self {
        Self { kind }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SyntaxError: {}", self.kind)
    }
}

pub fn get_reserved_word_token<S>(word: &[u8]) -> Option<TokenKind> {
    match word {
        b"break" => Some(TokenKind::Break),
        b"do" => Some(TokenKind::Do),
        b"else" => Some(TokenKind::Else),
        b"elseif" => Some(TokenKind::ElseIf),
        b"end" => Some(TokenKind::End),
        b"function" => Some(TokenKind::Function),
        b"goto" => Some(TokenKind::Goto),
        b"if" => Some(TokenKind::If),
        b"in" => Some(TokenKind::In),
        b"local" => Some(TokenKind::Local),
        b"nil" => Some(TokenKind::Nil),
        b"for" => Some(TokenKind::For),
        b"while" => Some(TokenKind::While),
        b"repeat" => Some(TokenKind::Repeat),
        b"until" => Some(TokenKind::Until),
        b"return" => Some(TokenKind::Return),
        b"then" => Some(TokenKind::Then),
        b"true" => Some(TokenKind::Lit_Bool(true)),
        b"false" => Some(TokenKind::Lit_Bool(false)),
        b"not" => Some(TokenKind::Not),
        b"and" => Some(TokenKind::And),
        b"or" => Some(TokenKind::Or),
        _ => None,
    }
}

#[allow(dead_code)]
fn report_error(_errors: Vec<crate::error::SpannedSyntaxError>) -> String {
    todo!();
}
