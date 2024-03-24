use super::*;

impl<'source> Parser<'source> {
    pub fn parse_chunk(&mut self) -> Result<ast::Chunk, ()> {
        while !self.is_at_end() {
            
        }

        Ok(ast::Chunk::new())
    }
}