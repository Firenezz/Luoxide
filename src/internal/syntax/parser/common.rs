use crate::internal::syntax::lexer::TokenKind;

use super::*;

#[allow(dead_code)] // TODO: remove this if not used
impl<'source> Parser<'source> {
    pub(super) fn test(&mut self, kind: TokenKind) -> bool {
        if self.current().kind == kind {
            return true;
        }
        false
    }

    pub(super) fn test_in(&self, kinds: &[TokenKind]) -> Option<&TokenKind> {
        if kinds.iter().any(|kind| self.current().is(kind)) {
            return Some(&self.current().kind);
        }
        None
    }

    pub(super) fn is_unary(&self) -> Option<&TokenKind> {
        self.test_in(&[
            TokenKind::Op_Minus,
            TokenKind::Kw_Not,
            TokenKind::Op_Len,
            TokenKind::Op_BitXor,
        ])
    }

    //pub(super) fn bump_if_in(&mut self, kinds: &[TokenKind]) -> Option<&TokenKind> {}
}
