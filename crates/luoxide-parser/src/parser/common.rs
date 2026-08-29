use crate::ast::{self, NodeList};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser<'_> {
    /// Consumes and returns the current token if it matches `token`.
    pub(super) fn maybe(&mut self, token: TokenKind) -> Option<Token> {
        if !self.current_token().is(token) {
            return None;
        }
        let token = *self.current();
        self.bump();

        Some(token)
    }

    /// Consumes the current token if it is `token`.
    ///
    /// On mismatch, records an error and leaves the token unconsumed.
    pub(super) fn expect(&mut self, token: TokenKind) -> Option<Token> {
        if !self.current_token().is(token) {
            let current = *self.current_token();
            let error = self.unexpected_token([token], current.kind(), Some(current.span));
            self.record_error(error);
            return None;
        }
        let found = *self.current();
        self.bump();
        Some(found)
    }

    /// [`maybe`](Self::maybe), or `UnexpectedToken` if the current token does not match.
    #[allow(dead_code)]
    #[inline]
    pub(super) fn require(&mut self, token: TokenKind) -> crate::error::Result<Token> {
        self.maybe(token).ok_or_else(|| {
            let current = *self.current_token();
            self.unexpected_token([token], current.kind(), Some(current.span))
        })
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

    /// Consume the current token if [`TokenKind::is_name`](crate::token::TokenKind::is_name)
    /// and intern it as an [`ast::Identifier`].
    pub(super) fn maybe_identifier(&mut self) -> Option<ast::Identifier> {
        let kind = self.current_token().kind;
        if !kind.is_name() {
            return None;
        }
        let token = *self.current_token();
        self.bump();
        let lexeme = self.get_lexeme(&token);
        let name = self.intern.intern_name(lexeme);
        Some(ast::Identifier::new(name, token.span))
    }

    /// Like [`maybe_identifier`](Self::maybe_identifier) but fails with an
    /// `UnexpectedToken` error when the current token is not an identifier.
    pub(super) fn require_identifier(&mut self) -> crate::error::Result<ast::Identifier> {
        match self.maybe_identifier() {
            Some(identifier) => Ok(identifier),
            None => {
                let current = *self.current_token();
                Err(self.unexpected_token([token!(identifier)], current.kind(), Some(current.span)))
            }
        }
    }

    pub(super) fn parse_list<T>(
        &mut self,
        separator: TokenKind,
        mut parse_item: impl FnMut(&mut Self) -> crate::error::Result<T>,
    ) -> crate::error::Result<NodeList<T>> {
        let mut list = NodeList::new();
        loop {
            list.push(parse_item(self)?);
            if self.maybe(separator).is_none() {
                break;
            }
        }
        Ok(list)
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
        self.trace_eat();
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
