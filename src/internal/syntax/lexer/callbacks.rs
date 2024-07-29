use self::string::escape_unicode;

use super::*;

use logos::{FilterResult, Lexer as LogosLexer};

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

pub(super) fn newline_callback(lex: &mut LogosLexer<TokenKind>) -> usize {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    lex.extras.0
}

pub(super) fn interner_callback(lex: &mut LogosLexer<TokenKind>) -> Rc<[u8]> {
    lex.extras.2.intern(escape_unicode(
        lex.slice()[1..lex.slice().len() - 1].as_bytes(),
    ))
}

pub(super) fn interner_identifier_callback(lex: &mut LogosLexer<TokenKind>) -> Rc<[u8]> {
    lex.extras.2.intern(lex.slice().as_bytes())
}

fn mark_current_line(lex: &mut LogosLexer<TokenKind>) {
    lex.extras.3 = lex.extras.0;
    lex.extras.4 = lex.extras.1;
}

// Read a [=*[...]=*] sequence with matching numbers of '='. return Emit(Rc<[u8]>)
pub(super) fn long_string_callback(
    lex: &mut LogosLexer<TokenKind>,
) -> FilterResult<Rc<[u8]>, LexingError> {
    use logos::internal::LexerInternal;

    // for multi lines keep track of the "=" and the number of lines
    // example [===[ ... ]===]

    // For starter we count the number of "=" if there is any and add to stack

    mark_current_line(lex);

    let mut lines = lex.extras.1;
    let start_slice = lex.slice();

    // the regex should filter out the bad starts so the number of "=" should be the lenght of the slice - 4
    let number_equals = start_slice.len() - 2;

    let count_equals = |lex: &mut LogosLexer<'_, TokenKind>, ends_with| {
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

    let skip_newlines = string::skip_all_newlines(lex);

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
                    any_char => lex.bump(crate::internal::util::utf8_char_width(any_char)),
                }
            }
            None => return FilterResult::Error(LexingError::UnterminatedMultiLineComment),
        }
    }

    lex.extras.0 = lines;
    lex.extras.1 = lex.span().end;

    // trim the start and end
    let slice = lex.slice();
    let slice = &slice[2 + number_equals + skip_newlines..(slice.len() - (2 + number_equals))];
    FilterResult::Emit(lex.extras.2.intern(slice.as_bytes()))
    //FilterResult::Skip
}

pub(super) fn multiline_comment_callback(
    lex: &mut LogosLexer<TokenKind>,
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
