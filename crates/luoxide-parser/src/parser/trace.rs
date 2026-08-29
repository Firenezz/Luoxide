//! `tracing` events for the parse walk.
//!
//! | Target | Level | Events |
//! | --- | --- | --- |
//! | [`SHALLOW`] | `DEBUG` | enter, leave, mismatch, error, recover, sync |
//! | [`DEEP`] | `TRACE` | token consume (`eat`) |
//!
//! [`DEEP`] does not include production enter/leave. [`SHALLOW`] does not
//! include `eat`.

use tracing::{Level, debug_span, event};

use crate::error::ParseError;
use crate::token::{Token, TokenKind};

use super::Parser;

/// Production stack, mismatches, and recovery (`DEBUG`).
pub const SHALLOW: &str = "luoxide_parser::parse::shallow";

/// Per-token `eat` events (`TRACE`).
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

    /// `eat` event: production `name` and `depth` only.
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
        let label = error.kind.to_string();
        self.emit("error", None, None, Some(label.as_str()));
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

    /// Named production frame (not counted toward [`super::MAX_NESTING_DEPTH`]).
    pub(crate) fn with_frame<T>(
        &mut self,
        name: &'static str,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.frames.push(name);
        let span = debug_span!("parse");
        let _guard = span.enter();
        self.trace_enter_leave("enter");
        let result = f(self);
        self.trace_enter_leave("leave");
        self.frames.pop();
        result
    }

    /// Consume the current token without an `eat` event.
    pub(super) fn bump_untraced(&mut self) {
        self.lexer.bump();
    }
}
