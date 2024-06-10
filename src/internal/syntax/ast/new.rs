use crate::span::Span;

use super::{BinaryOperator, Expression, ExpressionKind, UnaryOperator};

use super::expression;

impl expression::Literal {
    #[inline]
    pub fn new_nil(span: impl Into<Span>) -> Expression {
        Expression {
            span: span.into(),
            kind: ExpressionKind::Literal(super::Literal::Nil),
        }
    }

    #[inline]
    pub fn new_int(span: impl Into<Span>, value: i64) -> Expression {
        Expression {
            span: span.into(),
            kind: ExpressionKind::Literal(super::Literal::Int(value)),
        }
    }

    #[inline]
    pub fn new_float(span: impl Into<Span>, value: f64) -> Expression {
        Expression {
            span: span.into(),
            kind: ExpressionKind::Literal(super::Literal::Float(value)),
        }
    }

    #[inline]
    pub fn new_bool(span: impl Into<Span>, value: bool) -> Expression {
        Expression {
            span: span.into(),
            kind: ExpressionKind::Literal(super::Literal::Bool(value)),
        }
    }

    #[inline]
    pub fn new_string(span: impl Into<Span>, value: std::rc::Rc<[u8]>) -> Expression {
        Expression {
            span: span.into(),
            kind: ExpressionKind::Literal(super::Literal::String(value)),
        }
    }
}

#[allow(clippy::new_ret_no_self)]
impl expression::Binary {
    #[inline]
    pub fn new(
        span: impl Into<Span>,
        left: Expression,
        right: Expression,
        op: BinaryOperator,
    ) -> Expression {
        Expression {
            span: span.into(),
            kind: ExpressionKind::Binary(Box::new(Self {
                left,
                right,
                operator: op,
            })),
        }
    }
}

#[allow(clippy::new_ret_no_self)]
impl expression::Unary {
    #[inline]
    pub fn new(span: impl Into<Span>, right: Expression, op: UnaryOperator) -> Expression {
        Expression {
            span: span.into(),
            kind: ExpressionKind::Unary(Box::new(Self { op, right })),
        }
    }
}
