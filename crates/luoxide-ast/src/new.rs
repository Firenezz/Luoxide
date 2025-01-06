use std::cell::{Cell, RefCell};

use luoxide_text::range::TextSpan;

use crate::ast::expressions::{CallExpression, Expression, ExpressionKind, Literal, MemberExpression};

impl Literal {
    pub const fn create_nil(at: TextSpan) -> Expression {
        create_expression(ExpressionKind::Literal(Literal::Nil), at)
    }

    pub const fn create_number(value: i64, at: TextSpan) -> Expression {
        create_expression(ExpressionKind::Literal(Literal::Int(value)), at)
    }

    pub const fn create_string(value: String, at: TextSpan) -> Expression {
        create_expression(ExpressionKind::Literal(Literal::String(value)), at)
    }

    pub const fn create_bool(value: bool, at: TextSpan) -> Expression {
        create_expression(ExpressionKind::Literal(Literal::Bool(value)), at)
    }
}

impl MemberExpression {
    pub fn create(base: Box<Expression>, property: Box<Expression>, at: TextSpan) -> Expression {
        create_expression(ExpressionKind::MemberExpression(MemberExpression { base, property }), at)
    }
}

impl CallExpression {
    pub fn create(callee: Box<Expression>, args: Vec<Expression>, at: TextSpan) -> Expression {
        create_expression(ExpressionKind::CallExpression(CallExpression { callee, arguments: args }), at)
    }
}

#[inline]
pub const fn create_expression(kind: ExpressionKind, span: TextSpan) -> Expression {
    Expression { kind, span }
}