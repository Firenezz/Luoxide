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
                TokenKind::SemiColon => {
                    self.advance();
                }
                TokenKind::Lit_Identifier(_) => {
                    self.advance();
                    match self.current().kind {
                        TokenKind::Comma | TokenKind::Assign => {
                            statements.push(self.parse_statement_assignment(ScopeKind::Global)?);
                        }
                        TokenKind::LeftParen => {
                            // Start function call parsing
                            todo!("parse_block - function call");
                        }
                        _ => {
                            todo!("parse_block - error - Invalid statement");
                        }
                    }
                }

                TokenKind::Else | TokenKind::ElseIf | TokenKind::End | TokenKind::Until => break,
                TokenKind::Return => {
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
