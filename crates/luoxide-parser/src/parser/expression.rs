//! Expression parsing: a Pratt / precedence-climbing core with dedicated
//! routines for primary, suffixed (`a.b`, `a[b]`, `a:m()`, `f()`) and simple
//! expressions.

use crate::ast::{self, BinaryOp, Expression, FunctionBody, Literal, NodeList, UnaryOp};
use crate::error::Result;
use crate::token::{Token, TokenKind};

use super::Parser;
use super::strings;

/// Maps a token to the binary operator it introduces, if any.
const fn binary_op(kind: TokenKind) -> Option<BinaryOp> {
    Some(match kind {
        token!("+") => BinaryOp::Add,
        token!("-") => BinaryOp::Sub,
        token!("*") => BinaryOp::Mul,
        token!("/") => BinaryOp::Div,
        token!("//") => BinaryOp::IDiv,
        token!("%") => BinaryOp::Mod,
        token!("^") => BinaryOp::Pow,
        token!("..") => BinaryOp::Concat,
        token!("==") => BinaryOp::Eq,
        token!("~=") => BinaryOp::NotEq,
        token!("<") => BinaryOp::Less,
        token!("<=") => BinaryOp::LessEq,
        token!(">") => BinaryOp::Greater,
        token!(">=") => BinaryOp::GreaterEq,
        token!(and) => BinaryOp::And,
        token!(or) => BinaryOp::Or,
        token!("&") => BinaryOp::BitAnd,
        token!("|") => BinaryOp::BitOr,
        token!("~") => BinaryOp::BitXor,
        token!("<<") => BinaryOp::Shl,
        token!(">>") => BinaryOp::Shr,
        _ => return None,
    })
}

/// Maps a token to the unary operator it introduces, if any.
const fn unary_op(kind: TokenKind) -> Option<UnaryOp> {
    Some(match kind {
        token!("-") => UnaryOp::Neg,
        token!(not) => UnaryOp::Not,
        token!("#") => UnaryOp::Len,
        token!("~") => UnaryOp::BitNot,
        _ => return None,
    })
}

