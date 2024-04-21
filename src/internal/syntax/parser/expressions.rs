use expressions::ast::Unary;

use crate::{
    internal::syntax::{lexer::TokenKind, LineAnnotated},
    span::{Span, Spanned},
};

use self::ast::{Expression, ExpressionKind, Literal};

use super::*;

impl<'source> Parser<'source> {
    pub fn expression(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        match self.current().kind {
            TokenKind::Lit_Integer(_) | TokenKind::Lit_Float(_) | TokenKind::Lit_String(_) => {
                self.parse_literal()
            },
            TokenKind::Op_Minus | TokenKind::Kw_Not | TokenKind::Op_Len | TokenKind::Op_BitXor => self.parse_unary(),
            _ => unreachable!("expression - unexpected token"),
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
            _ => todo!("Postfix expression"), //return self.postfix_expr(),
        };
        self.bump();

        let expr = self.parse_primary_expression()?;

        Ok(Spanned::new(
            Span::new(start, self.previous().span.end),
            ExpressionKind::Unary(Box::new(Unary { op, right: expr })),
        ))
    }

    pub fn parse_binary(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        todo!("binary expression");
    }

    pub fn parse_literal(
        &mut self,
    ) -> Result<ast::Expression<'source>, LineAnnotated<SpannedError>> {
        let start = self.current().span.start;
        let expression = match self.current().kind {
            TokenKind::Lit_Integer(lit) => ExpressionKind::Literal(Box::new(Literal::Int(lit))),
            _ => todo!("parse literal"),
        };

        self.bump();

        Ok(Spanned::new(Span::new(start, self.previous().span.end), expression))
    }

    pub fn parse_primary_expression(
        &mut self,
    ) -> Result<ast::Expression<'source>, LineAnnotated<SpannedError>> {
        return self.parse_literal()
    }
}
