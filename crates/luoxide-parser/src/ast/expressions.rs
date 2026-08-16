//! Expression nodes and operator tables.

use ecow::EcoString;
use luoxide_text::range::TextSpan;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use super::statements::Block;
use super::{Identifier, NodeList, P};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExpressionKind {
    /// `nil`, `true`, `42`, `3.14`, `"text"`
    Literal(Literal),
    /// A bare name: `x`
    Identifier(Identifier),
    /// `...`
    Varargs,
    /// `-x`, `not x`, `#x`, `~x`
    Unary {
        op: UnaryOp,
        operand: P<Expression>,
    },
    /// `a + b`, `a .. b`, `a and b`, ...
    Binary {
        op: BinaryOp,
        lhs: P<Expression>,
        rhs: P<Expression>,
    },
    /// `a[b]`
    Index {
        object: P<Expression>,
        index: P<Expression>,
    },
    /// `a.b`
    Member {
        object: P<Expression>,
        name: Identifier,
    },
    /// `f(a, b)`, `f{...}`, `f"text"`
    Call {
        callee: P<Expression>,
        args: NodeList<Expression>,
    },
    /// `a:m(b)`
    MethodCall(P<MethodCall>),
    /// `function(a, b) ... end`
    Function(P<FunctionBody>),
    /// `{ a, b = c, [d] = e }`
    Table(NodeList<Field>),
    /// `( expr )` — kept explicit because parentheses truncate multiple
    /// return values in Lua, so `(f())` is not equivalent to `f()`.
    Grouped(P<Expression>),
    /// Placeholder produced by error recovery; diagnostics carry the details.
    Error,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodCall {
    pub receiver: Expression,
    pub name: Identifier,
    pub args: NodeList<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionBody {
    pub params: NodeList<Identifier>,
    pub is_varargs: bool,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal {
    Nil,
    /// Lua integer (64-bit).
    Int(i64),
    /// Lua float (64-bit).
    Float(f64),
    /// `true` and `false`
    Bool(bool),
    /// String contents with escape sequences already resolved.
    String(EcoString),
}

/// One entry of a table constructor.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Field {
    pub kind: FieldKind,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FieldKind {
    /// `expr` — appended at the next array index.
    Positional(Expression),
    /// `name = expr`
    Named { name: Identifier, value: Expression },
    /// `[key] = expr`
    Indexed { key: Expression, value: Expression },
}

/// Unary operators, in order of appearance in the Lua manual.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOp {
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    /// `-`
    Neg,
    /// `not`
    #[cfg_attr(feature = "serde", serde(rename = "not"))]
    Not,
    /// `#`
    #[cfg_attr(feature = "serde", serde(rename = "#"))]
    Len,
    /// `~`
    #[cfg_attr(feature = "serde", serde(rename = "~"))]
    BitNot,
}

impl UnaryOp {
    /// Binding power of all unary operators (they share one precedence level,
    /// above every binary operator except `^`).
    pub const BINDING_POWER: u8 = 12;

    pub const fn as_str(self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "not",
            UnaryOp::Len => "#",
            UnaryOp::BitNot => "~",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    IDiv,
    Mod,
    Pow,
    // String
    Concat,
    // Comparison
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    // Logical
    And,
    Or,
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

impl BinaryOp {
    /// Left and right binding power, mirroring `luaP`'s priority table.
    ///
    /// For right-associative operators (`..`, `^`) the right power is lower
    /// than the left one, so `a .. b .. c` parses as `a .. (b .. c)`.
    pub const fn binding_power(self) -> (u8, u8) {
        match self {
            BinaryOp::Or => (1, 1),
            BinaryOp::And => (2, 2),
            BinaryOp::Less
            | BinaryOp::Greater
            | BinaryOp::LessEq
            | BinaryOp::GreaterEq
            | BinaryOp::NotEq
            | BinaryOp::Eq => (3, 3),
            BinaryOp::BitOr => (4, 4),
            BinaryOp::BitXor => (5, 5),
            BinaryOp::BitAnd => (6, 6),
            BinaryOp::Shl | BinaryOp::Shr => (7, 7),
            BinaryOp::Concat => (9, 8), // right associative
            BinaryOp::Add | BinaryOp::Sub => (10, 10),
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::IDiv | BinaryOp::Mod => (11, 11),
            BinaryOp::Pow => (14, 13), // right associative, binds tighter than unary
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::IDiv => "//",
            BinaryOp::Mod => "%",
            BinaryOp::Pow => "^",
            BinaryOp::Concat => "..",
            BinaryOp::Eq => "==",
            BinaryOp::NotEq => "~=",
            BinaryOp::Less => "<",
            BinaryOp::LessEq => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEq => ">=",
            BinaryOp::And => "and",
            BinaryOp::Or => "or",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "~",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
        }
    }
}

// Constructors: the parser never builds `Expression` values by hand; going
// through these keeps a single construction path (important for a future
// arena-backed builder).
impl Expression {
    #[inline]
    pub fn literal(literal: Literal, span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Literal(literal),
            span,
        }
    }

    #[inline]
    pub fn identifier(identifier: Identifier) -> Expression {
        let span = identifier.span;
        Expression {
            kind: ExpressionKind::Identifier(identifier),
            span,
        }
    }

    #[inline]
    pub fn varargs(span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Varargs,
            span,
        }
    }

    #[inline]
    pub fn unary(op: UnaryOp, operand: Expression, span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Unary {
                op,
                operand: P(operand),
            },
            span,
        }
    }

    #[inline]
    pub fn binary(op: BinaryOp, lhs: Expression, rhs: Expression) -> Expression {
        let span = lhs.span.merge(rhs.span);
        Expression {
            kind: ExpressionKind::Binary {
                op,
                lhs: P(lhs),
                rhs: P(rhs),
            },
            span,
        }
    }

    #[inline]
    pub fn index(object: Expression, index: Expression, span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Index {
                object: P(object),
                index: P(index),
            },
            span,
        }
    }

    #[inline]
    pub fn member(object: Expression, name: Identifier) -> Expression {
        let span = object.span.merge(name.span);
        Expression {
            kind: ExpressionKind::Member {
                object: P(object),
                name,
            },
            span,
        }
    }

    #[inline]
    pub fn call(callee: Expression, args: NodeList<Expression>, span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Call {
                callee: P(callee),
                args,
            },
            span,
        }
    }

    #[inline]
    pub fn method_call(
        receiver: Expression,
        name: Identifier,
        args: NodeList<Expression>,
        span: TextSpan,
    ) -> Expression {
        Expression {
            kind: ExpressionKind::MethodCall(P(MethodCall {
                receiver,
                name,
                args,
            })),
            span,
        }
    }

    #[inline]
    pub fn function(body: FunctionBody, span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Function(P(body)),
            span,
        }
    }

    #[inline]
    pub fn table(fields: NodeList<Field>, span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Table(fields),
            span,
        }
    }

    #[inline]
    pub fn grouped(inner: Expression, span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Grouped(P(inner)),
            span,
        }
    }

    /// Error-recovery placeholder covering the skipped source range.
    #[inline]
    pub fn error(span: TextSpan) -> Expression {
        Expression {
            kind: ExpressionKind::Error,
            span,
        }
    }

    /// Whether this expression is a function or method call.
    ///
    /// Only calls are valid as expression-statements in Lua.
    #[inline]
    pub fn is_call(&self) -> bool {
        matches!(
            self.kind,
            ExpressionKind::Call { .. } | ExpressionKind::MethodCall(..)
        )
    }
}

// Node size budgets: exceeding these is a deliberate decision, not an accident.
const _: () = assert!(size_of::<Expression>() <= 48);
const _: () = assert!(size_of::<Literal>() <= 24);
