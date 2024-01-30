
use std::{borrow::Borrow, fmt, io::{Read, Cursor}, iter::Peekable, num::{IntErrorKind, ParseIntError}, rc::Rc, string};

use crate::intern::{DefaultInterner, StringInterner};

use thiserror::Error;

use logos::Lexer as LogosLexer;

use logos::{FilterResult, Logos, Skip};


fn newline_callback(lex: &mut LogosLexer<LogosToken<Rc<[u8]>>>) -> usize {
    lex.extras.1 += 1;
    lex.extras.2 = lex.span().end;
    lex.extras.1
}

fn print_char(char: u8) -> char {
    todo!()
}

#[derive(Default, Debug, Error, Clone, PartialEq)]
pub enum LexingError {
    #[error("short string not finished, expected matching {}", print_char(*.0))]
    UnterminatedShortString(u8),
    #[error("unexpected character: {}", print_char(*.0))]
    UnexpectedCharacter(u8),
    #[error("hexadecimal digit expected")]
    HexDigitExpected,
    #[error("missing '{{' in \\u{{xxxx}} escape")]
    EscapeUnicodeStart,
    #[error("missing '}}' in \\u{{xxxx}} escape")]
    EscapeUnicodeEnd,
    #[error("invalid unicode value in \\u{{xxxx}} escape")]
    EscapeUnicodeInvalid,
    #[error("\\ddd escape out of 0-255 range")]
    EscapeDecimalTooLarge,
    #[error("invalid escape sequence")]
    InvalidEscape,
    #[error("invalid long string delimiter")]
    InvalidLongStringDelimiter,
    #[error("unfinished long string")]
    UnterminatedLongString,
    #[error("unterminated multi line comment")]
    UnterminatedMultiLineComment,
    #[error("malformed number")]
    InvalidNumber(InvalidNumber, String),
    #[error("unkown error")]
    #[default]
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InvalidNumber {
    Empty,
    Invalid,
    Overflow,
    Zero,
    Unknown
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        match err.kind() {
            IntErrorKind::Empty => unreachable!("lexer should not produce this error"),
            IntErrorKind::InvalidDigit =>  LexingError::InvalidNumber(InvalidNumber::Invalid, err.to_string()),
            IntErrorKind::PosOverflow => LexingError::InvalidNumber(InvalidNumber::Overflow, err.to_string()),
            IntErrorKind::NegOverflow => LexingError::InvalidNumber(InvalidNumber::Overflow, err.to_string()),
            IntErrorKind::Zero => LexingError::InvalidNumber(InvalidNumber::Zero, err.to_string()),
            _ => todo!(),
        }
    }
}

