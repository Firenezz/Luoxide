use std::{num::ParseIntError, result};

use luoxide_text::range::TextSpan;
use thiserror::Error;

use crate::token::TokenKind;

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
    #[error("number literal is malformed")]
    MalformedNumber,
    #[error("string literal contains an invalid escape sequence")]
    InvalidEscape,
    #[error("expressions are nested too deeply")]
    NestingTooDeep,
    #[error("expected a statement")]
    ExpectedStatement { found: TokenKind },
    #[error("expression cannot be used as a statement")]
    NonCallExpressionStatement,
    #[error("expression cannot be assigned to")]
    InvalidAssignmentTarget,
    #[error("multiple errors occurred in a series")]
    ParseSeriesFailed { inner_errors: Vec<ParseError> },
    #[error("usage of a reserved keyword")]
    ReservedKeyword,
}

impl ParseErrorKind {
    pub(crate) fn flatten(&self) -> Option<&Vec<ParseError>> {
        match self {
            Self::ParseSeriesFailed { inner_errors } => Some(inner_errors),
            _ => None,
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
    pub fn is_nesting_too_deep(&self) -> bool {
        matches!(
            self.error,
            ErrorKind::ParserError {
                error_kind: ParseErrorKind::NestingTooDeep
            }
        )
    }

    pub fn details(&self) -> (&'static str, Vec<String>) {
        match &self.error {
            ErrorKind::LexerError => ("The lexer encountered an error", vec![]),
            ErrorKind::ParserError { error_kind } => match &error_kind {
                ParseErrorKind::UnexpectedEof => ("The source file ended unexpectedly", vec![]),
                ParseErrorKind::UnexpectedToken { expected, found } => {
                    let mut messages = vec![format!("found {}", found.describe())];
                    if expected.len() == 1 {
                        messages.push(format!("expected {}", expected[0].describe()));
                    } else {
                        messages.push("expected one of:".to_string());
                        messages.extend(expected.iter().map(|kind| format!("- {}", kind.describe())));
                    }
                    ("unexpected token", messages)
                }
                ParseErrorKind::InvalidNumber { inner_error } => (
                    match inner_error.kind() {
                        std::num::IntErrorKind::InvalidDigit => "Number is invalid",
                        std::num::IntErrorKind::PosOverflow => "Number is too big",
                        std::num::IntErrorKind::NegOverflow => "Number is too small",
                        // Zero/Empty cannot be produced by the parser's int
                        // paths; report generically instead of panicking.
                        _ => "Number could not be parsed",
                    },
                    vec![],
                ),
                ParseErrorKind::MalformedNumber => ("Number literal is malformed", vec![]),
                ParseErrorKind::InvalidEscape => (
                    "String literal contains an invalid escape sequence",
                    vec![],
                ),
                ParseErrorKind::NestingTooDeep => (
                    "Expressions or blocks are nested too deeply",
                    vec!["Reduce the nesting depth of the code".to_string()],
                ),
                ParseErrorKind::ExpectedStatement { found } => (
                    "Expected a statement",
                    vec![format!("Found a {found} instead")],
                ),
                ParseErrorKind::NonCallExpressionStatement => (
                    "Only function calls can be used as statements",
                    vec!["Assign the value or call a function instead".to_string()],
                ),
                ParseErrorKind::InvalidAssignmentTarget => (
                    "This expression cannot be assigned to",
                    vec!["Only names, fields (a.b) and indexes (a[b]) are assignable".to_string()],
                ),
                ParseErrorKind::ReservedKeyword => (
                    "Found a reserved keyword",
                    std::iter::once(format!("Found a reserved keyword, reserved keywords are: "))
                        .chain(token!(reserved_set).iter().map(|s| format!("- {s}")))
                        .collect(),
                ),
                ParseErrorKind::ParseSeriesFailed { .. } => ("A series returned an error", vec![]),
            },
            ErrorKind::UnknownError(error) => ("Unknown error occured", vec![format!("{}", error)]),
        }
    }

    pub(crate) fn series_from_vec(vec: Vec<ParseError>, at: TextSpan) -> ParseError {
        ParseError {
            error: ErrorKind::from_parser_error(ParseErrorKind::ParseSeriesFailed {
                inner_errors: vec,
            }),
            at: Some(at),
        }
    }
}
