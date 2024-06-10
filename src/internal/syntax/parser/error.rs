use crate::internal::syntax::{LineAnnotated, SyntaxErrorKind};

use super::*;

#[allow(dead_code)] // TODO: remove this after ast is finished
impl<'source> Parser<'source> {
    pub const NEAR_MESSAGE: &'static str = "near";

    pub(super) fn unexpected_token(&self, token: &Token) -> LineAnnotated<SpannedSyntaxError> {
        let line = self.lexer.get_current_line();

        LineAnnotated {
            line,
            value: SpannedSyntaxError {
                span: token.span,
                error: SyntaxError {
                    kind: SyntaxErrorKind::UnexpectedToken {
                        token: token.clone(),
                    },
                },
            },
        }
    }

    pub(super) fn reserved_word(&self) -> LineAnnotated<SpannedSyntaxError> {
        let line = self.lexer.get_current_line();

        LineAnnotated {
            line,
            value: SpannedSyntaxError {
                span: self.lexer.current().span,
                error: SyntaxError {
                    kind: SyntaxErrorKind::ReservedWord {
                        word: self.lexer.current().span.to_string(),
                    },
                },
            },
        }
    }

    /*pub(super) fn malformed_short_string(&self, span: Span, message: &str){

    }*/
}

pub struct ErrorContext {
    pub(crate) errors: Vec<LineAnnotated<SpannedSyntaxError>>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn push(&mut self, error: LineAnnotated<SpannedSyntaxError>) {
        self.errors.push(error);
    }
}
