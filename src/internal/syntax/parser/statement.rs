use super::*;

impl<'source> Parser<'source> {
    pub fn main_statement(&mut self) -> Result<ast::Statement, SpannedError> {
        //let statement = self.statement();

        todo!("main_statement");
    }

    pub fn statement(&mut self) -> Result<ast::Statement, SpannedError> {
        //self.simple_stmt()
        todo!("statement");
    }
}
