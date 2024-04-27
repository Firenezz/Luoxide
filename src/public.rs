
use crate::internal::syntax::ast;


pub fn parse_expression(source: &str) -> Result<ast::Expression, ()> {
    crate::internal::syntax::parser::parse_expression(source)
}