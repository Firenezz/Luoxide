use std::default;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::Cow;

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
    pub globals: Vec<String>,
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

pub type Statement = Spanned<StatementKind>;

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
pub enum StatementKind {
    Assignment(Box<Assignment>),
    Variable,
    Control(Box<Control>),
    Loop(Box<Loop>),
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
pub struct FunctionCall {
    pub target: Expression,
    pub args: Vec<Expression>,
}

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub struct Assignment {
    pub name: Vec<Identifier>,
    pub init: Vec<Expression>,
}

pub type Expression = Spanned<ExpressionKind>;

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub enum ExpressionKind {
    Literal(Box<Literal>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Varargs,
    Call(Box<FunctionCall>),
}

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
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
    String(Rc<[u8]>),
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

impl TryFrom<super::lexer::TokenKind> for BinaryOperator {
    type Error = ();

    fn try_from(token: super::lexer::TokenKind) -> Result<Self, Self::Error> {
        use super::lexer::TokenKind;
        match token {
            TokenKind::Op_Add => Ok(Self::Add),
            TokenKind::Op_Minus => Ok(Self::Sub),
            TokenKind::Op_Mul => Ok(Self::Mul),
            TokenKind::Op_Div => Ok(Self::Div),
            TokenKind::Op_Mod => Ok(Self::Mod),
            TokenKind::Op_Pow => Ok(Self::Pow),
            TokenKind::Op_Concat => Ok(Self::Concat),
            TokenKind::Op_NotEqual => Ok(Self::NotEqual),
            TokenKind::Op_LessThan => Ok(Self::LessThan),
            TokenKind::Op_LessEqual => Ok(Self::LessThanEqual),
            TokenKind::Op_GreaterThan => Ok(Self::GreaterThan),
            TokenKind::Op_GreaterEqual => Ok(Self::GreaterThanEqual),
            TokenKind::Op_BitAnd => Ok(Self::BitAnd),
            TokenKind::Op_BitOr => Ok(Self::BitOr),
            TokenKind::Op_BitXor => Ok(Self::BitXor),
            TokenKind::Op_Dot => Ok(Self::Concat),
            TokenKind::Op_Equal => Ok(Self::Equal),
            TokenKind::Kw_And => Ok(Self::And),
            TokenKind::Kw_Or => Ok(Self::Or),
            _ => Err(()),
        }
    }
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
pub enum UnaryOperator {
    Not,
    Minus,
    BitNot,
    Lenght,
}

impl From<super::lexer::TokenKind> for UnaryOperator {
    fn from(token: super::lexer::TokenKind) -> Self {
        use super::lexer::TokenKind;
        match token {
            TokenKind::Kw_Not => Self::Not,
            TokenKind::Op_Minus => Self::Minus,
            TokenKind::Op_BitXor => Self::BitNot,
            TokenKind::Op_Len => Self::Lenght,
            _ => unreachable!(),
        }
    }
}

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub struct Unary {
    pub op: UnaryOperator,
    pub right: Expression,
}
