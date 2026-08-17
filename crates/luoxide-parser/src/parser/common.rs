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

    /// Required token: if the current token is `token`, consume it.
    ///
    /// On mismatch, record an error and leave the token in place so the
    /// caller can still recover (missing `then` must not eat the next name).
    pub(super) fn expect(&mut self, token: TokenKind) -> Option<Token> {
        if !self.current_token().is(token) {
            let current = *self.current_token();
            let error = self.unexpected_token([token], current.kind(), Some(current.span));
            self.error_context.add_error(error);
            return None;
        }
        let found = *self.current();
        self.bump();
        Some(found)
    }

    /// Like [`expect`](Self::expect) but fails with an `UnexpectedToken` error
    /// when the current token does not match.
    pub(super) fn require(&mut self, token: TokenKind) -> crate::error::Result<Token> {
        match self.maybe(token) {
            Some(token) => Ok(token),
            None => {
                let current = *self.current_token();
                Err(self.unexpected_token([token], current.kind(), Some(current.span)))
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

    /// If the current token can be a Lua `Name`, consume it and build an
    /// [`ast::Identifier`]. Extra reserved words (`const`, `enum`, ...) are
    /// allowed here: they are not Lua keywords, so `local const <const>` and
    /// `local <const> x` parse even though `const` is a distinct token.
    pub(super) fn maybe_identifier(&mut self) -> Option<ast::Identifier> {
        let kind = self.current_token().kind;
        if !kind.is_name() {
            return None;
        }
        let token = *self.current_token();
        self.bump();
        Some(ast::Identifier::new(self.get_lexeme(&token), token.span))
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
