use expressions::ast::Unary;

use crate::{
    intern::StringInterner,
    internal::syntax::{lexer::TokenKind, LineAnnotated},
    span::{Span, Spanned},
};

use self::{
    ast::{BinaryOperator, ExpressionKind, Literal},
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

        let mut start_expr = match self.current().kind {
            // detect unary operators
            TokenKind::Op_Minus | TokenKind::Kw_Not | TokenKind::Op_Len | TokenKind::Op_BitXor => {
                let unary_operator = self.current().kind.clone(); // Clone should be fine because unary operators are smallish
                self.bump();
                let unary = self.parse_sub_expression(precedence::UNARY_PRIORITY)?;
                Spanned::new(
                    Span::new(unary.span.start, unary.span.end),
                    ExpressionKind::Unary(Box::new(Unary {
                        op: unary_operator.into(),
                        right: unary,
                    })),
                )
            }
            TokenKind::Brk_LeftParen => {
                self.bump();
                let expression = self.parse_sub_expression(0)?;
                if !self.test(TokenKind::Brk_RightParen) {
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
            if Precedence::from(operator).left < limit {
                break;
            }

            self.bump();

            let precedence = match Precedence::from(operator).get_associativity() {
                Associativity::Left => precedence + 1,
                Associativity::Right => precedence,
            };

            let right = self.parse_sub_expression(precedence);

            // Check recursion
            match right {
                Ok(right_expression) => {
                    start_expr = Spanned::new(
                        Span::new(start_expr.span.start, right_expression.span.end),
                        ExpressionKind::Binary(Box::new(ast::Binary {
                            left: start_expr,
                            operator,
                            right: right_expression,
                        })),
                    );
                }
                Err(_) => todo!("error"),
            }
        }

        Ok(start_expr)
    }

    #[inline]
    pub fn parse_simple_expression(
        &mut self,
    ) -> Result<ast::Expression, LineAnnotated<SpannedSyntaxError>> {
        let start = self.current().span.start;
        let expression = match self.current().kind {
            TokenKind::Lit_Integer(lit) => ExpressionKind::Literal(Box::new(Literal::Int(lit))),
            TokenKind::Lit_Float(lit) => ExpressionKind::Literal(Box::new(Literal::Float(lit))),
            // TODO: parse string literal correctly and handle escape sequences and interner
            TokenKind::Lit_String(ref lit) => {
                let parse_interned_string = Parser::<'source>::escape_unicode(lit.clone());
                ExpressionKind::Literal(Box::new(Literal::String(
                    self.state.interner.intern(parse_interned_string),
                )))
            }
            TokenKind::Lit_Bool(lit) => ExpressionKind::Literal(Box::new(Literal::Bool(lit))),
            TokenKind::Kw_Nil => ExpressionKind::Literal(Box::new(Literal::Nil)),
            // TODO: Check if we are in a vararg context (function)
            TokenKind::Op_Dots => ExpressionKind::Varargs,
            TokenKind::Brk_LeftCurly => todo!("table literal - table constructor"),
            TokenKind::Kw_Function => todo!("function literal - function constructor"),
            _ => todo!("suffixed expression"),
        };

        self.bump();

        Ok(Spanned::new(
            Span::new(start, self.previous().span.end),
            expression,
        ))
    }

    #[inline(always)]
    #[allow(dead_code)] // TODO: remove this after ast is finished
    fn parse_expression_list(
        &mut self,
    ) -> Result<Vec<ast::Expression>, LineAnnotated<SpannedSyntaxError>> {
        let mut expressions = vec![self.expression()?];

        while self.test(TokenKind::Tok_Comma) {
            self.bump();
            expressions.push(self.expression()?);
        }

        Ok(expressions)
    }
}
