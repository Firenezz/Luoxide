use std::ops::{Deref, DerefMut};

use crate::Cow;

use crate::span::{Span, Spanned};

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub struct Identifier<'source>(Spanned<Cow<'source, str>>);

impl<'source> Identifier<'source> {
    pub fn new(span: impl Into<Span>, lexeme: Cow<'source, str>) -> Self {
        Self(Spanned::new(span, lexeme))
    }

    pub fn lexeme(&self) -> Cow<'source, str> {
        self.0.deref().clone()
    }

    pub fn as_str(&self) -> &str {
        self.0.deref().as_ref()
    }
}

impl<'a, 'source> PartialEq<&'a str> for Identifier<'source> {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl<'source> Deref for Identifier<'source> {
    type Target = Spanned<Cow<'source, str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'source> DerefMut for Identifier<'source> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Chunk {
    pub body: Vec<Statement>,
    pub span: Span
}

impl Chunk {
    pub fn new() -> Self {
        Self { body: vec![], span: Span::new(0, 0) }
    }
}

pub type Statement = Spanned<StatementKind>;

#[cfg_attr(test, derive(Debug))]
pub enum StatementKind {
    Variable,
    Control(Box<Control>),
    Loop(Box<Loop>),
}

pub 

pub type Expression = Spanned<ExpressionKind>;

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub enum ExpressionKind {
    Literal,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub enum Loop {
    For,
    While,
    Repeat,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub enum Control {
    Return(Return),
    Break,
}

#[cfg_attr(test, derive(Debug))]
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

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub struct Variable {
    //pub name: Identifier,
    pub value: Expression,
}

pub enum Literal<'source> {
    Nil,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Cow<'source, str>),
    //Table(Box<Table<'source>>),
}

pub enum UnaryOperator {
    Not,
    Minus,
    Length,
    BitNot,
}
