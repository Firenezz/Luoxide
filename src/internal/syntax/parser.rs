pub mod chunk;
pub mod common;
mod error;
mod expressions;
pub mod precedence;
mod statement;

use crate::error::SpannedError;

use std::rc::Rc;

use crate::{intern::DefaultInterner, internal::syntax::lexer::Lexer};

use super::{
    ast,
    lexer::{Token, TokenKind},
    SyntaxError,
};

#[allow(dead_code)]
const MAX_RECURSION: usize = 200;

#[allow(dead_code)]
const MIN_PRIORITY: u8 = 0;

pub fn parse_chunk<Source: AsRef<str>>(source: Source) -> Result<ast::Chunk, SyntaxError> {
    let interner = Rc::from(DefaultInterner::default());

    let lexer = Lexer::new(source.as_ref(), interner.clone());
    let mut parser = Parser::new(lexer, interner.clone());
    let _ = parser.parse();
    todo!("parse")
}

#[allow(clippy::result_unit_err)] // TODO: Remove this
pub fn parse_expression<Source: AsRef<str>>(source: Source) -> Result<ast::Expression, ()> {
    let interner = Rc::from(DefaultInterner::default());

    let lexer = Lexer::new(source.as_ref(), interner.clone());
    let mut parser = Parser::new(lexer, interner.clone());
    Ok(parser.parse_expression().unwrap())
}

#[allow(dead_code)] // TODO: remove this after ast is finished
pub struct ParserState {
    interner: Rc<DefaultInterner>,
    current_loop: Option<()>,
    current_function: Option<()>,
}

#[allow(dead_code)] // TODO: remove this after ast is finished
impl ParserState {
    pub fn new() -> Self {
        Self {
            interner: Rc::from(DefaultInterner::default()),
            current_loop: None,
            current_function: None,
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

    pub fn bump_by<const N: usize>(&mut self) {
        for _ in 0..N {
            self.lexer.bump();
            if self.is_at_end() {
                break;
            }
        }
    }

    /*pub fn take<const N: usize>(&mut self) -> [Option<&Token>; N] {
        let mut result = [None; N];
        for i in 0..N {
            self.bump();
            if self.is_at_end() {
                break;
            }

            result[i] = Some(self.current());

        }

        result
    }*/

    pub fn bump_if(&mut self, kind: TokenKind) -> bool {
        if self.current().is(kind) {
            self.bump();
            return true;
        }
        false
    }
}

/*impl<'src> Iterator for Parser<'src> {
    type Item = &'src Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.bump();

        if self.is_at_end() {
            return None;
        }

        Some(self.current())
    }

}*/

#[cfg(test)]
mod tests;
