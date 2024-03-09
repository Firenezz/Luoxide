use super::*;

use hexfloat2::HexFloat64;
use logos::{FilterResult, Lexer as LogosLexer};

pub(super) fn newline_callback(lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>) -> usize {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    lex.extras.0
}

pub(super) fn interner_callback(lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>) -> Rc<[u8]> {
    lex.extras
        .2
        .intern(lex.slice()[1..lex.slice().len() - 1].as_bytes())
}

pub(super) fn interner_identifier_callback(lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>) -> Rc<[u8]> {
    lex.extras.2.intern(lex.slice().as_bytes())
}

// Read a [=*[...]=*] sequence with matching numbers of '='. return Emit(Rc<[u8]>)
pub(super) fn long_string_callback(
    lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>,
) -> FilterResult<Rc<[u8]>, LexingError> {
    use logos::internal::LexerInternal;

    // for multi lines keep track of the "=" and the number of lines
    // example [===[ ... ]===]

    // For starter we count the number of "=" if there is any and add to stack

    let mut lines = lex.extras.1;
    let start_slice = lex.slice();

    // the regex should filter out the bad starts so the number of "=" should be the lenght of the slice - 4
    let number_equals = start_slice.len() - 2;

    let count_equals = |lex: &mut LogosLexer<'_, TokenKind<Rc<[u8]>>>, ends_with| {
        let mut count = 0;
        while let Some(comment_char) = lex.read_at::<u8>(count) {
            if comment_char == ends_with {
                return Ok(count);
            } else if comment_char == b'=' {
                count += 1;
            } else {
                return Err((count, false));
            }
        }
        Err((count, true))
    };

    // ignore first newline
    if let Some(chars) = lex.read::<&[u8; 2usize]>() {
        match chars[0] {
            b'\n' | b'\r' => {
                let _ = match chars[1] {
                    b'\n' | b'\r' => lex.skip(2usize),
                    _ => lex.skip(1usize),
                };
            }
            _ => (),
        }
    }

    // now we can loop until the stack is empty

    // only the first number of equals is used the [=*[ other than the start number of equals are ignored

    loop {
        // get characters one by one until we find the end or an end of comment "]...]"
        match lex.read::<u8>() {
            Some(string_string) => {
                match string_string {
                    // all escape sequences are ignored
                    b']' => {
                        lex.bump(1usize);
                        // new scope might be ending
                        match count_equals(lex, b']') {
                            Ok(count) => {
                                lex.bump(count + 1usize);
                                if count == number_equals {
                                    break;
                                }
                            }
                            Err(result) => {
                                match result {
                                    (_, true) => {
                                        return FilterResult::Error(
                                            LexingError::UnterminatedMultiLineComment,
                                        )
                                    } // TODO: bump the lexer to the end of the comment
                                    (count, false) => {
                                        // end of string token didnt match ]...]
                                        lex.bump(count + 1usize);
                                    }
                                }
                            }
                        }
                    }
                    b'\n' | b'\r' => {
                        lines += 1;
                        lex.bump(1usize);
                    }
                    any_char => lex.bump(utf8_char_width(any_char)),
                }
            }
            None => return FilterResult::Error(LexingError::UnterminatedMultiLineComment),
        }
    }

    lex.extras.0 = lines;
    lex.extras.1 = lex.span().end;

    // trim the start and end
    let slice = lex.slice();
    let slice = &slice[2 + number_equals..(slice.len() - (2 + number_equals))];
    FilterResult::Emit(lex.extras.2.intern(slice.as_bytes()))
    //FilterResult::Skip
}

pub(super) fn multiline_comment_callback(
    lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>,
) -> FilterResult<Rc<[u8]>, LexingError> {
    use logos::internal::LexerInternal;
    match lex.read::<&[u8; 2usize]>() {
        Some(b"--") => {
            // we have a multi line comment
            lex.bump(2usize);
            long_string_callback(lex)
        }
        Some(_chars) => FilterResult::Error(LexingError::UnterminatedMultiLineComment),
        None => FilterResult::Error(LexingError::InvalidLongStringDelimiter),
    }
}

fn utf8_char_width(first_byte: u8) -> usize {
    match first_byte {
        0x00..=0x7F => 1,
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => panic!("Invalid UTF-8 character"),
    }
}

pub(super) fn hex_to_integer(
    lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>,
) -> FilterResult<i64, LexingError> {
    let slice = lex.slice();
    match i64::from_str_radix(&slice[2..], 16) {
        Ok(int) => FilterResult::Emit(int),
        Err(err) => FilterResult::Error(err.into()),
    }
}

pub(super) fn hex_to_float(
    lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>,
) -> FilterResult<f64, LexingError> {
    let slice = lex.slice();

    let float: HexFloat64 = slice.parse().unwrap();
    FilterResult::Emit(float.0)
}