impl<'source> Parser<'source> {
    /// Parses a full expression.
    ///
    /// ```BNF
    /// expression ::= simple_expression | expression binop expression | unop expression
    /// ```
    pub fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_sub_expression(0)
    }

    /// Precedence-climbing loop, mirroring `subexpr` in the reference Lua
    /// parser: parse a unary or simple operand, then keep consuming binary
    /// operators that bind tighter than `limit`.
    fn parse_sub_expression(&mut self, limit: u8) -> Result<Expression> {
        let at = self.current_token().span;
        self.with_depth("expression", at, |parser| {
            let lhs = match unary_op(parser.current_token().kind) {
                Some(op) => {
                    let op_token = *parser.current_token();
                    parser.bump();
                    let operand = parser.parse_sub_expression(UnaryOp::BINDING_POWER)?;
                    let span = op_token.span.merge(operand.span);
                    Expression::unary(op, operand, span)
                }
                None => parser.parse_simple_expression()?,
            };

            parser.parse_binary_rest(lhs, limit)
        })
    }

    /// The binary-operator half of [`parse_sub_expression`]: continues an
    /// expression whose left-hand side is already parsed. Used directly when
    /// a caller had to consume a token before knowing it starts an expression
    /// (e.g. table fields).
    pub(super) fn parse_binary_rest(
        &mut self,
        mut lhs: Expression,
        limit: u8,
    ) -> Result<Expression> {
        while let Some(op) = binary_op(self.current_token().kind) {
            let (left_power, right_power) = op.binding_power();
            if left_power <= limit {
                break;
            }
            self.bump();
            let rhs = self.parse_sub_expression(right_power)?;
            lhs = Expression::binary(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// ```BNF
    /// simple_expression ::= nil | true | false | Numeral | LiteralString | '...'
    ///     | functiondef | table_constructor | suffixed_expression
    /// ```
    fn parse_simple_expression(&mut self) -> Result<Expression> {
        let current = *self.current_token();

        Ok(match current.kind {
            token!(nil) => {
                self.bump();
                Expression::literal(Literal::Nil, current.span)
            }
            token!(true) => {
                self.bump();
                Expression::literal(Literal::Bool(true), current.span)
            }
            token!(false) => {
                self.bump();
                Expression::literal(Literal::Bool(false), current.span)
            }
            token!(number) => {
                self.bump();
                let value = self.parse_int_literal(&current);
                Expression::literal(Literal::Int(value), current.span)
            }
            token!(hex_number) => {
                self.bump();
                let value = self.parse_hex_literal(&current);
                Expression::literal(Literal::Int(value), current.span)
            }
            token!(float) => {
                self.bump();
                let value = self.parse_float_literal(&current);
                Expression::literal(Literal::Float(value), current.span)
            }
            token!(hex_float) => {
                self.bump();
                let value = self.parse_hex_float_literal(&current);
                Expression::literal(Literal::Float(value), current.span)
            }
            token!(NaN) => {
                self.bump();
                Expression::literal(Literal::Float(f64::NAN), current.span)
            }
            token!(string) | token!(multiline_string) => self.parse_string_literal(),
            token!("...") => {
                self.bump();
                Expression::varargs(current.span)
            }
            token!("{") => self.parse_table_constructor()?,
            token!(function) => {
                self.bump();
                let (body, span) = self.parse_function_body(current.span)?;
                Expression::function(body, span)
            }
            token!(EOF) => return Err(self.unexpected_eof(Some(current.span))),
            token!(Error) => {
                self.bump();
                return Err(self.lexer_error(Some(current.span)));
            }
            _ => return self.parse_suffixed_expression(),
        })
    }

    /// ```BNF
    /// suffixed_expression ::= primary_expression
    ///     { '.' Name | '[' expression ']' | ':' Name call_args | call_args }
    /// ```
    pub(super) fn parse_suffixed_expression(&mut self) -> Result<Expression> {
        let expression = self.parse_primary_expression()?;
        self.parse_suffixed_rest(expression)
    }

    /// The suffix loop of [`parse_suffixed_expression`], continuing from an
    /// already-parsed primary expression.
    pub(super) fn parse_suffixed_rest(&mut self, primary: Expression) -> Result<Expression> {
        let mut expression = primary;

        loop {
            expression = match self.current_token().kind {
                // Member access: `a.b`
                token!(".") => {
                    self.bump();
                    let name = self.require_identifier()?;
                    Expression::member(expression, name)
                }
                // Indexing: `a[b]`
                token!("[") => {
                    self.bump();
                    let index = self.parse_expression()?;
                    self.expect(token!("]"));
                    let span = expression.span.merge(self.previous_span());
                    Expression::index(expression, index, span)
                }
                // Method call: `a:m(...)`
                token!(":") => {
                    self.bump();
                    let name = self.require_identifier()?;
                    if !Self::starts_call_args(&self.current_token().kind) {
                        let current = *self.current_token();
                        let error = self.unexpected_token(
                            [token!("("), token!("{"), token!(string)],
                            &current.kind,
                            Some(current.span),
                        );
                        self.record_error(error);
                        let span = expression.span.merge(name.span);
                        return Ok(Expression::method_call(
                            expression,
                            name,
                            NodeList::new(),
                            span,
                        ));
                    }
                    let args = self.parse_call_args()?;
                    let span = expression.span.merge(self.previous_span());
                    Expression::method_call(expression, name, args, span)
                }
                // Call: `f(...)`, `f{...}`, `f"..."`
                kind if Self::starts_call_args(&kind) => {
                    let args = self.parse_call_args()?;
                    let span = expression.span.merge(self.previous_span());
                    Expression::call(expression, args, span)
                }
                _ => break,
            };
        }

        Ok(expression)
    }

    /// ```BNF
    /// primary_expression ::= Name | '(' expression ')'
    /// ```
    fn parse_primary_expression(&mut self) -> Result<Expression> {
        let current = *self.current_token();

        match current.kind {
            _ if current.kind.is_name() => {
                let identifier = self.maybe_identifier().expect("current token is a name");
                Ok(Expression::identifier(identifier))
            }
            token!("(") => {
                self.bump();
                let inner = self.parse_expression()?;
                self.expect(token!(")"));
                let span = current.span.merge(self.previous_span());
                Ok(Expression::grouped(inner, span))
            }
            token!(EOF) => Err(self.unexpected_eof(Some(current.span))),
            token!(Error) => {
                self.bump();
                Err(self.lexer_error(Some(current.span)))
            }
            _ => Err(self.unexpected_token(
                [token!(identifier), token!("(")],
                &current.kind,
                Some(current.span),
            )),
        }
    }

    const fn starts_call_args(kind: &TokenKind) -> bool {
        matches!(
            kind,
            token!("(") | token!("{") | token!(string) | token!(multiline_string)
        )
    }

    /// ```BNF
    /// call_args ::= '(' [expression {',' expression}] ')' | table_constructor |
    /// ```
    fn parse_call_args(&mut self) -> Result<NodeList<Expression>> {
        match self.current_token().kind {
            token!("(") => {
                self.bump();
                let mut args = NodeList::new();
                if self.current_is_not(token!(")")) {
                    loop {
                        // One bad argument doesn't discard the whole call:
                        // record the error, skip to the next anchor (`,` or
                        // `)`) and keep an error placeholder in its place.
                        match self.parse_expression() {
                            Ok(arg) => args.push(arg),
                            Err(error) => {
                                let start = self.current_token().span;
                                self.recover_expression(error);
                                args.push(Expression::error(
                                    start.merge(self.current_token().span),
                                ));
                            }
                        }
                        if self.maybe(token!(",")).is_none() {
                            break;
                        }
                    }
                }
                self.expect(token!(")"));
                Ok(args)
            }
            token!("{") => {
                let table = self.parse_table_constructor()?;
                let mut args = NodeList::new();
                args.push(table);
                Ok(args)
            }
            token!(string) | token!(multiline_string) => {
                let literal = self.parse_string_literal();
                let mut args = NodeList::new();
                args.push(literal);
                Ok(args)
            }
            _ => unreachable!("caller checked starts_call_args"),
        }
    }

    /// ```BNF
    /// functionbody ::= '(' [parlist] ')' block end
    /// parlist ::= namelist [',' varargparam] | varargparam
    /// varargparam ::= '...' [Name]
    /// ```
    ///
    /// `start` is the span of the `function` keyword (or the whole
    /// `function a.b:c` prefix for declarations); the returned span covers the
    /// body up to and including `end`.
    pub(super) fn parse_function_body(
        &mut self,
        start: luoxide_text::range::TextSpan,
    ) -> Result<(FunctionBody, luoxide_text::range::TextSpan)> {
        self.with_frame("function_body", |parser| {
            parser.parse_function_body_inner(start)
        })
    }

    fn parse_function_body_inner(
        &mut self,
        start: luoxide_text::range::TextSpan,
    ) -> Result<(FunctionBody, luoxide_text::range::TextSpan)> {
        self.expect(token!("("));
        let params = self.parse_parlist()?;
        self.expect(token!(")"));

        let body = self.parse_block();
        self.expect(token!(end));

        let span = start.merge(self.previous_span());
        Ok((FunctionBody { params, body }, span))
    }

    /// ```BNF
    /// parlist ::= namelist [',' varargparam] | varargparam
    /// varargparam ::= '...' [Name]
    /// ```
    fn parse_parlist(&mut self) -> Result<NodeList<ast::Param>> {
        let mut params = NodeList::new();
        if self.current_is(token!(")")) {
            return Ok(params);
        }
        loop {
            if let Some(varargs) = self.parse_vararg_param() {
                params.push(ast::Param::Varargs(varargs));
                return Ok(params);
            }
            params.push(ast::Param::Name(self.require_identifier()?));
            if self.maybe(token!(",")).is_none() {
                return Ok(params);
            }
        }
    }

    fn parse_vararg_param(&mut self) -> Option<ast::VarargsParam> {
        self.maybe(token!("..."))?;
        Some(ast::VarargsParam {
            name: self.maybe_identifier(),
        })
    }

    /// Parses a short or long string literal. Invalid escape sequences are
    /// reported as recoverable errors; the literal keeps the valid prefix.
    pub(super) fn parse_string_literal(&mut self) -> Expression {
        let token = *self.current_token();
        debug_assert!(token.is_string());
        self.bump();

        let raw = self.get_lexeme(&token);
        let content = match token.kind {
            token!(string) => {
                let (content, bad_escape) = strings::unescape_short(raw);
                if let Some(offset) = bad_escape {
                    let at = luoxide_text::range::TextSpan::at(
                        token.span.start + luoxide_text::size::TextSize::new(offset as u32),
                        luoxide_text::size::TextSize::new(2),
                    );
                    let error = self.invalid_escape(Some(at));
                    self.record_error(error);
                }
                content
            }
            _ => strings::unescape_long(raw),
        };

        Expression::literal(Literal::String(content), token.span)
    }
}

// Number literal parsing. Failures are recoverable: the error is recorded and
// a placeholder value keeps the AST intact.
impl Parser<'_> {
    const INT_PLACEHOLDER: i64 = 0;
    const FLOAT_PLACEHOLDER: f64 = 0.0;

    fn parse_int_literal(&mut self, token: &Token) -> i64 {
        let lexeme = self.get_lexeme(token);
        let cleaned;
        let digits = if lexeme.contains('_') {
            cleaned = lexeme.replace('_', "");
            cleaned.as_str()
        } else {
            lexeme
        };

        match digits.parse::<i64>() {
            Ok(value) => value,
            Err(error) => {
                let error = self.int_parse_error(error, Some(token.span));
                self.record_error(error);
                Self::INT_PLACEHOLDER
            }
        }
    }

    fn parse_hex_literal(&mut self, token: &Token) -> i64 {
        let lexeme = self.get_lexeme(token);
        let cleaned;
        let digits = if lexeme.contains('_') {
            cleaned = lexeme.replace('_', "");
            cleaned.as_str()
        } else {
            lexeme
        };
        let digits = &digits[2..]; // strip `0x`

        // Lua hex literals wrap around on overflow instead of erroring,
        // so `0xFFFFFFFFFFFFFFFF` is `-1`.
        match i64::from_str_radix(digits, 16) {
            Ok(value) => value,
            Err(_) => match u64::from_str_radix(digits, 16) {
                Ok(value) => value as i64,
                Err(error) => {
                    let error = self.int_parse_error(error, Some(token.span));
                    self.record_error(error);
                    Self::INT_PLACEHOLDER
                }
            },
        }
    }

    fn parse_float_literal(&mut self, token: &Token) -> f64 {
        let lexeme = self.get_lexeme(token);
        match lexeme.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                let error = self.malformed_number(Some(token.span));
                self.record_error(error);
                Self::FLOAT_PLACEHOLDER
            }
        }
    }

    fn parse_hex_float_literal(&mut self, token: &Token) -> f64 {
        let lexeme = self.get_lexeme(token);
        match lexeme.parse::<hexfloat2::HexFloat64>() {
            Ok(value) => value.into(),
            Err(_) => {
                let error = self.malformed_number(Some(token.span));
                self.record_error(error);
                Self::FLOAT_PLACEHOLDER
            }
        }
    }
}
