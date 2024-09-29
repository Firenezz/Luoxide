use luoxide_ast::ast::{self, expressions::{Expression, Literal}};
use tracing::{event, Level};

use crate::{token::{self, Token, TokenKind}, token_set::TokenSet};

use super::Parser;

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
        Ok(match self.current_token().kind {
            token!("[") => {
                self.bump();
            }
            _ => {
                let value = self.parse_expression()?;
                Field::new(None, value)
            }
        })
    }


    pub fn parse_field_list(&mut self) -> Result<ast::expressions::Expression> {
        const FIELD_SEPERATOR: [TokenKind; 2] = [token!(","), token!(";")];
        const FIELD_SEPERATOR_SET: TokenSet = TokenSet::new(FIELD_SEPERATOR);

        debug_assert!(self.current_is(token!("{")));

        let mut fields: Vec<Field> = vec![];

        fn parse_field(parser: &mut Parser<'_>) -> Result<ast::expressions::Expression> {
            

            let span = parser.current_token().span;

            match parser.current_token().kind {
                
                _ => todo!()
            }

        }

        parse_field(self);
    }

    pub fn parse_table_constructor(&mut self) -> Result<ast::expressions::Expression> {

    }
}

impl<'source> Parser<'source> {
    pub fn parse_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        event!(Level::INFO, "parsing expression");
        self.parse_simple_expression()
    }

    fn parse_prefix_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        /*
            prefix_expression ::= 
         */
    }

    /// Prefix expression
    ///
    fn parse_primary_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        /*
            ```BNF
            primary_expression ::= primaryexp { '.' NAME | '[' exp ']' | ':' NAME funcargs | funcargs }
            ```
         */
        let current = self.current_token();

        let expr = self.parse_simple_expression();

        todo!()
    }

    /// Primary expression
    ///
    fn parse_simple_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        /*
            ```BNF
            simple_expression ::= nil | true | false | Numeral | float | LiteralString | functiondef
                | table_constructor | primary_expression
            ```
         */
        // Assume current token is an identified literal or an unknown token

        event!(Level::INFO, "parsing primary expression");

        let current = self.current_token();

        if !LITERAL_SET.contains(current.kind) {
            event!(Level::ERROR, "unexpected_token");
            self.unexpected_token(LITERALS);
            return Err(());
        }

        match current.kind() {
            token!(nil) => Ok(Literal::create_nil(current.span)),
            token!(true) => Ok(Literal::create_bool(true, current.span)),
            token!(false) => Ok(Literal::create_bool(false, current.span)),
            token!(number) => Ok(Literal::create_number(
                match str::parse(self.get_lexeme(current)) {
                    Ok(number) => number,
                    Err(err) => {
                        self.int_parse_error(err);
                        return Err(());
                    }
                },
                current.span,
            )),
            token!(hex_number) => Ok(Literal::create_number(
                {
                    match i64::from_str_radix(&self.get_lexeme(current)[2..], 16) {
                        Ok(number) => number,
                        Err(err) => {
                            self.int_parse_error(err);
                            return Err(());
                        }
                    }
                },
                current.span,
            )),
            token!(string) => {
                // strings from lexer needs to be escaped
                todo!("strings");
            }
            token!("(") => {
                let mut span = current.span;
                //self.parse_grouping_expression();
                //self.must
                span = span.merge(self.current().span);
                todo!()
            }
            _ => self.parse_primary_expression(),
        }
    }
}