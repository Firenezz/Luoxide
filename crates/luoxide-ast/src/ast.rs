use std::{ops::Deref, rc::Rc};

use luoxide_text::{range::TextSpan, size::TextSize};

type IdentifierName = String;

//#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone, Debug)]
pub struct Identifier {
    name: String,
}

#[derive(Debug, Clone, Copy)]
pub struct AstMetadata {
    pub span: TextSpan,
    pub line: TextSize,
    pub column: TextSize,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn to_str(&self) -> &str {
        self.name.as_str()
    }

    pub fn to_owned(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Grouping<T> {
    pub value: T,
    pub span: TextSpan,
}

impl<T> Deref for Grouping<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

use expressions::*;
pub mod expressions {

    use crate::operator::{BinaryOperator, UnaryOperator};

    use super::*;

    //#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone, Debug)]
    pub struct Expression {
        pub kind: ExpressionKind,
        pub span: TextSpan,
        #[cfg(feature = "metadata")]
        pub ast_metadata: AstMetadata,
    }

    //#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone, Debug)]
    pub enum ExpressionKind {
        Empty,
        Literal(Literal),
        Identifier(Identifier),
        MemberExpression(MemberExpression),
        VarGet,
        Indexer,
        CallExpression(CallExpression),
        UnaryOperator(UnaryExpression),
        BinaryOperator(BinaryExpression),
    }

    //#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone, Debug)]
    pub enum Literal {
        Nil,
        // Max is i32 but for ease of implementation we use i64 for now
        /// Represent a number in the range -2^31 to 2^31 - 1
        Int(i64),
        /// Represent a floating number in the range -2^63 to 2^63 - 1
        Float(f64),
        /// Represent a boolean
        /// `true` and `false`
        Bool(bool),
        /// Represent a string
        ///
        /// This comes from a interner
        String(u32),
    }

    #[derive(Debug, Clone)]
    pub struct CallExpression {
        pub callee: Box<Expression>,
        pub arguments: Vec<Expression>,
    }

    #[derive(Debug, Clone)]
    pub struct MemberExpression {
        pub base: Box<Expression>,
        pub property: Box<Expression>,
    }

    #[derive(Debug, Clone)]
    pub struct UnaryExpression {
        pub operator: UnaryOperator,
        pub operand: Box<Expression>,
    }

    #[derive(Debug, Clone)]
    pub struct BinaryExpression {
        pub left: Box<Expression>,
        pub operator: BinaryOperator,
        pub right: Box<Expression>,
    }
}

#[derive(Debug)]
pub struct Field {
    pub key: Option<Expression>,
    pub init: Expression,
}

impl Field {
    pub fn new(key: Option<Expression>, init: Expression) -> Field {
        Field { key, init }
    }
}

impl Identifier {
    pub fn create_identifier<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
        }
    }
}
