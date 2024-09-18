use std::{num::ParseIntError, ops::Deref};

use luoxide_text::range::TextSpan;

use crate::{
    error::{ParseErrorKind, ParseError},
    token::TokenKind,
};

use super::Parser;

#[derive(Debug)]
pub struct Spanned<T> { pub value: T, pub span: TextSpan }

impl<T> Spanned<T> {
    pub fn new(value: T, span: TextSpan) -> Self {
        Self { value, span }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl Deref for Spanned<ParseError> {
    type Target = ParseError;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub struct ErrorContext {
    pub errors: Vec<Spanned<ParseError>>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: Spanned<ParseError>) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn take_errors(&mut self) -> Vec<Spanned<ParseError>> {
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
    pub(super) fn unexpected_token<const N: usize>(&mut self, expected: [TokenKind; N]) {
        self.error_context.add_error(Spanned {
            value: ParseError::ParserError {
                error_kind: ParseErrorKind::ExpectedToken {
                    expected: Box::from(&expected[..]),
                    found: *self.current_token(),
                },
            },
            span: self.current_token().span,
        });
    }

    pub(super) fn unexpected_eof<const N: usize>(&mut self, expected: Option<[TokenKind; N]>) {
        self.error_context.add_error(Spanned {
            value: ParseError::ParserError {
                error_kind: ParseErrorKind::UnexpectedEof {
                    expected: expected.map_or_else(|| None, |x| Some(Box::from(&x[..]))),
                },
            },
            span: self.current_token().span,
        });
    }

    pub(super) fn int_parse_error(&mut self, error: std::num::ParseIntError) {
        self.error_context.add_error(Spanned { value: error.into(), span: self.current_token().span });
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::ParserError {
            error_kind: ParseErrorKind::InvalidNumber { inner_error: Box::new(err) },
        }
    }
}
