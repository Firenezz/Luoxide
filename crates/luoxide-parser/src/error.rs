use std::result;

use luoxide_text::range::TextSpan;
use thiserror::Error;

use crate::token::{Token, TokenKind};

pub type Result<T> = result::Result<T, ParseError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseErrorKind {
    #[error("unexpected <eof>, expected {0}")]
    UnexpectedEof {
        expected: Box<[TokenKind]>,
    },
    #[error("expected <>, expected {0}")]
    ExpectedToken {
        expected: Box<[TokenKind]>,
        found: Token,
    },
    UnexpectedEndOfInput {
        expected: Option<Box<[TokenKind]>>,
    },
    InvalidNumber {
        inner_error: Box<dyn std::error::Error>,
    },
}

impl core::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEof { expected } => match expected {
                Some(expected) => {
                    write!(f, "unexpected <eof>, expected ")?;
                    display_tokenkind_slice(f, expected)
                }
                None => write!(f, "unexpected <eof>"),
            },
            Self::ExpectedToken { expected, found } => {
                write!(f, "expected ")?;
                display_tokenkind_slice(f, expected)?;
                write!(f, ", found {}", found.kind())
            }
            Self::UnexpectedEndOfInput { .. } => write!(f, "unexpected end of input"),
            Self::InvalidNumber { .. } => write!(f, "invalid number"),
        }
    }
}

pub struct ErrorDetails {
    pub error: ParseError,
    pub at: Option<TextSpan>,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("")]
    LexerError,
    #[error("")]
    ParserError { #[from] error_kind: ParseErrorKind },
    #[error("an unknown error occurred")]
    UnknownError( #[from] Box<dyn std::error::Error>),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LexerError => write!(f, "lexer error"),
            Self::ParserError { error_kind } => {
                write!(f, "{}", error_kind)
            }
            Self::UnknownError(inner_error) => write!(f, "{}", inner_error),
        }
    }
}

impl core::fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)?;
        if !f.alternate() {
            return Ok(());
        }
        if let Some(at) = &self.at {
            write!(f, " at {}", at)
        } else {
            Ok(())
        }
    }
}

fn display_tokenkind_slice(
    f: &mut std::fmt::Formatter<'_>,
    slice: &[TokenKind],
) -> std::fmt::Result {
    for token in slice[0..slice.len() - 1].iter() {
        write!(f, "{:?}, ", token)?;
    }

    write!(f, "{:?}", slice[slice.len() - 1])
}
