use std::{borrow::Borrow, thread::Scope};

use ast::Statement;
use statement::ast::Assignment;

use crate::{
    intern::StringInterner,
    internal::syntax::LineAnnotated,
    span::{Span, Spanned},
};

use self::ast::Identifier;

use super::*;

impl<'source> Parser<'source> {
    pub fn statement(&mut self) -> Result<ast::Statement, SpannedSyntaxError> {
        //self.simple_stmt()
        todo!("statement");
    }

    /// Check if the current token is a statement anchor
    ///
    /// This function returns true if the token indicates a possible statement start
    /// This function is used to synchronize the parser after a ParserError
    pub fn statement_anchor_token(&mut self, token: &TokenKind) -> bool {
        // First flow control keywords and keywords are a good start
        match token {
            TokenKind::Break
            | TokenKind::Do
            | TokenKind::Goto
            | TokenKind::If
            | TokenKind::ElseIf
            | TokenKind::Else => return true,
            TokenKind::End
            | TokenKind::While
            | TokenKind::For
            | TokenKind::Function
            | TokenKind::Local
            | TokenKind::Return => return true,
            TokenKind::SemiColon => return true,
            _ => (),
        };
        false
        // TODO: add assignment operators and labels
    }

    pub(crate) fn parse_statement_assignment(
        &mut self,
        local: ScopeKind,
    ) -> Result<ast::Statement, LineAnnotated<SpannedSyntaxError>> {
        // Start assigment parsing
        let start_index = self.previous().span.start;

        let variable_list = self
            .one_or_more(
                match local {
                    ScopeKind::Global => |parser: &mut Parser| parser.parse_target::<false>(),
                    ScopeKind::Local => |parser: &mut Parser| parser.parse_target::<true>(),
                },
                token!(","),
            )
            .into_result()
            .unwrap();

        match (self.expect_current(token!("=")), local) {
            (Fail, ScopeKind::Global) => todo!("Add error"),
            (Fail, ScopeKind::Local) => todo!("Add local assignment without initializer"),
            (Success(_), _) => self.advance(),
        }
        let expression_list = self.parse_expression_list().unwrap();

        let end_index = self.previous().span.end;
        Ok(Statement {
            value: ast::StatementKind::Assignment(Assignment {
                name: variable_list,
                init: expression_list,
            }),
            location: None,
        })
    }

    pub(crate) fn parse_target<const LOCAL: bool>(&mut self) -> ParseResult<ast::Expression> {
        match (self.current().kind.clone(), LOCAL) {
            (token!(lit_identifier, ident), false) => {
                self.advance();
                Success(Identifier::new_identifier(self.current().span, ident))
            }
            (token!(lit_identifier, ident), true) => {
                self.advance();
                match self.expect_current(token!(",")) {
                    Fail => {
                        todo!("Success")
                    }
                    Success(_) => Success(Identifier::new_identifier(self.current().span, ident)),
                }
            }
            _ => todo!("Add error"),
        }
    }

    /// Parse a variable list
    ///
    /// ```BNF
    /// variable_list ::= variable {',' variable}
    /// ```
    pub(crate) fn parse_variable_list(
        &mut self,
    ) -> Result<Vec<ast::Identifier>, SpannedSyntaxError> {
        todo!()
    }
}
