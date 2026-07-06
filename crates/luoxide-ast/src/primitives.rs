use crate::ast::expressions::Expression;



pub type ByteOffset = u32;


pub enum Statement {
    Expr(Expression),
}

pub enum Delimiter {
    Parentheses,
    SquareBrackets,
    CurlyBrackets,
    DoEnd,
    ThenEnd,
}