use crate::{internal::syntax::LineAnnotated, span::Span};

use super::*;

impl<'source> Parser<'source> {
    pub fn parse(&mut self) -> Result<ast::Chunk, LineAnnotated<SpannedError>> {
        Ok(ast::Chunk {
            block: self.parse_block()?,
            span: Span::new(0, 0),
        })
    }

    pub fn parse_expression(&mut self) -> Result<ast::Expression, ()> {
        self.expression()
    }

    /// Parse a block
    ///
    /// Equivalent of a chunk
    ///
    /// ```BNF
    /// block ::= {statement} [return_statement]
    /// ```
    pub(super) fn parse_block(&mut self) -> Result<ast::Block, LineAnnotated<SpannedError>> {
        let mut statements = vec![];
        //let mut return_statement = None;

        while !self.is_end_of_block() {
            statements.push(self.statement());
        }

        //Ok(ast::Block { statements, span:  })

        todo!("parse_block")
    }
}
