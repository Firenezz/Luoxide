use crate::internal::syntax::lexer::TokenKind;

use super::*;

impl<'source> Parser<'source> {
    pub fn expression(&mut self) -> Result<ast::Expression, ()> {
        match self.current().kind {
            _ => self.parse_unary(),
        };

        todo!("expression");
    }

    pub fn parse_unary(&mut self) -> Result<ast::Expression, ()> {
        assert!(self.is_unary().is_some());

        Ok(match self.current().kind {
            TokenKind::Op_Minus => ast::Unary {
                op: ast::UnaryOperator::Minus,
                right: self.parse_unary()?,
            },
            _ => todo!("Syntax Error"),
        })
    }

    pub fn parse_primary_expression(&mut self) -> Result<ast::Expression<'source>, ()> {
        // TODO: Check recursion limit
    }
}
