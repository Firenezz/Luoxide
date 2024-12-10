use core::fmt;
use std::{borrow::Borrow, mem::discriminant};

use logos::{Logos, Skip};
use luoxide_text::{range::TextSpan, traits::Ranged};

use crate::lexer::{self, Tokens};

// Making sure the Token size doesn't change without warning
static_assert_size!(Token, 12);

/// This struct describes a token and it's length
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Token {
    /// The kind of the token
    pub kind: TokenKind,

    /// The span of the token
    pub span: TextSpan,
}

impl Token {
    #[inline]
    pub fn is(&self, kind: impl Borrow<TokenKind>) -> bool {
        discriminant(&self.kind) == discriminant(kind.borrow())
    }

    #[inline]
    pub const fn kind(&self) -> &TokenKind {
        &self.kind
    }

    #[inline]
    pub const fn as_tuple(&self) -> (TokenKind, TextSpan) {
        (self.kind, self.span)
    }

    #[inline]
    pub const fn is_string(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Lit_String | TokenKind::Lit_MultilineString
        )
    }
}

impl Ranged for Token {
    fn range(&self) -> TextSpan {
        self.span
    }
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
/*
    Extras:
    (last_line_number, start_of_last_line, begin_line_number, start_of_begin_line)
*/
#[logos(extras = (usize, usize, usize, usize))]
#[logos(source = str)]
// TODO: Add more whitespaces
#[logos(skip r"[ \x07\x08\x0b\x0c\x1b\t\f]+")] // Ignore this regex pattern between tokens
pub enum TokenKind {
    // Keywords
    #[token("break")]
    Break,
    #[token("do")]
    Do,
    #[token("else")]
    Else,
    #[token("elseif")]
    ElseIf,
    #[token("end")]
    End,
    #[token("function")]
    Function,
    #[token("goto")]
    Goto,
    #[token("if")]
    If,
    #[token("in")]
    In,
    #[token("local")]
    Local,
    #[token("nil")]
    Nil,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("repeat")]
    Repeat,
    #[token("until")]
    Until,
    #[token("return")]
    Return,
    #[token("then")]
    Then,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    // Reserved keywords
    #[token("enum")]
    Enum,
    #[token("const")]
    Const,
    #[token("auto")]
    Auto,
    #[token("global")]
    Global,
    #[token("defer")]
    Defer,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("fallthrough")]
    Fallthrough,

    // Brackets
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,

    // Misc characters
    #[token(";")]
    SemiColon,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("...")]
    Dots,
    #[token("@")]
    At,

    // Operators
    #[token("-")]
    Minus,
    #[token("+")]
    Add,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("//")]
    IDiv,
    #[token("^")]
    Pow,
    #[token("%")]
    Mod,
    #[token("#")]
    Pound,
    #[token("~")]
    Tilde,
    #[token("&")]
    Amper,
    #[token("|")]
    BitOr,
    #[token(">>")]
    ShiftRight,
    #[token("<<")]
    ShiftLeft,
    #[token("=")]
    Assign,
    #[token("..")]
    Concat,

    // Equality operators
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    Equal,
    #[token("~=")]
    NotEqual,

    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*")]
    Lit_Identifier,
    #[regex("[0-9][0-9_]*", priority = 5)]
    Lit_Number,
    #[regex("0x[0-9a-fA-F_]+")]
    Lit_HexNumber,
    /// Token for floats
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luoxidant::intern::DefaultInterner;
    /// use luoxidant::compiler::lexer::{Lexer, TokenKind, Tokens, DisplayToken, TokenVec};
    /// use std::rc::Rc;
    ///
    /// let input = r##"
    /// -- floats
    /// 3.0
    /// 3.1416
    /// 314.16e-2
    /// 0.31416E1
    /// 34e1
    /// 0x0.1E
    /// 0xA23p-4
    /// 0X1.921FB54442D18P+1
    /// NaN
    /// "##;
    ///
    /// let interner = Rc::from(DefaultInterner::default());
    /// let lexer = Lexer::new(input, interner.clone());
    /// let tokens = TokenVec(Tokens(lexer)
    ///     .map(|(string, token)| {
    ///         DisplayToken(token, string)
    ///     })
    ///     .collect::<Vec<_>>());
    ///
    /// println!("{:#?}", tokens); // TODO: use snapshot testing
    /// ```
    ///
    #[regex(r"[0-9]+(\.[0-9]+)?([Ee][+-]?[0-9]+)?")]
    Lit_Float,
    #[regex(r"0[xX]([0-9a-fA-F][0-9a-fA-F]*)?(\.[0-9a-fA-F][0-9a-fA-F]*)?([pP][+-]?[0-9]{1,2})?")]
    Lit_HexFloat,
    /// Token for strings
    ///
    /// The strings are interned in the interner of the lexer
    ///
    /// Example of a string
    /// `"hello world"`,
    /// `'Hello World'`,
    /// `[[
    /// hello world
    /// ]]`,
    /// `[=[
    /// hello world
    /// ]=]`,
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    Lit_String,
    #[regex(r#"\[(=*)\["#, callbacks::long_string_callback)]
    Lit_MultilineString,
    #[token("true")]
    Lit_True,
    #[token("false")]
    Lit_False,
    #[token("NaN")]
    NaN,

    #[doc(hidden)]
    #[regex(r"--\[(=*)\[", callbacks::multiline_comment_callback)]
    _Tok_MultilineComment,
    #[doc(hidden)]
    #[regex(r"--[^\[][^\n|\r|\n\r]*", |_| Skip)]
    _Tok_Comment,

    //#[token("\n\r")]
    #[token("\r\n", callbacks::increment_line_number)]
    //#[token("\r")]
    #[token("\n", callbacks::increment_line_number)]
    _Newline,

    Tok_Error,
    /// Token for end of file
    Tok_Eof,

    #[doc(hidden)]
    #[default]
    _Unknown,
}

