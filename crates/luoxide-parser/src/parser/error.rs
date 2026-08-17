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

    pub fn add_error(&mut self, error: ParseError) {
        // `NestingTooDeep` is reported once: retrying the same opener would
        // otherwise fill memory with duplicate diagnostics.
        if error.is_nesting_too_deep() && self.errors.iter().any(ParseError::is_nesting_too_deep) {
            return;
        }
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

fn parser_error(kind: ParseErrorKind, at: Option<TextSpan>) -> ParseError {
    ParseError::new(ErrorKind::ParserError { error_kind: kind }, at)
}

impl Parser<'_> {
    #[track_caller]
    pub(super) fn unexpected_token<const N: usize>(
        &self,
        expected: [TokenKind; N],
        found: &TokenKind,
        at: Option<TextSpan>,
    ) -> ParseError {
        ParseError::capturing(
            ErrorKind::ParserError {
                error_kind: ParseErrorKind::UnexpectedToken {
                    expected: Box::from(expected),
                    found: *found,
                },
            },
            at,
        )
    }

    pub(super) fn unexpected_eof(&self, at: Option<TextSpan>) -> ParseError {
        parser_error(ParseErrorKind::UnexpectedEof, at)
    }

    /// Error for a token the lexer already flagged as invalid.
    pub(super) fn lexer_error(&self, at: Option<TextSpan>) -> ParseError {
        ParseError::new(ErrorKind::LexerError, at)
    }

    pub(super) fn int_parse_error(
        &self,
        error: std::num::ParseIntError,
        at: Option<TextSpan>,
    ) -> ParseError {
        parser_error(error.into(), at)
    }

    pub(super) fn malformed_number(&self, at: Option<TextSpan>) -> ParseError {
        parser_error(ParseErrorKind::MalformedNumber, at)
    }

    pub(super) fn invalid_escape(&self, at: Option<TextSpan>) -> ParseError {
        parser_error(ParseErrorKind::InvalidEscape, at)
    }

    pub(super) fn nesting_too_deep(&self, at: Option<TextSpan>) -> ParseError {
        parser_error(ParseErrorKind::NestingTooDeep, at)
    }

    pub(super) fn non_call_expression_statement(&self, at: Option<TextSpan>) -> ParseError {
        parser_error(ParseErrorKind::NonCallExpressionStatement, at)
    }

    pub(super) fn invalid_assignment_target(&self, at: Option<TextSpan>) -> ParseError {
        parser_error(ParseErrorKind::InvalidAssignmentTarget, at)
    }

    pub fn reserved_keyword(&self, at: Option<TextSpan>) -> ParseError {
        parser_error(ParseErrorKind::ReservedKeyword, at)
    }
}
