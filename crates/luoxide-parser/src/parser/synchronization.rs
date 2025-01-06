use tracing::{event, Level};

use crate::{
    token::{Token, TokenKind},
    token_set::TokenSet,
};

use super::Parser;

impl Parser<'_> {
    /// General synchronization
    pub fn synchronize(&mut self, synchronize_points: TokenSet) {
        event!(Level::TRACE, "Synchronizing parser");
        while synchronize_points.contains(self.lexer.current().kind)
            && self.current_is_not(token!(EOF))
        {
            self.bump();
        }
    }

    pub(super) fn synchronize_expression(&mut self) -> Token {
        // Sync anchor for expressions are:
        // token!(EOF)
        // ";" if we lucky this is a good token to sync with
        // all block delimiters that start or end a block
        // function definitions
        // table constructors
        // return statements

        // use matches!() macros here because we know the set of token at compile time

        while !Self::is_expression_sync_point(self.current_kind()) {
            self.bump();
        }

        *self.current()
    }

    const fn is_expression_sync_point(token_kind: &TokenKind) -> bool {
        matches!(
            token_kind,
            token!(EOF)
            | token!(";")
            | token!("{")
            | token!("}")
            | token!("(")
            | token!(")")
            // block delimiters
            | token!(if)
            | token!(do)
            | token!(function)
            | token!(for)
            | token!(while)
            | token!(repeat)
            | token!(until)
            | token!(else)
            | token!(then)
            | token!(end)
        )
    }
}