impl TokenKind {
    #[inline]
    #[no_mangle]
    pub const fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Break
            | TokenKind::Do
            | TokenKind::Else
            | TokenKind::ElseIf
            | TokenKind::End
            | TokenKind::Lit_True
            | TokenKind::Lit_False
            | TokenKind::For
            | TokenKind::Function
            | TokenKind::Goto
            | TokenKind::If
            | TokenKind::In
            | TokenKind::Local
            | TokenKind::Nil
            | TokenKind::Repeat
            | TokenKind::Return
            | TokenKind::Then
            | TokenKind::Until
            | TokenKind::While
        )
    }

    #[inline]
    pub const fn is_bracket(&self) -> bool {
        matches!(
            self,
            TokenKind::LeftCurly
                | TokenKind::RightCurly
                | TokenKind::LeftSquare
                | TokenKind::RightSquare
                | TokenKind::LeftParen
                | TokenKind::RightParen
        )
    }

    #[inline]
    pub const fn is_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::And
                | TokenKind::Or
                | TokenKind::Not
                | TokenKind::Concat
                | TokenKind::LessThan
                | TokenKind::LessEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterEqual
                | TokenKind::Equal
                | TokenKind::NotEqual
        )
    }

    #[inline]
    pub const fn is_literal(&self) -> bool {
        matches!(
            self,
            TokenKind::Lit_Float
                | TokenKind::Lit_HexFloat
                | TokenKind::Lit_String
                | TokenKind::Lit_MultilineString
                | TokenKind::Lit_True
                | TokenKind::Lit_False
                | TokenKind::NaN
        )
    }

    #[inline]
    pub const fn is_identifier(&self) -> bool {
        matches!(self, TokenKind::Lit_Identifier)
    }

    #[inline]
    pub const fn is_number(&self) -> bool {
        matches!(self, TokenKind::Lit_Number | TokenKind::Lit_HexNumber)
    }

    #[inline]
    pub const fn is_string(&self) -> bool {
        matches!(self, TokenKind::Lit_String | TokenKind::Lit_MultilineString)
    }

    #[inline]
    pub const fn is_trivia(&self) -> bool {
        matches!(
            self,
            TokenKind::_Tok_MultilineComment | TokenKind::_Tok_Comment | TokenKind::_Newline
        )
    }

    #[inline]
    pub const fn is_newline(&self) -> bool {
        matches!(self, TokenKind::_Newline)
    }

    #[inline]
    pub const fn is_singleton(&self) -> bool {
        matches!(
            self,
            TokenKind::Lit_True | TokenKind::Lit_False | TokenKind::NaN | TokenKind::Nil
        )
    }

    #[inline]
    pub const fn is_unary_arithmetic_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Minus | TokenKind::Not | TokenKind::Pound | TokenKind::Tilde
        )
    }

    #[inline]
    pub const fn is_reserved(&self) -> bool {
        matches!(self, token!(reserved))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LexingError {
    UnterminatedMultiline(MultilineKind),
    InvalidUtf8Char,
    #[default]
    Unkown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultilineKind {
    String,
    Comment,
}

mod callbacks {
    use super::*;
    use hexfloat2::HexFloat64;
    use logos::{FilterResult, Lexer as LogosLexer};

    // Read a [=*[...]=*] sequence with matching numbers of '='. return Emit(Rc<[u8]>)
    pub(super) fn long_string_callback(
        lex: &mut LogosLexer<TokenKind>,
    ) -> FilterResult<(), LexingError> {
        use logos::internal::LexerInternal;

        // for multi lines keep track of the "=" and the number of lines
        // example [===[ ... ]===]

        // For starter we count the number of "=" if there is any and add to stack

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
                                                LexingError::UnterminatedMultiline(
                                                    MultilineKind::String,
                                                ),
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
                        any_char => match utf8_char_width(any_char) {
                            Ok(amount) => lex.bump(amount),
                            Err(_) => return FilterResult::Error(LexingError::InvalidUtf8Char),
                        },
                    }
                }
                None => {
                    return FilterResult::Error(LexingError::UnterminatedMultiline(
                        MultilineKind::String,
                    ))
                }
            }
        }

        lex.extras.0 = lines;
        lex.extras.1 = lex.span().end;

        // trim the start and end
        //let slice = lex.slice();
        //let slice = &slice[2 + number_equals..(slice.len() - (2 + number_equals))];
        FilterResult::Emit(())
        //FilterResult::Emit(lex.extras.2.intern(slice.as_bytes()))
        //FilterResult::Skip
    }

    pub(super) fn multiline_comment_callback(
        lex: &mut LogosLexer<TokenKind>,
    ) -> FilterResult<(), LexingError> {
        use logos::internal::LexerInternal;
        match lex.read::<&[u8; 2usize]>() {
            Some(b"--") => {
                // we have a multi line comment
                lex.bump(2usize);
                match long_string_callback(lex) {
                    FilterResult::Emit(()) => FilterResult::Emit(()),
                    FilterResult::Error(err) => match err {
                        LexingError::UnterminatedMultiline(_) => FilterResult::Error(
                            LexingError::UnterminatedMultiline(MultilineKind::Comment),
                        ),
                        _ => FilterResult::Error(LexingError::Unkown),
                    },
                    FilterResult::Skip => FilterResult::Skip,
                }
            }
            Some(_chars) => {
                unreachable!("LogosLexer should have detected \"--\" for it to call this function")
            }
            None => unreachable!("we should not be here. \nToken: {:#?}", lex.slice()),
        }
    }

    pub(super) fn increment_line_number(lexer: &mut logos::Lexer<'_, TokenKind>) {
        let extras = &mut lexer.extras;
        extras.0 += 1;
        extras.1 = 0;
    }

    fn utf8_char_width(first_byte: u8) -> Result<usize, LexingError> {
        match first_byte {
            0x00..=0x7F => Ok(1),
            0xC2..=0xDF => Ok(2),
            0xE0..=0xEF => Ok(3),
            0xF0..=0xF4 => Ok(4),
            _ => Err(LexingError::InvalidUtf8Char),
        }
    }

    /*pub(super) fn hex_to_integer(
        lex: &mut LogosLexer<TokenKind<Rc<[u8]>>>,
    ) -> FilterResult<i64, LexingError> {
        let slice = lex.slice();
        match i64::from_str_radix(&slice[2..], 16) {
            Ok(int) => FilterResult::Emit(int),
            Err(err) => FilterResult::Error(err.into()),
        }
    }*/

    /// Convert a hex string to a float
    ///
    /// # Panics
    ///
    /// Panics if the string is not a valid hex string
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// use luoxide_parser::token::hex_to_float;
    ///
    /// assert_eq!(hex_to_float("3.0"), 3.0);
    /// assert_eq!(hex_to_float("3.1416"), 3.1416);
    /// assert_eq!(hex_to_float("314.16e-2"), 3.1416);
    /// assert_eq!(hex_to_float("0.31416E1"), 3.1416);
    /// assert_eq!(hex_to_float("34e1"), 34e1);
    /// assert_eq!(hex_to_float("0x0.1E"), 0.1);
    /// assert_eq!(hex_to_float("0xA23p-4"), 0.123);
    /// assert_eq!(hex_to_float("0X1.921FB54442D18P+1"), 1.921FB54442D18);
    /// ```
    ///
    pub(super) fn hex_to_float(str: &str) -> f64 {
        let float: HexFloat64 = str.parse().unwrap();
        float.into()
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match &self {
                TokenKind::Break => write!(f, "Break"),
                TokenKind::Do => write!(f, "Do"),
                TokenKind::Else => write!(f, "Else"),
                TokenKind::ElseIf => write!(f, "ElseIf"),
                TokenKind::End => write!(f, "End"),
                TokenKind::Function => write!(f, "Function"),
                TokenKind::Goto => write!(f, "Goto"),
                TokenKind::If => write!(f, "If"),
                TokenKind::In => write!(f, "In"),
                TokenKind::Local => write!(f, "Local"),
                TokenKind::Nil => write!(f, "Nil"),
                TokenKind::For => write!(f, "For"),
                TokenKind::While => write!(f, "While"),
                TokenKind::Repeat => write!(f, "Repeat"),
                TokenKind::Until => write!(f, "Until"),
                TokenKind::Return => write!(f, "Return"),
                TokenKind::Then => write!(f, "Then"),
                TokenKind::Not => write!(f, "Not"),
                TokenKind::And => write!(f, "And"),
                TokenKind::Or => write!(f, "Or"),
                TokenKind::NaN => write!(f, "NaN"),
                TokenKind::LeftCurly => write!(f, "LeftCurly"),
                TokenKind::RightCurly => write!(f, "RightCurly"),
                TokenKind::LeftSquare => write!(f, "LeftSquare"),
                TokenKind::RightSquare => write!(f, "RightSquare"),
                TokenKind::LeftParen => write!(f, "LeftParen"),
                TokenKind::RightParen => write!(f, "RightParen"),
                TokenKind::SemiColon => write!(f, "SemiColon"),
                TokenKind::Colon => write!(f, "Colon"),
                TokenKind::DoubleColon => write!(f, "DoubleColon"),
                TokenKind::Comma => write!(f, "Comma"),
                TokenKind::Minus => write!(f, "Minus"),
                TokenKind::Add => write!(f, "Add"),
                TokenKind::Mul => write!(f, "Mul"),
                TokenKind::Div => write!(f, "Div"),
                TokenKind::IDiv => write!(f, "IDiv"),
                TokenKind::Pow => write!(f, "Pow"),
                TokenKind::Mod => write!(f, "Mod"),
                TokenKind::Pound => write!(f, "Pound"),
                TokenKind::Tilde => write!(f, "Tilde"),
                TokenKind::Amper => write!(f, "BitAnd"),
                TokenKind::BitOr => write!(f, "BitOr"),
                TokenKind::ShiftRight => write!(f, "ShiftRight"),
                TokenKind::ShiftLeft => write!(f, "ShiftLeft"),
                TokenKind::Assign => write!(f, "Assign"),
                TokenKind::Dot => write!(f, "Dot"),
                TokenKind::Concat => write!(f, "Concat"),
                TokenKind::Dots => write!(f, "Dots"),
                TokenKind::LessThan => write!(f, "LessThan"),
                TokenKind::LessEqual => write!(f, "LessEqual"),
                TokenKind::GreaterThan => write!(f, "GreaterThan"),
                TokenKind::GreaterEqual => write!(f, "GreaterEqual"),
                TokenKind::Equal => write!(f, "Equal"),
                TokenKind::NotEqual => write!(f, "NotEqual"),
                TokenKind::Lit_Identifier => write!(f, "Identifier"),
                TokenKind::Lit_Number => write!(f, "Number"),
                TokenKind::Lit_HexNumber => write!(f, "HexNumber"),
                TokenKind::Lit_Float => write!(f, "Float"),
                TokenKind::Lit_HexFloat => write!(f, "HexFloat"),
                TokenKind::Lit_String => write!(f, "String"),
                TokenKind::Lit_MultilineString => write!(f, "MultiLineString"),
                TokenKind::Lit_True => write!(f, "True"),
                TokenKind::Lit_False => write!(f, "False"),
                TokenKind::_Tok_MultilineComment => write!(f, "MultiLineComment"),
                TokenKind::_Tok_Comment => write!(f, "Comment"),
                TokenKind::_Newline => write!(f, "NewLine"),
                TokenKind::Tok_Error => write!(f, "Error"),
                TokenKind::Tok_Eof => write!(f, "Eof"),
                TokenKind::_Unknown => write!(f, "Unknown"),

                // Reserved
                TokenKind::Enum => write!(f, "Enum"),
                TokenKind::Const => write!(f, "Const"),
                TokenKind::Auto => write!(f, "Auto"),
                TokenKind::Global => write!(f, "Global"),
                TokenKind::Defer => write!(f, "Defer"),
                TokenKind::Switch => write!(f, "Switch"),
                TokenKind::Case => write!(f, "Case"),
                TokenKind::Fallthrough => write!(f, "Fallthrough"),

                _ => write!(f, "New token"),
            }
        } else {
            write!(f, "{:?}", self)
        }
    }
}
