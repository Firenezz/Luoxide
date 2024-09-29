use luoxide_ast::ast::{self, Field};

use super::{error, Parser};
use crate::{error::ParseErrorKind, token::{Token, TokenKind}, token_set::TokenSet};

impl Parser<'_> {
    pub(super) fn must(&mut self, token: TokenKind) -> CheckStatus {
        if !self.current_token().is(token) {
            self.unexpected_token([token]);
            return CheckStatus::Failed;
        }
        self.bump();

        CheckStatus::Success
    }

    pub(super) fn must_not(&mut self, token: TokenKind) -> CheckStatus {
        if self.current_token().is(token) {
            self.unexpected_token([token]);
            return CheckStatus::Failed;
        }
        self.bump();

        CheckStatus::Success
    }

    pub(super) fn must_be_in<const N: usize>(&mut self, tokens: [TokenKind; N]) -> CheckStatus {
        let token_set = TokenSet::new(tokens);
        if token_set.contains(*self.current_token().kind()) {
            self.unexpected_token(tokens);
            return CheckStatus::Failed;
        }
        self.bump();

        CheckStatus::Success
    }

    #[inline]
    pub(super) fn current_is(&self, token: TokenKind) -> bool {
        self.current_token().is(token)
    }

    #[inline]
    pub(super) fn current_is_not(&self, token: TokenKind) -> bool {
        !self.current_token().is(token)
    }

    pub fn check_identifier(&mut self) -> Option<ast::Identifier> {
        match self.must(token!(identifier)) {
            CheckStatus::Success => Some(
                ast::Identifier::create_identifier(self.get_lexeme(self.current()))
            ),
            CheckStatus::Failed => None
        }
    }
}

impl Parser<'_> {

    pub fn bump(&mut self) -> Result<()> {
        self.lexer.bump();

        if  {
            
        }
    }
}

impl<'src> Parser<'src> {
    pub fn get_lexeme(&self, token: &Token) -> &'src str {
        self.lexer.lexeme(&token)
    }
}

pub enum CheckStatus {
    Success,
    Failed,
}
