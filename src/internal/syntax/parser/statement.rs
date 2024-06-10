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
    ) -> Result<ast::Statement, LineAnnotated<SpannedSyntaxError>> {
        // Start assigment parsing
        let start_index = self.previous().span.start;

        let variable_list = self.parse_variable_list().unwrap();
        match self.expect_current(token!("=")) {
            Fail => todo!("parse_statement_assignment - error"),
            Success(_) => (),
        }
        self.advance();
        let expression_list = self.parse_expression_list().unwrap();

        let end_index = self.previous().span.end;
        Ok(Spanned {
            span: Span::new(start_index, end_index),
            value: ast::StatementKind::Assignment(Box::new(Assignment {
                name: variable_list,
                init: expression_list,
            })),
        })
    }

    /// Parse a variable list
    ///
    /// ```BNF
    /// variable_list ::= variable {',' variable}
    /// ```
    pub(crate) fn parse_variable_list(
        &mut self,
    ) -> Result<Vec<ast::Identifier>, SpannedSyntaxError> {
        // first there must be one name
        // Assume the parser has been bumped to differentiate the varlist from a function call

        let mut name_list = vec![];

        // var
        if let TokenKind::Lit_Identifier(ref variable_name) = self.previous().kind {
            let interned_name = self.state.interner.intern(variable_name.clone());
            name_list.push(Identifier::new(self.previous().span, interned_name.clone()));
        } else {
            todo!("parse_name_list - error");
        }

        // {',' name}
        while self.advance_if(TokenKind::Comma) {
            if let TokenKind::Lit_Identifier(ref variable_name) = self.current().kind {
                let interned_name = self.state.interner.intern(variable_name.clone());
                name_list.push(Identifier::new(self.current().span, interned_name.clone()));
                self.advance();
            } else {
                todo!("parse_name_list - error");
            }
        }

        Ok(name_list)
    }
}
