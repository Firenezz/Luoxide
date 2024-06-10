use logos::Lexer as LogosLexer;

use crate::internal::util::{from_digit, from_hex_digit, is_newline, is_space};

use super::{
    TokenKind, ASCII_BACKSPACE, ASCII_BELL, ASCII_FORM_FEED,
    ASCII_VERTICAL_TAB,
};

pub(crate) fn escape_unicode(string: impl AsRef<[u8]>) -> String {
    // TODO: Do correct string escaping error handling

    let mut save_string = String::with_capacity(32);
    let chars = String::from_utf8_lossy(string.as_ref()).to_string();
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
                    Some('z') => while chars_iter.next_if(|c| is_space(*c as u8)).is_some() {},

                    // \xXX
                    Some('x') => {
                        let mut char_byte = 0u8;
                        let mut count = 0;

                        for hex in chars_iter.by_ref().take(2).map(|c| from_hex_digit(c as u8)) {
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
                            .map(|c| from_hex_digit(c as u8))
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
                        let mut digit_byte = from_digit(c as u8)
                            .expect("the character should be a valid digit to enter this branch")
                            as u16;
                        let mut count = 1usize;

                        while let Some(&c) = chars_iter.peek() {
                            if count > 2 {
                                break;
                            }

                            let digit = match from_digit(c as u8) {
                                Some(digit) => digit,
                                None => break,
                            };

                            digit_byte = (digit_byte * 10) + digit as u16;
                            chars_iter.next();
                            count += 1;
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

pub(crate) fn skip_all_newlines(lexer: &mut LogosLexer<TokenKind>) -> usize {
    use logos::internal::LexerInternal;
    let mut count = 0;

    while let Some(char) = lexer.read::<u8>() {
        // ignore first newline
        match char {
            char if is_newline(char) => lexer.bump(1usize),
            _ => break,
        }
        count += 1
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unescape_string_correctly() {
        assert_eq!(escape_unicode("\\\\"), "\\".to_string());
        assert_eq!(escape_unicode("\\n"), "\n".to_string());
        assert_eq!(escape_unicode("\\t"), "\t".to_string());
        assert_eq!(escape_unicode("\\r"), "\r".to_string());
        assert_eq!(escape_unicode("\\\""), "\"".to_string());
        assert_eq!(escape_unicode("\\'"), "'".to_string());
        assert_eq!(
            escape_unicode("\\v"),
            (ASCII_VERTICAL_TAB as char).to_string()
        );
        assert_eq!(escape_unicode("\\f"), (ASCII_FORM_FEED as char).to_string());
        assert_eq!(escape_unicode("\\a"), (ASCII_BELL as char).to_string());
        assert_eq!(escape_unicode("\\b"), (ASCII_BACKSPACE as char).to_string());
        assert_eq!(escape_unicode("\\z\n\r skipped"), "skipped".to_string()); // skips the next whitespaces

        assert_eq!(escape_unicode("\\u{1234}"), "\u{1234}".to_string());
        assert_eq!(escape_unicode("\\x30"), "0".to_string());
        assert_eq!(escape_unicode("\\u{1D306}"), "𝌆".to_string());
        assert_eq!(escape_unicode("\\064"), "@".to_string());
        assert_eq!(escape_unicode("\\064g"), "@g".to_string());
        assert_eq!(escape_unicode("\\64\\t t"), "@\t t".to_string());

        assert_eq!(
            escape_unicode("\\97lo\\10\\04923\"\\u{1D306}\\\\"),
            "alo\n123\"𝌆\\".to_string()
        );
    }
}
