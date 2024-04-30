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

    pub(crate) fn escape_unicode(string: impl AsRef<[u8]>) -> String {
        let chars = String::from_utf8_lossy(string.as_ref()).to_string();
        // TODO: Do correct string escaping error handling

        let mut save_string = String::with_capacity(32);
        let mut chars_iter = chars.chars().peekable();

        while let Some(c) = chars_iter.next() {
            match c {
                '\\' => {
                    let special_char = chars_iter.next();
                    match special_char {
                        Some('\\') => {
                            save_string.push('\\');
                        }
                        Some('n') => {
                            save_string.push('\n');
                        }
                        Some('r') => {
                            save_string.push('\r');
                        }
                        Some('t') => {
                            save_string.push('\t');
                        }
                        Some('"') => {
                            save_string.push('"');
                        }
                        Some('\'') => {
                            save_string.push('\'');
                        }
                        Some('v') => {
                            save_string.push(ASCII_VERTICAL_TAB.into());
                        }
                        Some('f') => {
                            save_string.push(ASCII_FORM_FEED.into());
                        }
                        Some('a') => {
                            save_string.push(ASCII_BELL.into());
                        }
                        Some('b') => {
                            save_string.push(ASCII_BACKSPACE.into());
                        }
                        Some('z') => {
                            todo!("ignore the next whitespaces");
                        }

                        // \xXX
                        Some('x') => {
                            let mut char_byte = 0u8;
                            let mut count = 0;

                            for hex in chars_iter
                                .by_ref()
                                .take(2)
                                .map(|c| Self::from_hex_digit(c as u8))
                            {
                                if let Some(hex) = hex {
                                    char_byte = (char_byte << 4) | hex;
                                    count += 1;
                                } else {
                                    todo!("return malformed string");
                                }
                            }

                            if count != 2 {
                                todo!("return malformed string");
                            }

                            save_string.push(char_byte as char);
                        }

                        // \u{xxxxxxxx}
                        Some('u') => {
                            if chars_iter.next() != Some('{') {
                                todo!("return malformed string - invalid escape sequence \\u");
                            }
                            let mut char_byte = 0u32;

                            for hex in chars_iter
                                .by_ref()
                                .take_while(|c| *c != '}')
                                .filter(|c| *c != '_')
                                .map(|c| Self::from_hex_digit(c as u8))
                                .enumerate()
                            {
                                if hex.0 > 8 {
                                    todo!("return malformed string - invalid escape sequence \\u - missing '}}'");
                                }
                                if let Some(hex) = hex.1 {
                                    char_byte = (char_byte << 4) | hex as u32;
                                } else {
                                    todo!("return malformed string - invalid escape sequence \\u - not an hex digit");
                                }
                            }

                            save_string.push(
                                std::char::from_u32(char_byte).expect("Invalid unicode codepoint"),
                            );
                        }

                        // \ddd
                        Some(c) if c.is_ascii_digit() => {
                            let mut digit_byte = 0u16;
                            let mut count = Self::from_digit(c as u8)
                                .ok_or_else(|| todo!("return malformed string"))
                                .unwrap() as u16;

                            for digit in chars_iter
                                .by_ref()
                                .take(3)
                                .map(|c| Self::from_digit(c as u8))
                            {
                                if let Some(digit) = digit {
                                    digit_byte = (digit_byte * 10) + digit as u16;
                                    count += 1;
                                } else {
                                    todo!("return malformed string");
                                }
                            }

                            if count < 2 {
                                todo!("return malformed string");
                            }

                            save_string.push(
                                std::char::from_u32(digit_byte as u32)
                                    .expect("Invalid unicode codepoint"),
                            );
                        }

                        Some(_) => {
                            todo!("return malformed string - invalid escape sequence");
                        }
                        None => {
                            todo!("return malformed string - unexpected end of string");
                        }
                    }
                }
                _ => {
                    save_string.push(c);
                }
            }
        }

        save_string
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

    #[test]
    fn unescape_string_correctly() {
        assert_eq!(Parser::<'static>::escape_unicode("\\\\"), "\\".to_string());
        assert_eq!(Parser::<'static>::escape_unicode("\\n"), "\n".to_string());
        assert_eq!(Parser::<'static>::escape_unicode("\\t"), "\t".to_string());
        assert_eq!(Parser::<'static>::escape_unicode("\\r"), "\r".to_string());
        assert_eq!(Parser::<'static>::escape_unicode("\\\""), "\"".to_string());
        assert_eq!(Parser::<'static>::escape_unicode("\\'"), "'".to_string());
        assert_eq!(
            Parser::<'static>::escape_unicode("\\v"),
            (ASCII_VERTICAL_TAB as char).to_string()
        );
        assert_eq!(
            Parser::<'static>::escape_unicode("\\f"),
            (ASCII_FORM_FEED as char).to_string()
        );
        assert_eq!(
            Parser::<'static>::escape_unicode("\\a"),
            (ASCII_BELL as char).to_string()
        );
        assert_eq!(
            Parser::<'static>::escape_unicode("\\b"),
            (ASCII_BACKSPACE as char).to_string()
        );
        //assert_eq!(Parser::<'static>::escape_unicode("\\z"), "\0".to_string()); // skips the next whitespaces

        assert_eq!(
            Parser::<'static>::escape_unicode("\\u{1234}"),
            "\u{1234}".to_string()
        );
        assert_eq!(Parser::<'static>::escape_unicode("\\x30"), "0".to_string());
        assert_eq!(
            Parser::<'static>::escape_unicode("\\u{1D306}"),
            "𝌆".to_string()
        );
        assert_eq!(Parser::<'static>::escape_unicode("\\064"), "@".to_string());
    }
}
