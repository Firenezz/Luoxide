//! Parser action tracer: depth, production path, and cursor, logged via `tracing`.
//!
//! Two independent targets so you can pick a layer without the other:
//!
//! - [`SHALLOW`] (`DEBUG`): enter/leave, mismatch, error, recover, sync
//! - [`DEEP`] (`TRACE`): every consumed token (`eat`); `name` + `depth` only, not `frames`
//!
//! ```text
//! RUST_LOG=luoxide_parser::parse::shallow=debug
//! RUST_LOG=luoxide_parser::parse::deep=trace
//! RUST_LOG=luoxide_parser::parse::shallow=debug,luoxide_parser::parse::deep=trace
//! ```
//!
//! Snapshot tests should leave `RUST_LOG` unset.

use tracing::{Level, debug_span, event};

use crate::error::{ErrorKind, ParseError};
use crate::token::{Token, TokenKind};

use super::Parser;

/// Production stack, mismatches, and recovery. Filter with
/// `luoxide_parser::parse::shallow=debug`.
pub const SHALLOW: &str = "luoxide_parser::parse::shallow";

/// Per-token `eat` events. Filter with `luoxide_parser::parse::deep=trace`.
pub const DEEP: &str = "luoxide_parser::parse::deep";

impl Parser<'_> {
    fn current_frame(&self) -> &'static str {
        self.frames.last().copied().unwrap_or("parser")
    }

    fn emit(
        &self,
        action: &'static str,
        expected: Option<TokenKind>,
        skipped: Option<u32>,
        error: Option<&str>,
    ) {
        let current = self.current_token();
        let lexeme = self.get_lexeme(current);
        let prev = self.get_lexeme(self.previous_token());
        let expected = expected.map(TokenKind::as_lua);
        let name = self.current_frame();
        event!(
            target: SHALLOW,
            Level::DEBUG,
            depth = self.depth,
            frames = ?self.frames,
            action,
            token = %current.kind,
            lexeme,
            at = %current.span,
            prev,
            expected,
            skipped,
            error,
            name,
            "{action}"
        );
    }

    /// Token cursor only: production `name` + `depth`, not the full frame stack.
    pub(super) fn trace_eat(&self) {
        let current = self.current_token();
        event!(
            target: DEEP,
            Level::TRACE,
            depth = self.depth,
            name = self.current_frame(),
            token = %current.kind,
            lexeme = self.get_lexeme(current),
            at = %current.span,
            prev = self.get_lexeme(self.previous_token()),
            "eat"
        );
    }

    pub(super) fn trace_enter_leave(&self, action: &'static str) {
        self.emit(action, None, None, None);
    }

    pub(super) fn trace_mismatch(&self, expected: TokenKind) {
        self.emit("mismatch", Some(expected), None, None);
    }

    pub(super) fn trace_mismatch_any(&self) {
        self.emit("mismatch", None, None, None);
    }

    pub(super) fn trace_error(&self, error: &ParseError) {
        let label;
        let message = match &error.error {
            ErrorKind::ParserError { error_kind } => {
                label = error_kind.to_string();
                label.as_str()
            }
            ErrorKind::LexerError => "lexer error",
            ErrorKind::UnknownError(_) => "unknown error",
        };
        self.emit("error", None, None, Some(message));
    }

    pub(super) fn trace_sync(&self, skipped: u32, from: &Token) {
        event!(
            target: SHALLOW,
            Level::DEBUG,
            depth = self.depth,
            name = self.current_frame(),
            frames = ?self.frames,
            action = "sync",
            skipped,
            from = %from.span,
            token = %self.current_token().kind,
            lexeme = self.get_lexeme(self.current_token()),
            at = %self.current_token().span,
            "sync"
        );
    }

    pub(super) fn trace_recover(&self, kind: &'static str) {
        self.emit("recover", None, None, Some(kind));
    }

    pub(super) fn record_error(&mut self, error: ParseError) {
        self.trace_error(&error);
        self.error_context.add_error(error);
    }

    /// Named production frame (does not count toward [`super::MAX_NESTING_DEPTH`]).
    pub(crate) fn with_frame<T>(
        &mut self,
        name: &'static str,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.frames.push(name);
        // Anonymous `parse` span for subscriber nesting. Cursor and production
        // name live on the action event so parent spans stay quiet.
        let span = debug_span!("parse");
        let _guard = span.enter();
        self.trace_enter_leave("enter");
        let result = f(self);
        self.trace_enter_leave("leave");
        self.frames.pop();
        result
    }

    /// Consume the current token without an `eat` trace (used while skipping).
    pub(super) fn bump_untraced(&mut self) {
        self.lexer.bump();
    }
}
