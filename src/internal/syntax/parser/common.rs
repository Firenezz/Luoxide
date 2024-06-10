use crate::internal::syntax::lexer::TokenKind;

use super::*;

#[allow(dead_code)]
const ASCII_BELL: u8 = 0x07;
#[allow(dead_code)]
const ASCII_BACKSPACE: u8 = 0x08;
#[allow(dead_code)]
const ASCII_VERTICAL_TAB: u8 = 0x0b;
#[allow(dead_code)]
const ASCII_FORM_FEED: u8 = 0x0c;
#[allow(dead_code)]
const ASCII_ESCAPE: u8 = 0x1b;

#[allow(dead_code)] // TODO: remove this if not used
impl<'source> Parser<'source> {
    /// Returns true if the current token is of the given kind. Otherwise, returns false.
    ///
    /// Does not consume the token.
    ///
    /// Returns true if the current token is of the given kind. Otherwise, returns false.
    #[inline]
    pub(super) fn either<const N: usize>(&mut self, kinds: [TokenKind; N]) -> bool {
        kinds.iter().any(|kind| self.current().is(kind))
    }

    /// Must be one of the given kinds, otherwise error.
    ///
    /// Does not consume the token.
    ///
    /// Returns true if the current token is of the given kind. Otherwise, returns false.
    #[inline]
    pub(super) fn either_in(&mut self, kinds: impl IntoIterator<Item = TokenKind>) -> bool {
        kinds.into_iter().any(|kind| self.current().is(kind))
    }

    /// Must be one of the given kinds, otherwise error.
    ///
    /// Does not consume the token.
    ///
    /// Returns true if the current token is of the given kind. Otherwise, returns false.
    #[inline]
    pub(super) fn expect_either<const N: usize>(
        &mut self,
        kinds: [TokenKind; N],
    ) -> ParseResult<&Token> {
        if !self.either(kinds) {
            return Fail;
        }

        Success(self.current())
    }

    /// Success if the current token is of the given kind. Otherwise, error.
    ///
    /// Does not consume the token.
    ///
    /// Return [`ParseResult::Success`] if the current token is of the given kind. Otherwise, [`ParseResult::Fail`].
    #[inline]
    pub(super) fn expect_current(&self, kind: TokenKind) -> ParseResult<()> {
        if self.current().is(kind) {
            return Success(());
        }

        Fail
    }

    /// Success if the sequence of tokens is found. Otherwise, error.
    ///
    /// Does consume the tokens except the last one.
    #[inline]
    pub(super) fn expect_sequence<const N: usize>(
        &mut self,
        kinds: [TokenKind; N],
    ) -> ParseResult<()> {
        for kind in kinds {
            if !self.current().is(kind) {
                return Fail;
            }
            self.advance();
        }

        Success(())
    }

    #[inline]
    pub(super) fn is_unary(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Minus | TokenKind::Not | TokenKind::Pound | TokenKind::BitXor
        )
    }

    pub(super) fn is_binary(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Add
                | TokenKind::Minus
                | TokenKind::Mul
                | TokenKind::Div
                | TokenKind::Mod
                | TokenKind::Pow
                | TokenKind::Concat
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterEqual
                | TokenKind::BitAnd
                | TokenKind::BitOr
                | TokenKind::Dot
                | TokenKind::Equal
                | TokenKind::And
                | TokenKind::Or
        )
    }

    pub(crate) fn parse_variable(&mut self) -> Result<ast::StatementKind, ()> {
        todo!("parse_variable")
    }

    fn from_digit(c: u8) -> Option<u8> {
        if c.is_ascii_digit() {
            Some(c - b'0')
        } else {
            None
        }
    }

    fn from_hex_digit(c: u8) -> Option<u8> {
        if c.is_ascii_digit() {
            Some(c - b'0')
        } else if matches!(c, b'a'..=b'f') {
            Some(10 + c - b'a')
        } else if matches!(c, b'A'..=b'F') {
            Some(10 + c - b'A')
        } else {
            None
        }
    }

    //pub(super) fn bump_if_in(&mut self, kinds: &[TokenKind]) -> Option<&TokenKind> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_digit() {
        (b'1'..=b'9').for_each(|c| assert_eq!(Parser::from_digit(c), Some(c - b'0')));
    }

    #[test]
    fn test_from_hex_digit() {
        (b'1'..=b'9').for_each(|c| assert_eq!(Parser::from_hex_digit(c), Some(c - b'0')));
        (b'a'..=b'f').for_each(|c| assert_eq!(Parser::from_hex_digit(c), Some(10 + c - b'a')));
        (b'A'..=b'F').for_each(|c| assert_eq!(Parser::from_hex_digit(c), Some(10 + c - b'A')));
    }
}
