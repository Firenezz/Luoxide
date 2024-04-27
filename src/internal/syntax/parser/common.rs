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

    pub(crate) fn escape_unicode(&self, string: impl AsRef<[u8]>) -> String {
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
                        Some('x') => {
                            let mut hex_string = String::with_capacity(2);
                            for hex in chars_iter.by_ref() {
                                if hex_string.len() > 2 {
                                    break;
                                }
                                if hex.is_ascii_hexdigit() {
                                    hex_string.push(hex);
                                }
                            }

                            save_string.push(u8::from_str_radix(&hex_string, 16).unwrap().into());
                        }
                        Some('u') => {
                            if chars_iter.peek() != Some(&'{') {
                                save_string.push(c);
                                save_string.push(special_char.unwrap());
                                continue;
                            }
                            let mut hex_string = String::with_capacity(5);
                            for hex in chars_iter.by_ref() {
                                if hex.is_ascii_hexdigit() {
                                    hex_string.push(hex);
                                }

                                // the sequence should end with '}'
                            }

                            // push the unicode equivalent of hex_string

                            save_string.push(std::char::from_u32(u32::from_str_radix(&hex_string, 16).unwrap()).expect("Invalid unicode codepoint - this should be handled with the lexer"));
                        }

                        Some(char) => {
                            // check if it's a number for \ddd
                            if char.is_ascii_digit() {
                                // read 3 digits
                                let mut number = String::with_capacity(3);
                                for digit in chars_iter.by_ref() {
                                    if number.len() > 3 {
                                        break;
                                    }
                                    if digit.is_ascii_digit() {
                                        number.push(digit);
                                    }
                                }
                            }
                            save_string.push(char);
                        }
                        None => {
                            todo!("return malformed string");
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

    //pub(super) fn bump_if_in(&mut self, kinds: &[TokenKind]) -> Option<&TokenKind> {}
}
