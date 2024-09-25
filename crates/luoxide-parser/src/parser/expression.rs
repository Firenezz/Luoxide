use luoxide_ast::ast::{self, expressions::{Expression, Literal}};
use tracing::{event, Level};

use crate::{token::{Token, TokenKind}, token_set::TokenSet};

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

impl<'source> Parser<'source> {
    pub fn parse_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        event!(Level::INFO, "parsing expression");
        self.parse_primary_expression()
    }

    /// Prefix expression
    ///
    fn parse_suffixed_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        /*
            ```BNF
            suffixed_expression ::= primaryexp { '.' NAME | '[' exp ']' | ':' NAME funcargs | funcargs }
            ```
         */
        let current = self.current_token();

        let expr = self.parse_primary_expression();

        match current.kind() {

        }

        todo!()
    }

    /// Primary expression
    ///
    fn parse_primary_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        /*
            ```BNF
            primary_expression ::= nil | false | true | Numeral | LiteralString
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
            token!("(") => {
                let mut span = current.span;
                //self.parse_grouping_expression();
                //self.must
                span = span.merge(self.current().span);
                todo!()
            }
            _ => self.parse_suffixed_expression(),
        }
    }
}