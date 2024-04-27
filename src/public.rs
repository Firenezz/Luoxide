use crate::internal::syntax::ast;

#[allow(clippy::result_unit_err)] // TODO: remove this after ast is finished
pub fn parse_expression(source: &str) -> Result<ast::Expression, ()> {
    crate::internal::syntax::parser::parse_expression(source)
}
