use ecow::EcoVec;
use luoxide_ast::ast::{self, Field};
use luoxide_text::traits::Ranged;
use tracing::event;
use tracing::Level;

use super::{error, Parser};
use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::outcome::Outcome;
use crate::{
    error::ParseErrorKind,
    token::{Token, TokenKind},
    token_set::TokenSet,
};

use crate::error::Result as ParseResult;

impl Parser<'_> {
    pub(super) fn expect(&mut self, token: TokenKind) -> Option<Token> {
        if !self.current_token().is(token) {
            return None;
        }
        let token = *self.current();
        self.bump();

        Some(token)
    }

    /*pub(super) fn must_not(&mut self, token: TokenKind) -> CheckStatus {
        if self.current_token().is(token) {
            self.unexpected_token([token]);
            return CheckStatus::Failed;
        }
        self.bump();

        CheckStatus::Success
    }*/

    pub(super) fn expect_one_of<const N: usize>(&mut self, tokens: [TokenKind; N]) -> Option<Token> {
        let token_set = TokenSet::new(tokens);
        if token_set.contains(*self.current_token().kind()) {
            return None;
        }
        let token = *self.current();
        self.bump();

        Some(token)
    }

    pub(super) fn series_of<AST: Clone>(
        &mut self,
        parser: &impl Fn(&mut Self) -> ParseResult<AST>,
        separator: TokenKind
    ) -> Outcome<EcoVec<AST>, Vec<ParseError>> {
        let mut results = EcoVec::new();
        let mut errors = Vec::new();
        while !self.is_at_end() {
            match parser(self) {
                Ok(node) => {
                    results.push(node);
                },
                Err(error) => errors.push(error),
            }

            match self.expect(separator) {
                Some(..) => (),
                None => {
                    if errors.len() > 0 {
                        return Outcome::PartialFailure(results, errors);
                    } else {
                        return Outcome::Ok(results);
                    }
                },
            }

        };

        if errors.len() > 0 {
            return Outcome::PartialFailure(results, errors);
        } else {
            return Outcome::Ok(results);
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

    /// If next token is a Identifier, consume it and return the relevant info, otherwise, return None
    pub fn maybe_identifier(&mut self) -> Option<ast::Identifier> {
        match self.expect(token!(identifier)) {
            Some(name_token) => Some(ast::Identifier::create_identifier(
                self.get_lexeme(&name_token),
            )),
            None => None,
        }
    }
}

impl Parser<'_> {
    #[inline]
    pub fn current(&self) -> &Token {
        self.lexer.previous()
    }

    #[inline]
    pub fn current_kind(&self) -> &TokenKind {
        self.lexer.previous().kind()
    }
    #[inline]
    pub fn next_token(&self) -> &Token {
        self.lexer.current()
    }

    #[inline]
    pub fn next_token_kind(&self) -> &TokenKind {
        self.lexer.current().kind()
    }

    #[inline]
    pub fn bump(&mut self) {
        self.lexer.bump();
    }

    /*pub fn synchronize(&mut self, synchronize_points: TokenSet) {
        event!(Level::TRACE, "Synchronizing parser");
        while synchronize_points.contains(self.lexer.current().kind)
            && self.current_is_not(token!(EOF))
        {
            self.bump();
        }
    }*/
}

impl<'src> Parser<'src> {
    pub fn get_lexeme(&self, token: &Token) -> &'src str {
        self.lexer.lexeme(&token)
    }
}

pub enum CheckStatus<Token> {
    Success(Token),
    Failed,
}
