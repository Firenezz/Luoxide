use crate::ast;
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser<'_> {
    /// Consumes and returns the current token if it matches `token`.
    pub(super) fn expect(&mut self, token: TokenKind) -> Option<Token> {
        if !self.current_token().is(token) {
            return None;
        }
        let token = *self.current();
        self.bump();

        Some(token)
    }

    /// Like [`expect`](Self::expect) but records an `UnexpectedToken` error
    /// when the current token does not match.
    pub(super) fn expect_or_error(&mut self, token: TokenKind) -> Option<Token> {
        match self.expect(token) {
            Some(token) => Some(token),
            None => {
                let current = *self.current_token();
                let error = self.unexpected_token([token], current.kind(), Some(current.span));
                self.error_context.add_error(error);
                None
            }
        }
    }

    #[inline]
    pub(super) fn current_is(&self, token: TokenKind) -> bool {
        self.current_token().is(token)
    }

    #[inline]
    pub(super) fn current_is_not(&self, token: TokenKind) -> bool {
        !self.current_token().is(token)
    }

    #[inline]
    pub(super) fn is_at_end(&self) -> bool {
        self.current_is(token!(EOF))
    }

    /// If the current token is an identifier, consume it and build an
    /// [`ast::Identifier`], otherwise return `None`.
    pub(super) fn maybe_identifier(&mut self) -> Option<ast::Identifier> {
        self.expect(token!(identifier))
            .map(|token| ast::Identifier::new(self.get_lexeme(&token), token.span))
    }

    /// Like [`maybe_identifier`](Self::maybe_identifier) but fails with an
    /// `UnexpectedToken` error when the current token is not an identifier.
    pub(super) fn require_identifier(&mut self) -> crate::error::Result<ast::Identifier> {
        match self.maybe_identifier() {
            Some(identifier) => Ok(identifier),
            None => {
                let current = *self.current_token();
                Err(self.unexpected_token(
                    [token!(identifier)],
                    current.kind(),
                    Some(current.span),
                ))
            }
        }
    }
}

impl Parser<'_> {
    #[inline]
    pub fn current(&self) -> &Token {
        self.lexer.current()
    }

    #[inline]
    pub fn current_kind(&self) -> &TokenKind {
        self.lexer.current().kind()
    }

    #[inline]
    pub fn bump(&mut self) {
        self.lexer.bump();
    }

    /// Span of the most recently consumed token; useful for closing spans
    /// after `bump`/`expect`.
    #[inline]
    pub(super) fn previous_span(&self) -> luoxide_text::range::TextSpan {
        self.lexer.previous().span
    }
}

impl<'src> Parser<'src> {
    pub fn get_lexeme(&self, token: &Token) -> &'src str {
        self.lexer.lexeme(token)
    }
}
