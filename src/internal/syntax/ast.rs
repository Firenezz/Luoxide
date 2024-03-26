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
    pub block: Vec<Statement>,
    pub span: Span,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            block: vec![],
            span: Span::new(0, 0),
        }
    }
}

pub type Statement = Spanned<StatementKind>;

#[cfg_attr(test, derive(Debug))]
pub enum StatementKind {
    Variable,
    Control(Box<Control>),
    Loop(Box<Loop>),
}

pub type Expression<'src> = Spanned<ExpressionKind<'src>>;

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub enum ExpressionKind<'src> {
    Literal(Box<Literal<'src>>),
    /*Binary(Box<Binary<'src>>),*/
    Unary(Box<Unary<'src>>),
    /*GetVar(Box<GetVar<'src>>),
    SetVar(Box<SetVar<'src>>),
    GetField(Box<GetField<'src>>),
    SetField(Box<SetField<'src>>),
    GetIndex(Box<GetIndex<'src>>),
    SetIndex(Box<SetIndex<'src>>),
    Call(Box<Call<'src>>),
    GetSelf,
    GetSuper,*/
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
pub struct Variable<'a> {
    //pub name: Identifier,
    pub value: Expression<'a>,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub enum Literal<'src> {
    None,
    Int(i32),
    Float(f64),
    Bool(bool),
    String(Cow<'src, str>),
    List(Vec<Expression<'src>>),
    Table(Vec<(Expression<'src>, Expression<'src>)>), // TODO: Not implemented
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnaryOperator {
    Not,
    Minus,
    BitNot,
    Lenght,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub struct Unary<'src> {
    pub op: UnaryOperator,
    pub right: Expression<'src>,
}