fn multiline_comment_callback(lex: &mut LogosLexer<LogosToken<Rc<[u8]>>>) -> FilterResult<String, LexingError> {
    use logos::internal::LexerInternal;

    // for multi lines keep track of the "=" and the number of lines
    // example --[===[ ... --]===]

    // For starter we count the number of "=" if there is any and add to stack

    let mut stack = vec![];
    let mut lines = lex.extras.1;
    let start_slice = lex.slice();
    
    // the regex should filter out the bad starts so the number of "=" should be the lenght of the slice - 4
    stack.push(start_slice.len() - 4);

    let count_equals = |lex: &mut LogosLexer<'_, LogosToken<Rc<[u8]>>>, ends_with| {
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

    // now we can loop until the stack is empty

    while let Some(number_equals) = stack.pop() {
        loop {
            // get characters one by one until we find the end or an end of comment "--]...]"
            match lex.read::<u8>() {
                Some(comment_char) => {
                    match comment_char {
                        b'-' => {
                            lex.bump(1usize);
                            // read the character 2 bytes further for checking "--]" or "--["
                            match lex.read::<&[u8; 2usize]>() {
                                Some(b"-[") => {
                                    lex.bump(2usize);
                                    // new scope might be starting
                                    match count_equals(lex, b'[') {
                                        Ok(count) => {
                                            stack.push(number_equals);
                                            stack.push(count);
                                            lex.bump(count + 1usize);
                                            break;
                                        },
                                        Err(result) => {
                                            match result {
                                                (_, true) => return FilterResult::Error(LexingError::UnterminatedMultiLineComment), // TODO: bump the lexer to the end of the comment
                                                (count, false) => {
                                                    // end of comment token didnt match --[...[
                                                    lex.bump(count + 1usize);
                                                }
                                            }
                                        }
                                    }
                                },
                                Some(b"-]") => {
                                    lex.bump(2usize);
                                    // new scope might be ending
                                    match count_equals(lex, b']') {
                                        Ok(count) => {
                                            lex.bump(count + 1usize);
                                            if count == number_equals {
                                                break;
                                            }
                                        },
                                        Err(result) => {
                                            match result {
                                                (_, true) => return FilterResult::Error(LexingError::UnterminatedMultiLineComment), // TODO: bump the lexer to the end of the comment
                                                (count, false) => {
                                                    // end of comment token didnt match --]...]
                                                    lex.bump(count + 1usize);
                                                }
                                            }
                                        },
                                    }
                                    
                                },
                                Some(_temp) => lex.bump(2usize), // read the next character
                                None => return FilterResult::Error(LexingError::UnterminatedMultiLineComment),
                            }
                        },
                        b']' => {
                            lex.bump(1usize);
                            // new scope might be ending
                            match count_equals(lex, b']') {
                                Ok(count) => {
                                    lex.bump(count + 1usize);
                                    if count == number_equals {
                                        break;
                                    }
                                },
                                Err(result) => {
                                    match result {
                                        (_, true) => return FilterResult::Error(LexingError::UnterminatedMultiLineComment), // TODO: bump the lexer to the end of the comment
                                        (count, false) => {
                                            // end of comment token didnt match --]...]
                                            lex.bump(count + 1usize);
                                        }
                                    }
                                },
                            }
                        }
                        b'\n' | b'\r' => {
                            lines += 1;
                            lex.bump(1usize);
                        },
                        any_char => lex.bump(utf8_char_width(any_char)),
                    }
                },
                None => return FilterResult::Error(LexingError::UnterminatedMultiLineComment),
            }
        }
    }

    lex.extras.1 = lines;
    lex.extras.2 = lex.span().end;
    FilterResult::Emit(lex.slice().to_owned())
    //FilterResult::Skip
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

// Read a [=*[...]=*] sequence with matching numbers of '='. return Emit(Rc<[u8]>)
fn long_string_callback(lex: &mut LogosLexer<LogosToken<Rc<[u8]>>>) -> FilterResult<Rc<[u8]>, LexingError> {
    use logos::internal::LexerInternal;

    // for multi lines keep track of the "=" and the number of lines
    // example [===[ ... ]===]

    // For starter we count the number of "=" if there is any and add to stack

    let mut lines = lex.extras.1;
    let start_slice = lex.slice();
    
    // the regex should filter out the bad starts so the number of "=" should be the lenght of the slice - 4
    let number_equals = start_slice.len() - 2;

    let count_equals = |lex: &mut LogosLexer<'_, LogosToken<Rc<[u8]>>>, ends_with| {
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
                match chars[1] {
                    b'\n' | b'\r' => {
                        lex.bump(2usize);
                    },
                    _ => lex.bump(1usize),
                    
                }
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
                match string_string { // all escape sequences are ignored
                    b']' => {
                        lex.bump(1usize);
                        // new scope might be ending
                        match count_equals(lex, b']') {
                            Ok(count) => {
                                lex.bump(count + 1usize);
                                if count == number_equals {
                                    break;
                                }
                            },
                            Err(result) => {
                                match result {
                                    (_, true) => return FilterResult::Error(LexingError::UnterminatedMultiLineComment), // TODO: bump the lexer to the end of the comment
                                    (count, false) => {
                                        // end of comment token didnt match --]...]
                                        lex.bump(count + 1usize);
                                    }
                                }
                            },
                        }
                    }
                    b'\n' | b'\r' => {
                        lines += 1;
                        lex.bump(1usize);
                    },
                    any_char => lex.bump(utf8_char_width(any_char)),
                }
            },
            None => return FilterResult::Error(LexingError::UnterminatedMultiLineComment),
        }
    }

    lex.extras.1 = lines;
    lex.extras.2 = lex.span().end;

    // trim the start and end
    let slice = lex.slice();
    let slice = &slice[2 + number_equals..(slice.len() - (2 + number_equals))];
    FilterResult::Emit(lex.extras.0.intern(slice.as_bytes()))
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
#[logos(error = LexingError)]
#[logos(type S = Rc<[u8]>)]
#[logos(extras = (dyn StringInterner<String = Rc<[u8]>>, usize, usize))]
#[logos(source = BufReadSource)]
pub enum LogosToken<S> {
    #[token("\n\r", newline_callback)]
    #[token("\r\n", newline_callback)]
    #[token("\r", newline_callback)]
    #[token("\n", newline_callback)]
    Newline(usize),
    #[regex(r"--\[(=*)\[", multiline_comment_callback)]
    MultiLineComment(String),
    #[regex(r"--[^\[][^\n|\r|\n\r]*", |_|  Skip)]
    SingleLineComment,
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
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,
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
    Len,
    #[token("~")]
    BitNotXor,
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token(">>")]
    ShiftRight,
    #[token("<<")]
    ShiftLeft,
    #[token("..")]
    Concat,
    #[token("...")]
    Dots,
    #[token("=")]
    Assign,
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
    #[token(".")]
    Dot,
    #[token(";")]
    SemiColon,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token(",")]
    Comma,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*", interner_callback)]
    Name(S),
    #[regex(r#"'(?:[^']|\\')*'"#, interner_callback)] // TODO: use sublexer to parse strings
    #[regex(r#""(?:[^"]|\\")*""#, interner_callback)]
    #[regex(r#"\[(=*)\["#, long_string_callback)]
    String(S),
    /// Numerals are only lexed as integers in the range [-(2^63-1), 2^63-1], otherwise they will be
    /// lexed as floats.
    #[regex(r"\d+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    //([eE][+-]?[0-9]+))
    #[regex(r"([0-9]+([.][0-9]+))", |lex| lex.slice().parse().ok())]
    Float(f64),
}

