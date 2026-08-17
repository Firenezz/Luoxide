use luoxide_text::size::TextSize;
use tracing::{event, Level};

use crate::error::ParseError;
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser<'_> {
    /// Skips tokens until an expression synchronization point.
    ///
    /// Anchors are tokens that plausibly follow an expression or start a new
    /// construct: statement separators, block delimiters and keywords.
    pub(super) fn synchronize_expression(&mut self) -> Token {
        event!(Level::TRACE, "synchronizing parser (expression)");
        self.skip_until(Self::is_expression_sync_point);
        *self.current()
    }

    /// Skips tokens until a token that can start or delimit a statement.
    pub(super) fn synchronize_statement(&mut self) -> Token {
        event!(Level::TRACE, "synchronizing parser (statement)");
        self.skip_until(Self::is_statement_sync_point);
        *self.current()
    }

    /// Records `error` and advances so recovery cannot stall on the same token.
    ///
    /// `NestingTooDeep` is special: the failing token is usually a block opener
    /// (`do`, `if`, ...) which is also a sync point. Staying there would retry
    /// the same statement forever. Skip to a block terminator instead.
    pub(super) fn recover_statement(&mut self, error: ParseError) {
        let start = self.current_token().span.start;
        let nesting = error.is_nesting_too_deep();
        self.error_context.add_error(error);

        if nesting {
            event!(Level::TRACE, "recovering from nesting-too-deep (statement)");
            self.skip_until(Self::is_block_terminator);
            return;
        }

        self.synchronize_statement();
        self.ensure_progress(start, Self::is_block_terminator);
    }

    /// Same idea as [`recover_statement`], for expression lists (call args,
    /// table fields). Closers (`)`, `}`, `]`) are left in place so the caller
    /// can still match them.
    pub(super) fn recover_expression(&mut self, error: ParseError) {
        let start = self.current_token().span.start;
        let nesting = error.is_nesting_too_deep();
        self.error_context.add_error(error);

        if nesting {
            event!(
                Level::TRACE,
                "recovering from nesting-too-deep (expression)"
            );
            self.skip_until(Self::is_expression_sync_point);
            self.ensure_progress(start, Self::is_expression_closer);
            return;
        }

        self.synchronize_expression();
        self.ensure_progress(start, Self::is_expression_closer);
    }

    fn skip_until(&mut self, stop: fn(&TokenKind) -> bool) {
        while !stop(self.current_kind()) && !self.is_at_end() {
            self.bump();
        }
    }

    /// If recovery did not consume anything, skip one token so the caller
    /// cannot retry the same failing token. Closers / terminators are left
    /// alone so enclosing constructs can still finish.
    fn ensure_progress(&mut self, start: TextSize, leave_in_place: fn(&TokenKind) -> bool) {
        if self.current_token().span.start != start {
            return;
        }
        if self.is_at_end() || leave_in_place(self.current_kind()) {
            return;
        }
        self.bump();
    }

    pub(super) const fn is_block_terminator(token_kind: &TokenKind) -> bool {
        matches!(
            token_kind,
            token!(EOF) | token!(end) | token!(else) | token!(else if) | token!(until)
        )
    }

    const fn is_expression_closer(token_kind: &TokenKind) -> bool {
        matches!(
            token_kind,
            token!(EOF)
                | token!(")")
                | token!("}")
                | token!("]")
                | token!(",")
                | token!(";")
                | token!(end)
                | token!(else)
                | token!(else if)
                | token!(until)
                | token!(then)
        )
    }

    const fn is_expression_sync_point(token_kind: &TokenKind) -> bool {
        matches!(
            token_kind,
            token!(EOF)
                | token!(";")
                | token!(",")
                | token!("{")
                | token!("}")
                | token!("(")
                | token!(")")
                | token!(if)
                | token!(do)
                | token!(function)
                | token!(for)
                | token!(while)
                | token!(repeat)
                | token!(until)
                | token!(else)
                | token!(else if)
                | token!(then)
                | token!(end)
                | token!(return)
                | token!(local)
        )
    }

    const fn is_statement_sync_point(token_kind: &TokenKind) -> bool {
        matches!(
            token_kind,
            token!(EOF)
            | token!(";")
            | token!(end)
            | token!(else)
            | token!(else if)
            | token!(until)
            // statement starters, including `do` (safe because recover_statement
            // either skips to a terminator on nesting errors or bumps on stall)
            | token!(do)
            | token!(local)
            | token!(if)
            | token!(function)
            | token!(for)
            | token!(while)
            | token!(repeat)
            | token!(return)
            | token!(break)
            | token!(goto)
            | token!("::")
        )
    }
}
