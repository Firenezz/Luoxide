use crate::internal::syntax::lexer::TokenKind;

use super::*;

impl<'parser> Parser<'parser> {
    pub(super) fn test_next(&mut self, kind: TokenKind) -> bool {
        if self.current().kind == kind {
            return true;
        }
        false
    }

    pub(super) fn test_next_in(&mut self, kinds: &[TokenKind]) -> Option<TokenKind> {
        if kinds.contains(&self.current().kind) {
            return Some(self.current().kind);
        }
        None
    }
}
