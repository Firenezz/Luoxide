
use luoxide_ast::{ast::{
    self,
    expressions::{self, BinaryExpression, CallExpression, Expression, ExpressionKind, Literal, MemberExpression}, Identifier,
}, new::create_expression, operator::UnaryOperator};
use luoxide_text::{range::TextSpan, traits::Ranged};
use tracing::{event, Level};

use crate::{
    error::{ErrorKind, ParseError, ParseErrorKind}, parser::expression, token::{self, Token, TokenKind}, token_set::TokenSet
};

use super::{precedence::{self, Associativity, Precedence}, Parser};
use crate::error::Result;

const LITERALS: [crate::token::TokenKind; 10] = [
    token!(nil),
    token!(true),
    token!(false),
    token!(number),
    token!(hex_number),
    token!(float),
    token!(hex_float),
    token!(NaN),
    token!(string),
    token!(multiline_string),
];

const LITERAL_SET: TokenSet = TokenSet::new(LITERALS);

// Parsing of expression used in general
impl Parser<'_> {
    /// Parses a list of function expression
    ///
    /// ```BNF
    ///     args ::= expression { }
    /// ```
    ///
    ///
    pub fn parse_expressionlist(&mut self) -> Vec<()> {
        todo!()
    }

    pub fn parse_statlist(&mut self) -> Vec<()> {
        todo!()
    }

    /// Parses a list of function arguments
    ///
    /// ```BNF
    ///     args ::= '(' [explist] ')' | table_constructor | LiteralString
    /// ```
    ///
    ///
    pub fn parse_args(&mut self) -> Vec<ast::expressions::Expression> {
        /*
           ```BNF
               args
           ```
        */

        todo!()
    }

    pub fn parse_field_selector(&mut self) -> ast::expressions::Expression {
        let current = self.current_token();

        /*match current.kind() {

        }*/

        todo!()
    }

    /// Parse a Field
    ///
    /// ```BNF
    ///     field ::= '[' expression ']' '=' expression | Name '=' expression | expression
    /// ```
    pub fn parse_field(&mut self) -> Result<ast::Field> {
        // No assert. Expression part of this parse will handle it
        todo!()
        /*Ok(match self.current_token().kind {
            token!("[") => {
                self.bump();
                self.parse_expression();
            }
            _ => {
                let value = self.parse_expression()?;
                Field::new(None, value)
            }
        })*/
    }

    pub fn parse_field_list(&mut self) -> Result<ast::expressions::Expression> {
        /*const FIELD_SEPERATOR: [TokenKind; 2] = [token!(","), token!(";")];
        const FIELD_SEPERATOR_SET: TokenSet = TokenSet::new(FIELD_SEPERATOR);

        debug_assert!(self.current_is(token!("{")));

        let mut fields: Vec<Field> = vec![];

        fn parse_field(parser: &mut Parser<'_>) -> Result<ast::expressions::Expression> {
            let span = parser.current_token().span;

            match parser.current_token().kind {
                _ => todo!(),
            }
        }

        parse_field(self);*/
        todo!()
    }

    pub fn parse_table_constructor(&mut self) -> Result<ast::expressions::Expression> {
        todo!()
    }
}

// TODO: Use a bump allocator
impl<'source> Parser<'source> {

