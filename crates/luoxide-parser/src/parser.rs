pub mod common;
pub mod error;
pub mod expression;
pub mod synchronization;

use tracing::{event, info_span, Instrument, Level};

use luoxide_ast::ast;
use luoxide_text::{size::TextSize, source::Source};

use crate::{error::ParseError, lexer::Lexer, token::Token};

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

    pub mode: Mode,
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    Panic
}

pub struct Info {
    source: String,
    line: TextSize,
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

pub fn compile_expression(text: &str) -> Result<ast::expressions::Expression, Vec<ParseError>> {
    let mut parser = Parser::new(text);

    let span = info_span!("compile_expression", ast_expression = tracing::field::Empty);
    //let _guard = span.enter();
    event!(Level::INFO, "starting expression compilation of \"{text}\"");
    let ast = parser.parse_expression().instrument(span).into_inner();

    event!(Level::INFO, "expression parsed");

    match ast {
        Ok(expr) => Ok(expr),
        Err(_) => {
            event!(Level::ERROR, "parsing ended with an error");
            Err(vec![])
        }
    }
}
