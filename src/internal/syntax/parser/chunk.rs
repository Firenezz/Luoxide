use super::*;

impl<'source> Parser<'source> {
    pub fn parse(&mut self) -> Result<ast::Chunk, ()> {
        while !self.is_at_end() {
            self.parse_block()
        }

        Ok(ast::Chunk::new())
    }

    /// Parse a block
    ///
    /// Equivalent of a chunk
    ///
    /// ```
    /// block ::= chunk
    /// ```
    pub(super) fn parse_block(&mut self) {
        while !self.is_end_of_block() {}
    }
}
