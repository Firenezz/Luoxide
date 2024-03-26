pub mod chunk;
pub mod common;
mod statement;
//mod expressions;

use crate::error::SpannedError;

use std::rc::Rc;

use crate::{intern::DefaultInterner, internal::syntax::lexer::Lexer};

use super::{ast, lexer::Token, SyntaxError};

pub struct LineAnnotated<T> {
    pub inner: T,
    pub line_number: u64,
}

pub fn parse_chunk<Source: AsRef<str>>(source: Source) -> Result<ast::Chunk, SyntaxError> {
    let interner = Rc::from(DefaultInterner::default());

    let lexer = Lexer::new(source.as_ref(), interner.clone());
    let mut parser = Parser::new(lexer, interner.clone());
    parser.parse();
    todo!("parse")
}

pub struct ParserState {
    current_loop: Option<()>,
    current_function: Option<()>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            current_loop: None,
            current_function: None,
        }
    }
}

pub struct Parser<'source> {
    lexer: Lexer<'source>,
    chunk: ast::Chunk,
    recursion_guard: Rc<()>,
    interner: Rc<DefaultInterner>,
    errors: Vec<SpannedError>,
    state: ParserState,
}

impl<'source> Parser<'source> {
    pub fn new(lexer: Lexer<'source>, interner: Rc<DefaultInterner>) -> Self {
        Self {
            lexer,
            interner,
            recursion_guard: Rc::new(()),
            errors: vec![],
            state: ParserState::new(),
            chunk: ast::Chunk::new(),
        }
    }

    pub fn previous(&self) -> &Token {
        self.lexer.previous()
    }

    pub fn current(&self) -> &Token {
        self.lexer.current()
    }

    pub fn is_at_end(&self) -> bool {
        use crate::internal::syntax::lexer::TokenKind::Tok_Eof;
        self.current().is(Tok_Eof)
    }

    pub fn is_end_of_block(&self) -> bool {
        use crate::internal::syntax::lexer::TokenKind::Kw_End;
        self.current().is(Kw_End)
    }

    pub fn bump(&mut self) {
        self.lexer.bump();
    }

    /*pub fn bump_if(&mut self, kind: TokenKind) {
        if self.current().is(kind) {
            self.bump();
        }
    }*/
}
