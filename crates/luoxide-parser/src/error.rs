use core::error;
use std::{num::ParseIntError, rc::Rc, result};

use luoxide_text::range::TextSpan;
use thiserror::Error;

use crate::token::{Token, TokenKind};

pub type Result<T> = result::Result<T, ParseError>;

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
    #[error("multiple errors occurred in a series")]
    ParseSeriesFailed {
        inner_errors: Vec<ParseError>,
    },
    #[error("usage of a reserved keyword")]
    ReservedKeyword
}

impl ParseErrorKind {
    pub(crate) fn flatten(&self) -> Option<&Vec<ParseError>> {
        match self {
            Self::ParseSeriesFailed { inner_errors } => Some(inner_errors),
            _ => None
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

impl ErrorKind {
    pub(super) fn from_parser_error(kind: ParseErrorKind) -> Self {
        ErrorKind::ParserError { error_kind: kind }
    }
    pub(super) fn from_unknown_error(error: Box<dyn std::error::Error>) -> Self {
        ErrorKind::from(error)
    }
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
                        _ => unimplemented!("Number parser must have added a new error"),
                    },
                    vec![],
                ),
                ParseErrorKind::ReservedKeyword => (
                    "Found a reserved keyword",
                    std::iter::once(format!("Found a reserved keyword, reserved keywords are: "))
                        .chain(token!(reserved_set).iter().map(|s| format!("- {s}")))
                        .collect()
                ),
                ParseErrorKind::ParseSeriesFailed { .. } => {
                    ("A series returned an error", vec![])
                }

            },
            ErrorKind::UnknownError(error) => ("Unknown error occured", vec![format!("{}", error)]),
        }
    }

    pub(crate) fn series_from_vec(vec: Vec<ParseError>, at: TextSpan) -> ParseError {
        ParseError {
            error: ErrorKind::from_parser_error(ParseErrorKind::ParseSeriesFailed { inner_errors: vec }),
            at: Some(at)
        }
    }
}

