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

    pub(super) fn is_unary(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Op_Minus | TokenKind::Kw_Not | TokenKind::Op_Len | TokenKind::Op_BitXor
        )
    }

    pub(super) fn is_binary(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Op_Add
                | TokenKind::Op_Minus
                | TokenKind::Op_Mul
                | TokenKind::Op_Div
                | TokenKind::Op_Mod
                | TokenKind::Op_Pow
                | TokenKind::Op_Concat
                | TokenKind::Op_NotEqual
                | TokenKind::Op_LessThan
                | TokenKind::Op_LessEqual
                | TokenKind::Op_GreaterThan
                | TokenKind::Op_GreaterEqual
                | TokenKind::Op_BitAnd
                | TokenKind::Op_BitOr
                | TokenKind::Op_Dot
                | TokenKind::Op_Equal
                | TokenKind::Kw_And
                | TokenKind::Kw_Or
        )
    }

    //pub(super) fn bump_if_in(&mut self, kinds: &[TokenKind]) -> Option<&TokenKind> {}
}
