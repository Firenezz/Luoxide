use std::default;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::Cow;

use crate::span::{Span, Spanned};

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub struct Identifier(Spanned<Rc<str>>);

impl Identifier {
    pub fn new(span: impl Into<Span>, lexeme: Rc<str>) -> Self {
        Self(Spanned::new(span, lexeme))
    }

    pub fn lexeme(&self) -> Rc<str> {
        self.0.deref().clone()
    }

    pub fn as_str(&self) -> &str {
        self.0.deref().as_ref()
    }
}

impl<'a> PartialEq<&'a str> for Identifier {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl Deref for Identifier {
    type Target = Spanned<Rc<str>>;

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

/*#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub struct FunctionCall<'source> {
    pub target: Expression<'source>,
    pub args: Vec<Expression<'source>>,
}*/

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub struct Variable {
    //pub name: Identifier,
    pub value: Expression,
}

pub type Expression = Spanned<ExpressionKind>;

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub enum ExpressionKind {
    Literal(Box<Literal>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    /*GetVar(Box<GetVar<'src>>),
    SetVar(Box<SetVar<'src>>),
    GetField(Box<GetField<'src>>),
    SetField(Box<SetField<'src>>),
    GetIndex(Box<GetIndex<'src>>),
    SetIndex(Box<SetIndex<'src>>),
    Call(Box<Call<'src>>),
    GetSelf,
    GetSuper,*/
    //List(Vec<Expression<'src>>),
    //Table(Vec<(Expression<'src>, Expression<'src>)>), // TODO: Not implemented
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
    And,
    Or,
}

impl From<super::lexer::TokenKind> for BinaryOperator {
    fn from(token: super::lexer::TokenKind) -> Self {
        use super::lexer::TokenKind;
        match token {
            TokenKind::Op_Add => Self::Add,
            TokenKind::Op_Minus => Self::Sub,
            TokenKind::Op_Mul => Self::Mul,
            TokenKind::Op_Div => Self::Div,
            TokenKind::Op_Mod => Self::Mod,
            TokenKind::Op_Pow => Self::Pow,
            TokenKind::Op_Concat => Self::Concat,
            TokenKind::Op_NotEqual => Self::NotEqual,
            TokenKind::Op_LessThan => Self::LessThan,
            TokenKind::Op_LessEqual => Self::LessThanEqual,
            TokenKind::Op_GreaterThan => Self::GreaterThan,
            TokenKind::Op_GreaterEqual => Self::GreaterThanEqual,
            TokenKind::Op_BitAnd => Self::And,
            TokenKind::Op_BitOr => Self::Or,
            TokenKind::Op_Dot => Self::Concat,
            TokenKind::Op_Equal => Self::Equal,
            _ => unreachable!(),
        }
    }
}

#[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
#[derive(Clone)]
pub struct Binary {
    pub left: Expression,
    pub op: BinaryOperator,
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
