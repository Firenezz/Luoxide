use super::syntax::lexer::{ASCII_FORM_FEED, ASCII_VERTICAL_TAB};

pub(crate) fn is_newline(c: u8) -> bool {
    c == b'\n' || c == b'\r'
}

pub(crate) fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == ASCII_VERTICAL_TAB || c == ASCII_FORM_FEED || is_newline(c)
}

pub(crate) fn utf8_char_width(first_byte: u8) -> usize {
    match first_byte {
        0x00..=0x7F => 1,
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => panic!("Invalid UTF-8 character"),
    }
}

pub(crate) fn from_digit(c: u8) -> Option<u8> {
    if c.is_ascii_digit() {
        Some(c - b'0')
    } else {
        None
    }
}

pub(crate) fn is_digit(c: u8) -> bool {
    from_digit(c).is_some()
}

pub(crate) fn from_hex_digit(c: u8) -> Option<u8> {
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

pub(crate) fn is_hex_digit(c: u8) -> bool {
    from_hex_digit(c).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_digit() {
        (b'1'..=b'9').for_each(|c| assert_eq!(from_digit(c), Some(c - b'0')));
    }

    #[test]
    fn test_from_hex_digit() {
        (b'1'..=b'9').for_each(|c| assert_eq!(from_hex_digit(c), Some(c - b'0')));
        (b'a'..=b'f').for_each(|c| assert_eq!(from_hex_digit(c), Some(10 + c - b'a')));
        (b'A'..=b'F').for_each(|c| assert_eq!(from_hex_digit(c), Some(10 + c - b'A')));
    }
}
