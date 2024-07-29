use crate::span::Span;

use super::{BinaryOperator, Expression, ExpressionKind, UnaryOperator};

use super::expression;

impl expression::Literal {
    #[inline]
    pub fn new_nil() -> Expression {
        Expression {
            location: None,
            kind: ExpressionKind::Literal(super::Literal::Nil),
        }
    }

    #[inline]
    pub fn new_int(value: i64) -> Expression {
        Expression {
            location: None,
            kind: ExpressionKind::Literal(super::Literal::Int(value)),
        }
    }

    #[inline]
    pub fn new_float(value: f64) -> Expression {
        Expression {
            location: None,
            kind: ExpressionKind::Literal(super::Literal::Float(value)),
        }
    }

    #[inline]
    pub fn new_bool(value: bool) -> Expression {
        Expression {
            location: None,
            kind: ExpressionKind::Literal(super::Literal::Bool(value)),
        }
    }

    #[inline]
    pub fn new_string(value: std::rc::Rc<[u8]>) -> Expression {
        Expression {
            location: None,
            kind: ExpressionKind::Literal(super::Literal::String(value)),
        }
    }
}

#[allow(clippy::new_ret_no_self)]
impl expression::Binary {
    #[inline]
    pub fn new(left: Expression, right: Expression, op: BinaryOperator) -> Expression {
        Expression {
            location: None,
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
    pub fn new(right: Expression, op: UnaryOperator) -> Expression {
        Expression {
            location: None,
            kind: ExpressionKind::Unary(Box::new(Self { op, right })),
        }
    }
}
