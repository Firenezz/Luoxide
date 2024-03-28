use crate::{internal::syntax::{lexer::TokenKind, LineAnnotated}, span::{Span, Spanned}};

use super::*;

impl<'source> Parser<'source> {
    pub fn expression(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        match self.current().kind {
            _ => self.parse_unary(),
        };

        todo!("expression");
    }

    pub fn parse_unary(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        assert!(self.is_unary().is_some());

        let start = self.current().span.start;

        let op = match self.current().kind {
            TokenKind::Op_Minus => ast::UnaryOperator::Minus,
            TokenKind::Kw_Not => ast::UnaryOperator::Not,
            TokenKind::Op_Len => ast::UnaryOperator::Lenght,
            TokenKind::Op_BitXor => ast::UnaryOperator::BitNot,
            _ => todo!("Postfix expression")//return self.postfix_expr(),
          };
        self.bump();

        let start = self.current().span.start;
        let expr = self.parse_primary_expression()?;

        
    }

    pub fn parse_primary_expression(&mut self) -> Result<ast::Expression<'source>, LineAnnotated<SpannedError>> {
        
    }
}
