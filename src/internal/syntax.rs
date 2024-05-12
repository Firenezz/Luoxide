use core::fmt::Display;

use thiserror::Error;

use self::lexer::{LineInfo, Token, TokenKind};

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
        b"break" => Some(TokenKind::Kw_Break),
        b"do" => Some(TokenKind::Kw_Do),
        b"else" => Some(TokenKind::Kw_Else),
        b"elseif" => Some(TokenKind::Kw_ElseIf),
        b"end" => Some(TokenKind::Kw_End),
        b"function" => Some(TokenKind::Kw_Function),
        b"goto" => Some(TokenKind::Kw_Goto),
        b"if" => Some(TokenKind::Kw_If),
        b"in" => Some(TokenKind::Kw_In),
        b"local" => Some(TokenKind::Kw_Local),
        b"nil" => Some(TokenKind::Kw_Nil),
        b"for" => Some(TokenKind::Kw_For),
        b"while" => Some(TokenKind::Kw_While),
        b"repeat" => Some(TokenKind::Kw_Repeat),
        b"until" => Some(TokenKind::Kw_Until),
        b"return" => Some(TokenKind::Kw_Return),
        b"then" => Some(TokenKind::Kw_Then),
        b"true" => Some(TokenKind::Lit_Bool(true)),
        b"false" => Some(TokenKind::Lit_Bool(false)),
        b"not" => Some(TokenKind::Kw_Not),
        b"and" => Some(TokenKind::Kw_And),
        b"or" => Some(TokenKind::Kw_Or),
        _ => None,
    }
}
