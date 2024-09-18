use std::num::{IntErrorKind, ParseIntError};

use luoxide_ast::ast::{self, expressions::Literal};

use crate::{
    error::{ParseError, ParseErrorKind},
    token_set::TokenSet,
};

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
        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> Result<ast::expressions::Expression, ()> {
        // Assume current token is an identified literal or an unknown token

        let current = self.current_token();

        //assert!(!LITERAL_SET.contains(current.kind));
        if !LITERAL_SET.contains(current.kind) {
            self.unexpected_token(LITERALS);
            return Err(());
        }

        match current.kind {
            token!(nil) => Ok(Literal::create_nil(current.span)),
            token!(true) => Ok(Literal::create_bool(true, current.span)),
            token!(false) => Ok(Literal::create_bool(false, current.span)),
            token!(number) => Ok(Literal::create_number(
                match str::parse( self.lexer.lexeme(current)) {
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
                    match i64::from_str_radix(&self.lexer.lexeme(current)[2..], 16) {
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
                //self.parse_grouping_expression();
                self.must
                todo!()
            }
            _ => unreachable!("unreachable"),
        }
    }
}