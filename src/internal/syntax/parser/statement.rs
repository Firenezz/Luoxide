

use super::*;

impl<'source> Parser<'source> {
    pub fn main_statement(&mut self) -> Result<ast::Statement, SpannedError> {
        let statement = self.statement();

        todo!("main_statement");
    }
    
    pub fn statement(&mut self) -> Result<ast::Statement, SpannedError> {
        self.simple_stmt()
    }
    
    fn simple_stmt(&mut self) -> Result<ast::Statement, SpannedError> {
        match self.current().kind {
            _ => self.expr_stmt(),
        }
    }
    
    fn expr_stmt(&mut self) -> Result<ast::Statement, SpannedError> {
        self.assign_stmt()
    }
    
    fn assign_stmt(&mut self) -> Result<ast::Statement, SpannedError> {
        //let target = self.expr()?;
        
        
        
        todo!("assign_stmt");
    }
}