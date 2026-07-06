use luoxide_text::range::TextSpan;

use crate::{
    error::{ErrorKind, ParseError, ParseErrorKind},
    token::TokenKind,
};

use super::Parser;

pub struct ErrorContext {
    pub errors: Vec<ParseError>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: crate::parser::ParseError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn take_errors(&mut self) -> Vec<ParseError> {
        std::mem::take(&mut self.errors)
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            println!("{:?}", error);
        }
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<'_> {
    pub(super) fn unexpected_token<const N: usize>(
        &self,
        expected: [TokenKind; N],
        found: &TokenKind,
        at: Option<TextSpan>,
    ) -> ParseError {
        ParseError {
            error: ErrorKind::ParserError {
                error_kind: ParseErrorKind::UnexpectedToken {
                    expected: Box::from(expected),
                    found: *found,
                },
            },
            at,
        }
    }

    pub(super) fn unexpected_eof(&self, at: Option<TextSpan>) -> ParseError {
        ParseError {
            error: ErrorKind::ParserError {
                error_kind: ParseErrorKind::UnexpectedEof,
            },
            at, // CHECK, if possible use the TextSize
        }
    }

    pub(super) fn int_parse_error(&self, error: std::num::ParseIntError, at: Option<TextSpan>) -> ParseError {
        ParseError {
            error: ErrorKind::ParserError {
                error_kind: error.into(),
            },
            at,
        }
    }

    pub fn reserved_keyword(&self, at: Option<TextSpan>) -> ParseError {
        ParseError {
            error: ErrorKind::ParserError {
                error_kind: ParseErrorKind::ReservedKeyword,
            },
            at,
        }
    }

    
}
