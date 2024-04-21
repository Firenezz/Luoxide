use crate::internal::syntax::{LineAnnotated, SyntaxErrorKind};

use super::*;

#[allow(dead_code)] // TODO: remove this after ast is finished
impl<'source> Parser<'source> {
    pub(super) fn unexpected_token(&mut self, token: &Token) -> LineAnnotated<SpannedError> {
        let line = self.lexer.get_current_line();

        LineAnnotated {
            line,
            value: SpannedError {
                span: token.span,
                error: SyntaxError {
                    kind: SyntaxErrorKind::UnexpectedToken {
                        token: token.clone(),
                    },
                },
            },
        }
    }

    pub(super) fn reserved_word(&mut self) -> LineAnnotated<SpannedError> {
        let line = self.lexer.get_current_line();

        LineAnnotated {
            line,
            value: SpannedError {
                span: self.lexer.current().span,
                error: SyntaxError {
                    kind: SyntaxErrorKind::ReservedWord {
                        word: self.lexer.current().span.to_string(),
                    },
                },
            },
        }
    }
}