pub enum LuoxidantToken<S> {
    Newline(usize),
    MultiLineComment(String),
    SingleLineComment,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    For,
    While,
    Repeat,
    Until,
    Return,
    Then,
    True,
    False,
    Not,
    And,
    Or,
    Minus,
    Add,
    Mul,
    Div,
    IDiv,
    Pow,
    Mod,
    Len,
    BitNotXor,
    BitAnd,
    BitOr,
    ShiftRight,
    ShiftLeft,
    Concat,
    Dots,
    Assign,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    NotEqual,
    Dot,
    SemiColon,
    Colon,
    DoubleColon,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Name(S),
    String(S),
    /// Numerals are only lexed as integers in the range [-(2^63-1), 2^63-1], otherwise they will be
    /// lexed as floats.
    Integer(i64),
    //([eE][+-]?[0-9]+))
    Float(f64),
}

impl<S: AsRef<[u8]>> fmt::Debug for LuoxidantToken<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuoxidantToken::Break => write!(f, "Break"),
            LuoxidantToken::Do => write!(f, "Do"),
            LuoxidantToken::Else => write!(f, "Else"),
            LuoxidantToken::ElseIf => write!(f, "ElseIf"),
            LuoxidantToken::End => write!(f, "End"),
            LuoxidantToken::Function => write!(f, "Function"),
            LuoxidantToken::Goto => write!(f, "Goto"),
            LuoxidantToken::If => write!(f, "If"),
            LuoxidantToken::In => write!(f, "In"),
            LuoxidantToken::Local => write!(f, "Local"),
            LuoxidantToken::Nil => write!(f, "Nil"),
            LuoxidantToken::For => write!(f, "For"),
            LuoxidantToken::While => write!(f, "While"),
            LuoxidantToken::Repeat => write!(f, "Repeat"),
            LuoxidantToken::Until => write!(f, "Until"),
            LuoxidantToken::Return => write!(f, "Return"),
            LuoxidantToken::Then => write!(f, "Then"),
            LuoxidantToken::True => write!(f, "True"),
            LuoxidantToken::False => write!(f, "False"),
            LuoxidantToken::Not => write!(f, "Not"),
            LuoxidantToken::And => write!(f, "And"),
            LuoxidantToken::Or => write!(f, "Or"),
            LuoxidantToken::Minus => write!(f, "Minus"),
            LuoxidantToken::Add => write!(f, "Add"),
            LuoxidantToken::Mul => write!(f, "Mul"),
            LuoxidantToken::Div => write!(f, "Div"),
            LuoxidantToken::IDiv => write!(f, "IDiv"),
            LuoxidantToken::Pow => write!(f, "Pow"),
            LuoxidantToken::Mod => write!(f, "Mod"),
            LuoxidantToken::Len => write!(f, "Len"),
            LuoxidantToken::BitNotXor => write!(f, "BitNotXor"),
            LuoxidantToken::BitAnd => write!(f, "BitAnd"),
            LuoxidantToken::BitOr => write!(f, "BitOr"),
            LuoxidantToken::ShiftRight => write!(f, "ShiftRight"),
            LuoxidantToken::ShiftLeft => write!(f, "ShiftLeft"),
            LuoxidantToken::Concat => write!(f, "Concat"),
            LuoxidantToken::Dots => write!(f, "Dots"),
            LuoxidantToken::Assign => write!(f, "Assign"),
            LuoxidantToken::LessThan => write!(f, "LessThan"),
            LuoxidantToken::LessEqual => write!(f, "LessEqual"),
            LuoxidantToken::GreaterThan => write!(f, "GreaterThan"),
            LuoxidantToken::GreaterEqual => write!(f, "GreaterEqual"),
            LuoxidantToken::Equal => write!(f, "Equal"),
            LuoxidantToken::NotEqual => write!(f, "NotEqual"),
            LuoxidantToken::Dot => write!(f, "Dot"),
            LuoxidantToken::SemiColon => write!(f, "SemiColon"),
            LuoxidantToken::Colon => write!(f, "Colon"),
            LuoxidantToken::DoubleColon => write!(f, "DoubleColon"),
            LuoxidantToken::Comma => write!(f, "Comma"),
            LuoxidantToken::LeftParen => write!(f, "LeftParen"),
            LuoxidantToken::RightParen => write!(f, "RightParen"),
            LuoxidantToken::LeftBracket => write!(f, "LeftBracket"),
            LuoxidantToken::RightBracket => write!(f, "RightBracket"),
            LuoxidantToken::LeftBrace => write!(f, "LeftBrace"),
            LuoxidantToken::RightBrace => write!(f, "RightBrace"),
            LuoxidantToken::Integer(i) => write!(f, "Integer({})", *i),
            LuoxidantToken::Float(d) => write!(f, "Float({})", *d),
            LuoxidantToken::Name(n) => write!(f, "Name({:?})", String::from_utf8_lossy(n.as_ref())),
            LuoxidantToken::String(s) => write!(f, "String({:?})", String::from_utf8_lossy(s.as_ref())),
            _ => write!(f, "{:?}", self)
        }
    }
}

pub struct LexerState<'source, R: Read, S: StringInterner = DefaultInterner> {
    pub source: &'source R,
    pub interner: S,
    pub line: usize,
    pub column: usize,
}


impl<'source, R: Read, S> LexerState<'source, R, S> 
where
    S: StringInterner<String = S> {
    pub fn new(source: &'source R, interner: S::String) -> Self {
        Self {
            interner,
            line: 1,
            column: 1,
            source: todo!(),
        }
    }
}