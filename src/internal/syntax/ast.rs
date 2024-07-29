mod new;
pub mod visitor;

use crate::Cow;
use std::default;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::span::{Span, Spanned};

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub struct Identifier(Spanned<Rc<[u8]>>);

impl Identifier {
    pub fn new(span: impl Into<Span>, lexeme: Rc<[u8]>) -> Self {
        Self(Spanned::new(span, lexeme))
    }

    pub fn lexeme(&self) -> Rc<[u8]> {
        self.0.deref().clone()
    }

    pub fn as_str(&self) -> &[u8] {
        self.0.deref().as_ref()
    }
}

impl<'a> PartialEq<&'a [u8]> for Identifier {
    fn eq(&self, other: &&'a [u8]) -> bool {
        self.as_str() == *other
    }
}

impl Deref for Identifier {
    type Target = Spanned<Rc<[u8]>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
/// A chunk of syntax
///
/// Chunks are the root of the syntax tree.
/// It represents an indenpendently executable chunk of code
///
/// ```BNF
/// chunk ::= block
/// ```
pub struct Chunk {
    /// The body of the chunk
    pub block: Block,
    /// The span of the chunk
    ///
    /// This is normally the whole file or string
    pub span: Span,
    pub file_name: Option<Cow<'static, str>>,
    pub globals: Vec<Identifier>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            block: Block::default(),
            span: Span::new(0, 0),
            file_name: None,
            globals: vec![],
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: vec![],
            span: Span::new(0, 0),
        }
    }
}

impl default::Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

pub use statement::*;
pub mod statement {
    use crate::internal::syntax::parser::contexts::{Marked, Marker};

    use super::*;

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Statement {
        pub location: Option<Marker>,
        pub value: StatementKind,
    }

    impl Statement {
        pub fn new(kind: StatementKind) -> Self {
            Self {
                location: None,
                value: kind,
            }
        }
    }

    impl Marked for Statement {
        fn mark(&mut self, location: Marker) {
            self.location = Some(location)
        }
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub enum StatementKind {
        Assignment(Assignment),
        Variable,
        Control(Control),
        Loop(Loop),
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub enum Loop {
        For,
        While,
        Repeat,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub enum Control {
        Return(Return),
        Break,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Return {
        //pub value: Option<Expression<'source>>,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Assignment {
        pub name: Vec<Expression>,
        pub init: Vec<Expression>,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct FunctionCall {
        /// The target of the call
        ///
        /// example: `foo.bar` or `foo()()`
        pub base: Expression,
        pub arguments: Vec<Expression>,
    }
}

pub use expression::*;

pub mod expression {
    use crate::internal::syntax::parser::contexts::{Marked, Marker};

    use super::super::lexer::TokenKind;

    use super::*;

    //pub type Expression = Spanned<ExpressionKind>;

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Expression {
        pub location: Option<Box<Marker>>,
        pub kind: ExpressionKind,
    }

    impl Marked for Expression {
        fn mark(&mut self, location: Marker) {
            self.location = Some(Box::new(location));
        }
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    #[repr(C)]
    pub enum ExpressionKind {
        Literal(Literal),
        Binary(Box<Binary>),
        Unary(Box<Unary>),
        Member(Box<Member>),
        Varargs,
        Call(Box<FunctionCall>),
        Index(Box<Index>),

        Identifier(Identifier),
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Index {
        pub base: Expression,
        pub index: Expression,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    #[repr(C)]
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
        String(std::rc::Rc<[u8]>),
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Member {
        pub indexer: IndexerOperator,
        pub identifier: Identifier,
        pub base: Expression,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum IndexerOperator {
        Dot,
        Bracket,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Binary {
        pub left: Expression,
        pub operator: BinaryOperator,
        pub right: Expression,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum BinaryOperator {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        Pow,
        Concat,
        Equal,
        NotEqual,
        LessThan,
        LessThanEqual,
        GreaterThan,
        GreaterThanEqual,
        BitAnd,
        BitOr,
        BitXor,
        And,
        Or,
    }

    impl TryFrom<TokenKind> for BinaryOperator {
        type Error = ();

        fn try_from(token: super::super::lexer::TokenKind) -> Result<Self, Self::Error> {
            use super::super::lexer::TokenKind;
            match token {
                TokenKind::Add => Ok(Self::Add),
                TokenKind::Minus => Ok(Self::Sub),
                TokenKind::Mul => Ok(Self::Mul),
                TokenKind::Div => Ok(Self::Div),
                TokenKind::Mod => Ok(Self::Mod),
                TokenKind::Pow => Ok(Self::Pow),
                TokenKind::Concat => Ok(Self::Concat),
                TokenKind::NotEqual => Ok(Self::NotEqual),
                TokenKind::LessThan => Ok(Self::LessThan),
                TokenKind::LessEqual => Ok(Self::LessThanEqual),
                TokenKind::GreaterThan => Ok(Self::GreaterThan),
                TokenKind::GreaterEqual => Ok(Self::GreaterThanEqual),
                TokenKind::BitAnd => Ok(Self::BitAnd),
                TokenKind::BitOr => Ok(Self::BitOr),
                TokenKind::BitXor => Ok(Self::BitXor),
                TokenKind::Dot => Ok(Self::Concat),
                TokenKind::Equal => Ok(Self::Equal),
                TokenKind::And => Ok(Self::And),
                TokenKind::Or => Ok(Self::Or),
                _ => Err(()),
            }
        }
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Unary {
        pub op: UnaryOperator,
        pub right: Expression,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum UnaryOperator {
        Not,
        Minus,
        BitNot,
        Lenght,
    }

    impl From<TokenKind> for UnaryOperator {
        fn from(token: TokenKind) -> Self {
            use TokenKind;
            match token {
                TokenKind::Not => Self::Not,
                TokenKind::Minus => Self::Minus,
                TokenKind::BitXor => Self::BitNot,
                TokenKind::Pound => Self::Lenght,
                _ => unreachable!(),
            }
        }
    }

    impl Identifier {
        pub fn new_identifier(span: impl Into<Span>, lexeme: Rc<[u8]>) -> Expression {
            Expression {
                location: None,
                kind: ExpressionKind::Identifier(Self::new(span, lexeme)),
            }
        }
    }
}

pub trait Node: crate::internal::util::Sealed {
    fn span(&self) -> Span;

    fn similar(&self, other: &Self) -> bool;
}
