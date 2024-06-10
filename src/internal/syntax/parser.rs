pub mod chunk;
pub mod common;
mod error;
mod expressions;
pub mod precedence;
mod statement;

use crate::{error::SpannedSyntaxError, span::Span};

use std::rc::Rc;

use crate::{intern::DefaultInterner, internal::syntax::lexer::Lexer};

use super::{
    ast::{self, Expression},
    lexer::{Token, TokenKind},
    SyntaxError,
};

use ParseResult::*;

#[allow(dead_code)]
const MAX_RECURSION: usize = 200;

#[allow(dead_code)]
const MIN_PRIORITY: u8 = 0;

#[allow(clippy::result_unit_err)]
pub fn parse_chunk<Source: AsRef<str>>(source: Source) -> Result<ast::Chunk, ()> {
    let interner = Rc::from(DefaultInterner::default());

    let lexer = Lexer::new(source.as_ref(), interner.clone());
    let mut parser = Parser::new(lexer);
    Ok(parser.parse_chunk().unwrap())
}

#[allow(clippy::result_unit_err)] // TODO: Remove this
pub fn parse_expression<Source: AsRef<str>>(source: Source) -> Result<ast::Expression, ()> {
    let interner = Rc::from(DefaultInterner::default());

    let lexer = Lexer::new(source.as_ref(), interner.clone());
    let mut parser = Parser::new(lexer);
    Ok(parser.parse_expression().unwrap())
}

#[allow(dead_code)] // TODO: remove this after ast is finished
pub struct ParserState {
    interner: Rc<DefaultInterner>,
    current_loop: Option<()>,
    current_function: Option<()>,
    recursion_guard: Rc<()>,
}

pub struct FunctionState {
    _upvalues: Vec<Rc<[u8]>>,
    _environment: u8,
    _return_statements: Option<Vec<Expression>>,
}

#[allow(dead_code)] // TODO: remove this after ast is finished
impl ParserState {
    pub fn new() -> Self {
        Self {
            interner: Rc::from(DefaultInterner::default()),
            current_loop: None,
            current_function: None,
            recursion_guard: Rc::new(()),
        }
    }
}

impl Default for ParserState {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)] // TODO: remove this after ast is finished
pub struct Parser<'source> {
    lexer: Lexer<'source>,
    errors: Vec<SpannedSyntaxError>,
    state: ParserState,
}

impl<'source> Parser<'source> {
    pub fn new(lexer: Lexer<'source>) -> Self {
        Self {
            lexer,
            errors: vec![],
            state: ParserState::new(),
        }
    }

    pub fn previous(&self) -> &Token {
        self.lexer.previous()
    }

    pub fn current(&self) -> &Token {
        self.lexer.current()
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.current().span
    }

    pub fn is_at_end(&self) -> bool {
        use crate::internal::syntax::lexer::TokenKind::Tok_Eof;
        self.current().is(Tok_Eof)
    }

    pub fn is_end_of_block(&self) -> bool {
        use crate::internal::syntax::lexer::TokenKind::End;
        self.current().is(End)
    }

    #[inline]
    pub fn take(&mut self) -> &Token {
        self.lexer.bump();
        self.lexer.current()
    }

    pub fn advance(&mut self) {
        self.lexer.bump();
    }

    pub fn advance_by<const N: usize>(&mut self) {
        for _ in 0..N {
            self.lexer.bump();
            if self.is_at_end() {
                break;
            }
        }
    }

    pub fn advance_if(&mut self, kind: TokenKind) -> bool {
        matches!(self.expect_current(kind), Success(_))
    }

    /// Takes a token and it must be of the given kind
    ///
    ///
    pub fn must(&mut self, kind: TokenKind) {
        if !self.take().is(kind) {
            todo!("Report unexpected token");
        }
    }

    pub fn is(&mut self, kind: TokenKind) -> bool {
        self.current().is(kind)
    }
}

pub enum ParseResult<T> {
    Fail,
    Success(T),
}

impl<T> ParseResult<T> {
    #[inline]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    #[inline]
    pub fn is_fail(&self) -> bool {
        matches!(self, Self::Fail)
    }

    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Success(t) => Some(t),
            Self::Fail => None,
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Success(t) => Some(t),
            Self::Fail => None,
        }
    }

    #[inline]
    pub fn into_inner(self) -> Option<T> {
        match self {
            Self::Success(t) => Some(t),
            Self::Fail => None,
        }
    }
}

impl<T> From<ParseResult<T>> for Result<T, ()> {
    fn from(result: ParseResult<T>) -> Self {
        match result {
            ParseResult::<T>::Success(t) => Ok(t),
            ParseResult::<T>::Fail => Err(()),
        }
    }
}

impl<T> From<ParseResult<T>> for Option<T> {
    fn from(result: ParseResult<T>) -> Self {
        match result {
            ParseResult::<T>::Success(t) => Some(t),
            ParseResult::<T>::Fail => None,
        }
    }
}

impl<T> From<ParseResult<T>> for bool {
    fn from(result: ParseResult<T>) -> Self {
        match result {
            ParseResult::<T>::Success(_) => true,
            ParseResult::<T>::Fail => false,
        }
    }
}

impl<T> From<ParseResult<T>> for () {
    fn from(result: ParseResult<T>) -> Self {
        match result {
            ParseResult::<T>::Success(_) => (),
            ParseResult::<T>::Fail => (),
        }
    }
}

#[cfg(test)]
mod tests;
