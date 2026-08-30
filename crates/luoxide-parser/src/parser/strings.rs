//! String literal processing: turning raw lexemes (quotes and escape
//! sequences included) into their runtime contents.

use ecow::EcoString;

/// Unescapes a short string literal (`"..."` or `'...'`).
///
/// Returns the contents and, when an invalid escape sequence was found, the
/// byte offset of the first offending backslash inside `raw`. Invalid escapes
/// are skipped so parsing can continue with a best-effort value.
pub(super) fn unescape_short(raw: &str) -> (EcoString, Option<usize>) {
    // Strip the surrounding quotes; the lexer guarantees they are present.
    let content = &raw[1..raw.len() - 1];

    if !content.contains('\\') {
        return (EcoString::from(content), None);
    }

    let mut out = String::with_capacity(content.len());
    let mut first_bad = None;
    let bad = |offset: usize, first_bad: &mut Option<usize>| {
        if first_bad.is_none() {
            // +1 to account for the stripped opening quote
            *first_bad = Some(offset + 1);
        }
    };

    let mut chars = content.char_indices().peekable();
    while let Some((offset, c)) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }

        let Some(&(_, escape)) = chars.peek() else {
            // Trailing lone backslash; the lexer normally prevents this.
            bad(offset, &mut first_bad);
            break;
        };
        chars.next();

        match escape {
            'a' => out.push('\x07'),
            'b' => out.push('\x08'),
            'f' => out.push('\x0C'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            'v' => out.push('\x0B'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            '\'' => out.push('\''),
            // `\` + real newline is a line continuation (not part of the value).
            '\n' => {}
            '\r' => {
                if matches!(chars.peek(), Some(&(_, '\n'))) {
                    chars.next();
                }
            }
            // `\xXX`: exactly two hex digits.
            'x' => {
                let mut value = 0u32;
                let mut digits = 0;
                while digits < 2 {
                    match chars.peek() {
                        Some(&(_, d)) if d.is_ascii_hexdigit() => {
                            value = value * 16 + d.to_digit(16).unwrap();
                            chars.next();
                            digits += 1;
                        }
                        _ => break,
                    }
                }
                if digits == 2 {
                    out.push(value as u8 as char);
                } else {
                    bad(offset, &mut first_bad);
                }
            }
            // `\ddd`: one to three decimal digits, value must fit a byte.
            '0'..='9' => {
                let mut value = escape.to_digit(10).unwrap();
                let mut digits = 1;
                while digits < 3 {
                    match chars.peek() {
                        Some(&(_, d)) if d.is_ascii_digit() => {
                            value = value * 10 + d.to_digit(10).unwrap();
                            chars.next();
                            digits += 1;
                        }
                        _ => break,
                    }
                }
                if value <= 255 {
                    out.push(value as u8 as char);
                } else {
                    bad(offset, &mut first_bad);
                }
            }
            // `\u{XXX}`: code point up to 0x7FFFFFFF (Lua). Values outside
            // Unicode scalars cannot live in a Rust `char`; those become U+FFFD.
            'u' => {
                let mut ok = false;
                if matches!(chars.peek(), Some(&(_, '{'))) {
                    chars.next();
                    let mut value = 0u64;
                    let mut digits = 0;
                    while let Some(&(_, d)) = chars.peek() {
                        if d.is_ascii_hexdigit() {
                            value = value.saturating_mul(16) + u64::from(d.to_digit(16).unwrap());
                            chars.next();
                            digits += 1;
                        } else {
                            break;
                        }
                    }
                    if matches!(chars.peek(), Some(&(_, '}'))) && digits > 0 && value <= 0x7FFF_FFFF
                    {
                        chars.next();
                        match char::from_u32(value as u32) {
                            Some(c) => out.push(c),
                            None => out.push('\u{FFFD}'),
                        }
                        ok = true;
                    }
                }
                if !ok {
                    bad(offset, &mut first_bad);
                }
            }
            // `\z`: skip following whitespace (for wrapping long strings).
            'z' => {
                while matches!(chars.peek(), Some(&(_, c)) if c.is_ascii_whitespace()) {
                    chars.next();
                }
            }
            _ => bad(offset, &mut first_bad),
        }
    }

    (EcoString::from(out), first_bad)
}

/// Extracts the contents of a long string literal (`[[...]]`, `[=[...]=]`).
///
/// No escape sequences are processed; a leading newline directly after the
/// opening bracket is skipped, per the Lua manual.
pub(super) fn unescape_long(raw: &str) -> EcoString {
    // `[=*[` prefix: level is the number of `=` characters.
    let level = raw[1..].bytes().take_while(|&b| b == b'=').count();
    let delimiter = level + 2;

    let mut content = &raw[delimiter..raw.len() - delimiter];
    if let Some(stripped) = content
        .strip_prefix("\r\n")
        .or_else(|| content.strip_prefix("\n\r"))
        .or_else(|| content.strip_prefix('\n'))
        .or_else(|| content.strip_prefix('\r'))
    {
        content = stripped;
    }

    EcoString::from(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_string() {
        assert_eq!(unescape_short(r#""hello""#), ("hello".into(), None));
    }

    #[test]
    fn simple_escapes() {
        assert_eq!(unescape_short(r#""a\nb\tc""#), ("a\nb\tc".into(), None));
        assert_eq!(unescape_short(r#"'don\'t'"#), ("don't".into(), None));
    }

    #[test]
    fn numeric_escapes() {
        assert_eq!(unescape_short(r#""\65\66""#), ("AB".into(), None));
        assert_eq!(unescape_short(r#""\x41""#), ("A".into(), None));
        assert_eq!(unescape_short(r#""\u{48}i""#), ("Hi".into(), None));
        assert_eq!(unescape_short(r#""\u{1FFFFF}""#), ("\u{FFFD}".into(), None));
        assert_eq!(unescape_short("\"alo\\\nalo\""), ("aloalo".into(), None));
    }

    #[test]
    fn invalid_escape_reports_offset() {
        let (content, bad) = unescape_short(r#""a\qb""#);
        assert_eq!(content, "ab");
        assert_eq!(bad, Some(2));
    }

    #[test]
    fn long_string() {
        assert_eq!(unescape_long("[[hello]]"), "hello");
        assert_eq!(unescape_long("[==[a]=]b]==]"), "a]=]b");
        assert_eq!(unescape_long("[[\nfirst line]]"), "first line");
    }
}
