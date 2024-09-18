pub mod error;
pub mod expression;
pub mod common;

use luoxide_text::source::Source;

use crate::{lexer::Lexer, token::Token};

pub struct Parser<'source> {
    pub source: Source<'source>,
    pub lexer: Lexer<'source>,

    pub error_context: error::ErrorContext,

    state: State,
}

#[derive(Default, Debug)]
pub struct State {
    token: Token,

    diagnostics: Vec<()>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source: Source::new(source),
            lexer: Lexer::new(source),
            error_context: error::ErrorContext::new(),

            state: State::default(),
        }
    }

    pub fn parse(&mut self) {}
}

// Helper functions
impl Parser<'_> {
    pub fn current_token(&self) -> &Token {
        self.lexer.current()
    }
}

pub fn compile_expression(text: &str) {
    let mut parser = Parser::new(text);

    let ast = parser.parse_expression();

    if let Err(err) = ast {
        dbg!(parser.error_context.errors);
    }

    dbg!(ast);
}
