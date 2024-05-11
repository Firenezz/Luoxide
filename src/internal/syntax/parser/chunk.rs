use crate::{
    internal::syntax::{lexer::TokenKind, LineAnnotated},
    span::Span,
};

use super::*;

impl<'source> Parser<'source> {
    pub fn parse_chunk(&mut self) -> Result<ast::Chunk, LineAnnotated<SpannedSyntaxError>> {
        Ok(ast::Chunk {
            block: self.parse_block()?,
            span: Span::new(0, self.current().span.end),
            file_name: None,
            globals: vec![],
        })
    }

    /// Parse a block
    ///
    /// Equivalent of a chunk
    ///
    /// ```BNF
    /// block ::= {statement} [return_statement]
    /// ```
    pub(super) fn parse_block(&mut self) -> Result<ast::Block, LineAnnotated<SpannedSyntaxError>> {
        let mut statements = vec![];
        //let mut return_statement = None;

        let start_index = self.current().span.start;

        while !self.is_end_of_block() && !self.is_at_end() {
            match self.current().kind {
                TokenKind::Tok_SemiColon => {
                    self.bump();
                }
                TokenKind::Lit_Identifier(_) => {
                    self.bump();
                    match self.current().kind {
                        TokenKind::Tok_Comma | TokenKind::Op_Assign => {
                            statements.push(self.parse_statement_assignment()?);
                        }
                        TokenKind::Brk_LeftParen => {
                            // Start function call parsing
                            todo!("parse_block - function call");
                        }
                        _ => {
                            todo!("parse_block - error - Invalid statement");
                        }
                    }
                }

                TokenKind::Kw_Else
                | TokenKind::Kw_ElseIf
                | TokenKind::Kw_End
                | TokenKind::Kw_Until => break,
                TokenKind::Kw_Return => {
                    todo!("parse_block - return statement");
                    /*return_statement = Some(LineAnnotated::new(
                        next.line_number,
                        self.parse_return_statement()?,
                    ));*/
                    //break;
                }
                _ => {
                    todo!("parse_block - return statement");
                    /*statements.push(LineAnnotated::new(
                        next.line_number,
                        self.parse_statement()?,
                    ));*/
                }
            }
        }

        Ok(ast::Block {
            statements,
            span: Span::new(start_index, self.previous().span.end),
        })
    }
}
