use hexfloat2::HexFloat64;
use logos::{FilterResult, Lexer as LogosLexer};

use super::{LexingError, TokenKind};

pub(super) fn hex_to_integer(lex: &mut LogosLexer<TokenKind>) -> FilterResult<i64, LexingError> {
    let slice = lex.slice();
    match i64::from_str_radix(&slice[2..], 16) {
        Ok(int) => FilterResult::Emit(int),
        Err(err) => FilterResult::Error(err.into()),
    }
}

pub(super) fn hex_to_float(lex: &mut LogosLexer<TokenKind>) -> FilterResult<f64, LexingError> {
    let slice = lex.slice();

    let float: HexFloat64 = slice.parse().unwrap();
    FilterResult::Emit(float.0)
}
