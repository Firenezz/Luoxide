use crate::internal::syntax::{lexer::TokenKind, LineAnnotated};

use self::{
    ast::BinaryOperator,
    precedence::{Associativity, Precedence},
};

use super::*;

impl<'source> Parser<'source> {
    pub fn parse_expression(
        &mut self,
    ) -> Result<ast::Expression, LineAnnotated<SpannedSyntaxError>> {
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
    pub fn expression(&mut self) -> Result<ast::Expression, LineAnnotated<SpannedSyntaxError>> {
        self.parse_sub_expression(0)
    }

    pub fn parse_sub_expression(
        &mut self,
        limit: u8,
    ) -> Result<ast::Expression, LineAnnotated<SpannedSyntaxError>> {
        // First we are at the start of the expression
        // Assume that the caller bump the lexer before calling this
        // this call comes from parse_statement or parse_expression

        let start = self.current().span.start;

        let mut start_expression = match self.current().kind {
            // detect unary operators
            TokenKind::Minus | TokenKind::Not | TokenKind::Pound | TokenKind::BitXor => {
                let unary_operator = self.current().kind.clone();
                self.advance();
                let unary = self.parse_sub_expression(precedence::UNARY_PRIORITY)?;
                ast::Unary::new(
                    unary.span.start..unary.span.end,
                    unary,
                    unary_operator.into(),
                )
            }
            TokenKind::LeftParen => {
                self.advance();
                let expression = self.parse_sub_expression(0)?;
                self.must(TokenKind::RightParen);
                if !self.expect_current(TokenKind::RightParen).is_success() {
                    return Err(self.unexpected_token(self.current()));
                }
                expression
            }
            _ => self.parse_simple_expression()?,
        };

        while let Ok(operator) =
            std::convert::TryInto::<BinaryOperator>::try_into(self.current().kind.clone())
        {
            let precedence = Precedence::from(operator).left;
            if precedence < limit {
                break;
            }

            self.advance();

            let precedence = match Precedence::from(operator).get_associativity() {
                Associativity::Left => precedence + 1,
                Associativity::Right => precedence,
            };

            let right = self.parse_sub_expression(precedence);

            // TODO: Check recursion
            match right {
                Ok(right_expression) => {
                    start_expression = ast::Binary::new(
                        start..right_expression.span.end,
                        start_expression,
                        right_expression,
                        operator,
                    );
                }
                Err(_) => todo!("error"),
            }
        }

        Ok(start_expression)
    }

    #[allow(dead_code)]
    /// Parse a prefix expression
    ///
    /// ```BNF
    /// prefix_expression = var | functioncall | ‘(’ exp ‘)’
    /// ```
    pub(crate) fn parse_prefix_expression(
        &mut self,
    ) -> Result<ast::Expression, LineAnnotated<SpannedSyntaxError>> {
        todo!("parse_prefix_expression")
    }

    #[inline]
    pub fn parse_simple_expression(
        &mut self,
    ) -> Result<ast::Expression, LineAnnotated<SpannedSyntaxError>> {
        let expression = match self.current().kind {
            TokenKind::Lit_Integer(literal) => ast::Literal::new_int(self.current().span, literal),
            TokenKind::Lit_Float(literal) => ast::Literal::new_float(self.current().span, literal),
            // TODO: parse string literal correctly and handle escape sequences and interner
            TokenKind::Lit_String(ref literal) => {
                ast::Literal::new_string(self.current().span, literal.clone())
            }
            TokenKind::Lit_Bool(literal) => ast::Literal::new_bool(self.current().span, literal),
            TokenKind::Nil => ast::Literal::new_nil(self.current().span),
            // TODO: Check if we are in a vararg context (function)
            TokenKind::Dots => todo!("vararg"),
            TokenKind::LeftCurly => todo!("table literal - table constructor"),
            TokenKind::Function => todo!("function literal - function constructor"),
            _ => panic!(
                "Handle in syntax errors - Unexpected token: {:?}",
                self.current()
            ),
        };

        self.advance();

        Ok(expression)
    }

    #[inline(always)]
    #[allow(dead_code)] // TODO: remove this after ast is finished
    pub(crate) fn parse_expression_list(
        &mut self,
    ) -> Result<Vec<ast::Expression>, LineAnnotated<SpannedSyntaxError>> {
        let mut expressions = vec![self.expression()?];

        while self.expect_current(TokenKind::Comma).is_success() {
            self.advance();
            expressions.push(self.expression()?);
            self.advance();
        }

        Ok(expressions)
    }
}
