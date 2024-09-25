use luoxide_text::range::TextSpan;

use crate::ast::{expressions::{Expression, ExpressionKind, Literal}, Identifier};

impl<'_> Literal {
    pub fn create_nil(at: TextSpan) -> Expression<'_> {
        Expression {
            kind: ExpressionKind::Literal(Literal::Nil),
            span: at,
        }
    }

    pub fn create_number(value: i64, at: TextSpan) -> Expression<'_> {
        Expression {
            kind: ExpressionKind::Literal(Literal::Int(value)),
            span: at,
        }
    }

    pub fn create_string(value: String, at: TextSpan) -> Expression<'_> {
        Expression {
            kind: ExpressionKind::Literal(Literal::String(value)),
            span: at,
        }
    }

    pub fn create_bool(value: bool, at: TextSpan) -> Expression<'_> {
        Expression {
            kind: ExpressionKind::Literal(Literal::Bool(value)),
            span: at,
        }
    }
}

impl Identifier {
    pub fn create_identifier<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref()
        }
    }
}