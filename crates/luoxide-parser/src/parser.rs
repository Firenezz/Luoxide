pub mod common;
pub mod error;
pub mod expression;
pub mod statement;
mod strings;
pub mod synchronization;
pub mod table;
mod trace;

use ecow::EcoString;
use tracing::{Level, event, info_span};

use luoxide_text::{range::TextSpan, source::Source};

use crate::ast;
use crate::outcome::Outcome;
use crate::{error::ParseError, lexer::Lexer, token::Token};

/// Maximum nesting depth of expressions/statements before the parser reports
/// [`NestingTooDeep`](crate::error::ParseErrorKind::NestingTooDeep) instead of
/// overflowing the stack (compare Lua's `LUAI_MAXCCALLS`).
pub const MAX_NESTING_DEPTH: u32 = 200;

pub struct Parser<'source> {
    pub source: Source<'source>,
    pub lexer: Lexer<'source>,

    pub error_context: error::ErrorContext,

    /// Current recursion depth, guarded by [`Parser::with_depth`].
    depth: u32,
    /// Named productions currently on the stack (`chunk/block/statement/...`).
    frames: Vec<&'static str>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source: Source::new(source),
            lexer: Lexer::new(source),
            error_context: error::ErrorContext::new(),

            depth: 0,
            frames: Vec::new(),
        }
    }

    /// Runs `f` one nesting level deeper, erroring out instead of blowing the
    /// stack on pathological inputs like `((((((...`.
    pub(crate) fn with_depth<T>(
        &mut self,
        name: &'static str,
        at: TextSpan,
        f: impl FnOnce(&mut Self) -> crate::error::Result<T>,
    ) -> crate::error::Result<T> {
        if self.depth >= MAX_NESTING_DEPTH {
            return Err(self.nesting_too_deep(Some(at)));
        }
        self.depth += 1;
        let result = self.with_frame(name, f);
        self.depth -= 1;
        result
    }
}

// Helper functions
impl Parser<'_> {
    #[inline]
    pub fn current_token(&self) -> &Token {
        self.lexer.current()
    }

    #[inline]
    pub fn previous_token(&self) -> &Token {
        self.lexer.previous()
    }

    #[inline]
    pub fn current_lexeme(&self) -> EcoString {
        EcoString::from(self.get_lexeme(self.current_token()))
    }

    #[inline]
    pub fn previous_lexeme(&self) -> EcoString {
        EcoString::from(self.get_lexeme(self.previous_token()))
    }
}

/// Parses a whole source file.
///
/// Always produces a [`Chunk`](ast::Chunk): erroneous regions are represented
/// by `Error` nodes in the tree and described by the returned diagnostics.
pub fn compile_chunk(text: &str) -> Outcome<ast::Chunk, Vec<ParseError>> {
    let mut parser = Parser::new(text);

    let span = info_span!("compile_chunk");
    let _guard = span.enter();
    event!(Level::INFO, "starting chunk compilation");

    let chunk = parser.parse_chunk();

    let errors = parser.error_context.take_errors();
    if errors.is_empty() {
        Outcome::Ok(chunk)
    } else {
        event!(Level::ERROR, "parsing produced {} error(s)", errors.len());
        Outcome::PartialFailure(chunk, errors)
    }
}

/// Parses a single expression (mostly useful for tests and tooling).
pub fn compile_expression(text: &str) -> Outcome<ast::Expression, Vec<ParseError>> {
    let mut parser = Parser::new(text);

    let span = info_span!("compile_expression");
    let _guard = span.enter();
    event!(Level::INFO, "starting expression compilation of {:?}", text);

    let expression = match parser.parse_expression() {
        Ok(expression) => {
            if !parser.is_at_end() {
                let current = *parser.current_token();
                let error =
                    parser.unexpected_token([token!(EOF)], &current.kind, Some(current.span));
                parser.record_error(error);
            }
            expression
        }
        Err(error) => {
            let at = error.at.unwrap_or(parser.current_token().span);
            parser.record_error(error);
            ast::Expression::error(at)
        }
    };

    let errors = parser.error_context.take_errors();
    if errors.is_empty() {
        Outcome::Ok(expression)
    } else {
        event!(Level::ERROR, "parsing produced {} error(s)", errors.len());
        Outcome::PartialFailure(expression, errors)
    }
}