    /*
    fn parse_prefix_expression(&mut self) -> Result<ast::expressions::Expression> {
        /*
           prefix_expression ::=
        */
        todo!()
    }

    /// Prefix expression
    ///
    fn parse_primary_expression(&mut self) -> Result<ast::expressions::Expression> {
        /*
           ```BNF
           primary_expression ::= primaryexp { '.' NAME | '[' exp ']' | ':' NAME funcargs | funcargs }
           ```
        */
        let current = self.current_token();

        let expr = self.parse_simple_expression();

        todo!()
    }

    */

    fn parse_primary_expression(&mut self) -> Result<ast::expressions::Expression> {
        /*
           ```BNF
           primary_expression ::= NAME | '(' arglist ')'
           ```
        */

        let start = self.current().span.start;

        match self.current().kind {
            token!(identifier) => {
                let name = self.get_lexeme(self.current());
                let end = self.current().span.end;
                Ok(create_expression(ExpressionKind::Identifier(Identifier::new(name.to_string())), (start..end).into()))
            }
            // Function call ::= '(' arglist ')'
            token!("(") => {

                let arglist = self.parse_arg_list()?;
                self.expect(token!(")"));

                todo!();
                //expr
            }
            // Lexing error
            token!(Error) => todo!(),
            _ => {
                let current = *self.current_token();
                //Err(self.unexpected_token([token!(identifier), token!("(")], current.kind(), Some(current.span)))
                todo!()
            },
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expression>> {
        // Assume starting token is consumed
        let expr = match self.current_kind() {
            token!("(") => {
                self.bump();
                match self.current_kind() {
                    token!(")") => return Ok(vec![]),
                    _ => {
                        let expression = self.parse_expression()?;
                        self.series_of(&Self::parse_expression, token!(","));
                        //self.check(token!(")"));
                    }
                }
                vec![]
            }
            //token!("{") => vec![self.constructor()?],
            token!(string) => {
                todo!();
                //vec![expr]
            }
            _ => todo!()//return Err(Error::SyntaxError("function arguments expected".to_string()))
        };
        Ok(expr)
    }

    fn parse_suffixed_expression(&mut self) -> Result<Box<ast::expressions::Expression>> {
        /*
        ```BNF
           suffixed_expression ::= primary_expression { '.' NAME | '[' exp `]' | ':' NAME funcargs | funcargs }
           ```
         */

        let start = self.current().span.start;
        let mut expression_tree: Box<Expression> = self.allocate(self.parse_primary_expression()?);

        loop {
            match self.next_token_kind() {
                token!(".") => {
                    self.bump(); // consume the dot
                    
                    if self.next_token().is(token!(identifier)) {
                        let name = self.get_lexeme(self.next_token());
                        let ident = self.allocate(create_expression(ExpressionKind::Identifier(Identifier::new(name.to_string())), self.next_token().span));
                        
                        expression_tree = self.allocate(MemberExpression::create(expression_tree, ident, (start..self.next_token().span.end).into()));
                        self.bump();
                    } else {
                        panic!();
                    }
                },
                token!("[") => todo!(),
                token!(":") => todo!(),
                token!("(") => {
                    self.bump();
                    let args = vec![];
                    if !self.next_token().is(token!(")")) {
                        self.parse_arg_list()?;
                        self.expect(token!(")"));
                    }
                    self.bump();
                    expression_tree = self.allocate(CallExpression::create(expression_tree, args, (start..self.next_token().span.end).into()));
                },
                _ => break,
            }
        }

        Ok(expression_tree)
    }

    /// Primary expression
    ///
    fn parse_simple_expression(&mut self) -> Result<Box<Expression>> {
        /*
           ```BNF
           simple_expression ::= nil | true | false | Numeral | float | LiteralString | functiondef
               | table_constructor | primary_expression
           ```
        */
        // Assume current token is an identified literal or an unknown token

        const NUMBER_PLACEHOLDER: i64 = 0;

        event!(Level::INFO, "parsing primary expression");

        let current = *self.current();

        match current.kind() {
            token!(nil) => Ok(Literal::create_nil(current.span)),
            token!(true) => Ok(Literal::create_bool(true, current.span)),
            token!(false) => Ok(Literal::create_bool(false, current.span)),
            token!(number) => Ok(Literal::create_number(
                match str::parse(self.get_lexeme(&current)) {
                    Ok(number) => number,
                    Err(err) => {
                        let error = self.int_parse_error(err, Some(current.span));

                        self.synchronize_expression();
                        NUMBER_PLACEHOLDER
                    }
                },
                current.span,
            )),
            token!(hex_number) => Ok(Literal::create_number(
                {
                    match i64::from_str_radix(&self.get_lexeme(&current)[2..], 16) {
                        Ok(number) => number,
                        Err(err) => {
                            self.int_parse_error(err, Some(current.span));
                            NUMBER_PLACEHOLDER
                        }
                    }
                },
                current.span,
            )),
            token!(string) => {
                // strings from lexer needs to be escaped
                todo!("strings");
            }
            token!("...") => todo!("varargs"),
            token!(function) => todo!("function"),
            token!(EOF) => {
                let error = self.unexpected_eof(Some(current.span));
                self.error_context.add_error(error);
                todo!();
                Err(error)
            }
            _ => self.parse_suffixed_expression(),
        }
    }

    pub(crate) fn parse_expression(&mut self) -> Result<Expression> {
        event!(Level::INFO, "parsing expression");
        self.parse_sub_expression(0)
    }

    pub fn parse_sub_expression(
        &mut self,
        limit: u8,
    ) -> Result<Expression> {
        // First we are at the start of the expression
        // Assume that the caller bump the lexer before calling this
        // this call comes from parse_statement or parse_expression

        let start = self.current().span.start;

        let mut start_expression = match self.current().kind {
            // detect unary operators
            TokenKind::Minus | TokenKind::Not | TokenKind::Pound | TokenKind::Tilde => {
                let unary_operator = self.current().kind.clone();
                self.bump();
                let unary = self.parse_sub_expression(precedence::UNARY_PRIORITY)?;
                let span = TextSpan::new(start, self.current().span.end);
                create_expression(ExpressionKind::UnaryOperator(Box::new(expressions::UnaryExpression{operator: unary_operator.to_unary_op().unwrap(), operand: unary})), span)
                
            }
            TokenKind::LeftParen => {
                self.bump();
                let expression = self.parse_sub_expression(0)?;
                if self.expect(TokenKind::RightParen).is_none() {
                    return Err(self.unexpected_token([token!(")")], self.current_kind(), Some(self.next_token().span)));
                }
                expression
            }
            _ => self.parse_simple_expression()?,
        };

        while let Some(operator) =
            self.current().kind.to_binary_op()
        {
            let precedence = Precedence::from_binary_operator(&operator).left;
            if precedence < limit {
                break;
            }

            self.bump();

            let precedence = match Precedence::from_binary_operator(&operator).get_associativity() {
                Associativity::Left => precedence + 1,
                Associativity::Right => precedence,
            };

            let right = self.parse_sub_expression(precedence);

            // TODO: Check recursion
            match right {
                Ok(right_expression) => {
                    let span = TextSpan::new(start, self.current().span.end); // TODO: Use merge function
                    start_expression = create_expression(ExpressionKind::BinaryOperator(Box::new(BinaryExpression {left: start_expression, right: right_expression, operator})), span);
                }
                Err(_) => todo!("error"),
            }
        }

        Ok(start_expression)
    }
}
