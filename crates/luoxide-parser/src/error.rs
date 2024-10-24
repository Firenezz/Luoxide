use core::error;
use std::{num::ParseIntError, result};

use ecow::vec;
use luoxide_text::range::TextSpan;
use thiserror::Error;

use crate::token::{Token, TokenKind};

pub type Result<T> = result::Result<T, ParseError>;

pub enum Outcome<T, E> {
    Ok(T),
    PartialFailure(E),
    TotalFailure(E),
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseErrorKind {
    #[error("source file ended unexpectedly")]
    UnexpectedEof,
    #[error("found unexpected token")]
    UnexpectedToken {
        expected: Box<[TokenKind]>,
        found: TokenKind,
    },
    #[error("conversion of number returned an error")]
    InvalidNumber {
        #[from]
        inner_error: ParseIntError,
    },
}

#[derive(Debug)]
pub struct ParseError {
    pub error: ErrorKind,
    pub at: Option<TextSpan>,
}

impl ParseError {
    pub fn details(&self) -> (&'static str, Vec<String>) {
        match &self.error {
            ErrorKind::LexerError => todo!(),
            ErrorKind::ParserError { error_kind } => match &error_kind {
                ParseErrorKind::UnexpectedEof => ("The source file ended unexpectedly", vec![]),
                ParseErrorKind::UnexpectedToken { expected, found } => {
                    let found = match found {
                        _ => format!("a {found}"),
                    };

                    let messages = std::iter::once(format!("Found {found}, expected one of: "))
                        .chain(expected.iter().map(|s| format!("- {s}")))
                        .collect();

                    ("Found a token that was not expected", messages)
                }
                ParseErrorKind::InvalidNumber { inner_error } => (
                    match inner_error.kind() {
                        std::num::IntErrorKind::InvalidDigit => "Number is invalid",
                        std::num::IntErrorKind::PosOverflow => "Number is too big",
                        std::num::IntErrorKind::NegOverflow => "Number is too small",
                        std::num::IntErrorKind::Zero => {
                            unreachable!("Zero value are permitted in any numbers in lua")
                        }
                        std::num::IntErrorKind::Empty => {
                            unreachable!("Lexer should have not returned an empty lexeme")
                        }
                        _ => todo!(),
                    },
                    vec![],
                ),
            },
            ErrorKind::UnknownError(error) => ("Unknown error occured", vec![format!("{}", error)]),
        }
    }
}

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("")]
    LexerError,
    #[error("the parser encountered an error")]
    ParserError { error_kind: ParseErrorKind },
    #[error("an unknown error occurred")]
    UnknownError(#[from] Box<dyn std::error::Error>),
}