use expressions::ast::Unary;

use crate::{
    internal::syntax::{lexer::TokenKind, LineAnnotated},
    span::{Span, Spanned},
};

use self::ast::{ExpressionKind, Literal};

use super::*;

impl<'source> Parser<'source> {
    pub fn parse_expression(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        self.expression()
    }

    /// Parse an expression
    ///
    /// This function assume the current token is the start of the expression.
    ///
    /// ```BNF
    /// expression = literal
    ///             | unary
    ///             | binary
    /// ```
    pub fn expression(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        // First we are at the start of the expression
        // Assume that the caller bump the lexer before calling this
        // this call comes from parse_statement or parse_expression
        match self.current().kind {
            // detect literals
            TokenKind::Lit_Integer(_) | TokenKind::Lit_Float(_) | TokenKind::Lit_String(_) => {
                self.parse_literal()?
            }
            // detect unary operators
            TokenKind::Op_Minus | TokenKind::Kw_Not | TokenKind::Op_Len | TokenKind::Op_BitXor => {
                self.parse_unary()?
            }

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

        let expr = self.parse_expression()?;

        Ok(Spanned::new(
            Span::new(start, self.previous().span.end),
            ExpressionKind::Unary(Box::new(Unary { op, right: expr })),
        ))
    }

    pub fn parse_binary(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        todo!("binary expression");
    }

    pub fn parse_literal(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedError>> {
        let start = self.current().span.start;
        let expression = match self.current().kind {
            TokenKind::Lit_Integer(lit) => ExpressionKind::Literal(Box::new(Literal::Int(lit))),
            TokenKind::Lit_Float(lit) => ExpressionKind::Literal(Box::new(Literal::Float(lit))),
            // TODO: parse string literal correctly and handle escape sequences and interner
            TokenKind::Lit_String(ref lit) => {
                ExpressionKind::Literal(Box::new(Literal::String(lit.clone())))
            }
            TokenKind::Lit_Bool(lit) => ExpressionKind::Literal(Box::new(Literal::Bool(lit))),
            TokenKind::Kw_Nil => ExpressionKind::Literal(Box::new(Literal::Nil)),
            _ => todo!("parse literal"),
        };

        self.bump();

        Ok(Spanned::new(
            Span::new(start, self.previous().span.end),
            expression,
        ))
    }
}
