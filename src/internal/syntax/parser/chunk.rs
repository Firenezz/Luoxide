use super::*;

impl<'source> Parser<'source> {
    pub fn parse(&mut self) -> Result<ast::Chunk, ()> {
        while !self.is_at_end() {
            self.parse_block()
        }

        Ok(ast::Chunk::new())
    }

    pub fn parse_expression(&mut self) -> Result<ast::Expression, ()> {
        self.expression()
    }

    /// Parse a block
    ///
    /// Equivalent of a chunk
    ///
    /// ```BNF
    /// block ::= chunk
    /// ```
    pub(super) fn parse_block(&mut self) {
        while !self.is_end_of_block() {
            self.statement();
        }
    }
}
