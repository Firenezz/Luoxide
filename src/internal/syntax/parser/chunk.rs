use crate::{
    internal::syntax::{lexer::TokenKind, LineAnnotated},
    span::Span,
};

use super::*;

impl<'source> Parser<'source> {
    pub fn parse(&mut self) -> Result<ast::Chunk, LineAnnotated<SpannedError>> {
        Ok(ast::Chunk {
            block: self.parse_block()?,
            span: Span::new(0, 0),
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
    pub(super) fn parse_block(&mut self) -> Result<ast::Block, LineAnnotated<SpannedError>> {
        //let mut statements = vec![];
        //let mut return_statement = None;

        while !self.is_end_of_block() {
            match self.current().kind {
                TokenKind::Kw_Else
                | TokenKind::Kw_ElseIf
                | TokenKind::Kw_End
                | TokenKind::Kw_Until => break,
                TokenKind::Tok_SemiColon => {
                    self.bump();
                }
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

        //Ok(ast::Block { statements, span:  })

        todo!("parse_block")
    }
}
