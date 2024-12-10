use luoxide_text::range::TextSpan;

use crate::ast::{
    expressions::{Expression, ExpressionKind, Literal},
    Identifier,
};

impl Literal {
    pub fn create_nil(at: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Literal(Literal::Nil),
            span: at,
        }
    }

    pub fn create_number(value: i64, at: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Literal(Literal::Int(value)),
            span: at,
        }
    }

    pub fn create_string(value: String, at: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Literal(Literal::String(value)),
            span: at,
        }
    }

    pub fn create_bool(value: bool, at: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Literal(Literal::Bool(value)),
            span: at,
        }
    }
}
