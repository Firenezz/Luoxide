use super::error;

impl Parser<'_> {
    pub(super) fn must(&mut self, token: TokenKind) {
        if !self.current().is(token) {
            self.error_context
                .push(ParseErrorKind::ExpectedToken(token));
        }
        self.bump();
    }

    pub(super) fn must_not(&mut self, token: TokenKind) -> ParserActionStatus {
        if self.current().is(token) {
            self.unexpected_token([token]);
            return ParserActionStatus::Failed;
        }
        self.bump();

        ParserActionStatus::Success
    }

    #[inline]
    pub(super) fn current_is(&self, token: TokenKind) -> bool {
        self.current().is(token)
    }

    #[inline]
    pub(super) fn current_is_not(&self, token: TokenKind) -> bool {
        !self.current().is(token)
    }
}

pub enum ParserActionStatus {
    Success,
    Failed,
}
