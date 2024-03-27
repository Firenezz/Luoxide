use crate::internal::syntax::LineAnnotated;

use super::*;

impl<'source> Parser<'source> {
    pub(super) fn unexpected_token(&mut self, token: &Token) -> LineAnnotated<SpannedError> {
        let line = self.lexer.get_current_line();

        LineAnnotated {
            line,
            value: SpannedError {
                span: token.span,
                error: Error::UnexpectedToken {
                    line,
                    token: token.clone(),
                },
            },
        }
    }
}
