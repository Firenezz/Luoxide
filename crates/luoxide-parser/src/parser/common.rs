use luoxide_ast::ast;

use super::{error, Parser};
use crate::{error::ParseErrorKind, token::{Token, TokenKind}};

impl Parser<'_> {
    pub(super) fn must(&mut self, token: TokenKind) -> CheckStatus {
        if !self.current().is(token) {
            self.unexpected_token([token]);
            return CheckStatus::Failed;
        }
        self.bump();

        CheckStatus::Success
    }

    pub(super) fn must_not(&mut self, token: TokenKind) -> CheckStatus {
        if self.current().is(token) {
            self.unexpected_token([token]);
            return CheckStatus::Failed;
        }
        self.bump();

        CheckStatus::Success
    }

    pub(super) const fn must_be_in<const N: usize>(&mut self, tokens: [TokenKind; N]) -> CheckStatus {
        if tokens {
            self.unexpected_token([token]);
            return CheckStatus::Failed;
        }
        self.bump();

        CheckStatus::Success
    }

    #[inline]
    pub(super) fn current_is(&self, token: TokenKind) -> bool {
        self.current().is(token)
    }

    #[inline]
    pub(super) fn current_is_not(&self, token: TokenKind) -> bool {
        !self.current().is(token)
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
    pub fn current(&self) -> &Token {
        self.lexer.current()
    }

    pub fn bump(&mut self) {
        self.lexer.bump();
    }
}

impl<'src> Parser<'src> {
    pub fn get_lexeme(&self, token: &Token) -> &'src str {
        self.lexer.lexeme(&token)
    }
}

impl Parser<'_> {
    pub fn parse_statlist(&mut self) -> Vec<()> {

        todo!()
    }

    pub fn parse_field_selector(&mut self) -> ast::expressions::Expression {
        let current = self.current();



        match current.kind() {
            

            _ => 
        }
    }
}

pub enum CheckStatus {
    Success,
    Failed,
}
