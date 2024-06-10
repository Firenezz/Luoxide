use core::{
    borrow::Borrow,
    fmt,
    mem::discriminant,
    num::{IntErrorKind, ParseFloatError, ParseIntError},
    ops::Range,
};

use std::rc::Rc;

use logos::{Logos, Skip};
use thiserror::Error;

use crate::{
    intern::{DefaultInterner, StringInterner},
    span::Span,
};

mod callbacks;
pub mod string;
mod util;

type InternedString = Rc<[u8]>;

pub(crate) const ASCII_BELL: u8 = 0x07;
pub(crate) const ASCII_BACKSPACE: u8 = 0x08;
pub(crate) const ASCII_VERTICAL_TAB: u8 = 0x0b;
pub(crate) const ASCII_FORM_FEED: u8 = 0x0c;
//pub(crate) const ASCII_ESCAPE: u8 = 0x1b;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn is(&self, kind: impl Borrow<TokenKind>) -> bool {
        discriminant(&self.kind) == discriminant(kind.borrow())
    }
}

pub struct Lexer<'src> {
    source: &'src str,
    inner: logos::Lexer<'src, TokenKind>,
    #[allow(dead_code)] // TODO: Remove when the parser is done
    interner: Rc<DefaultInterner>, // CHECK: Could we make this generic?
    previous: Token,
    current: Token,
    end_of_file: Token,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LineNumber(pub usize);

impl From<usize> for LineNumber {
    fn from(line: usize) -> Self {
        Self(line)
    }
}

impl From<LineNumber> for usize {
    fn from(line: LineNumber) -> Self {
        line.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LineInfo {
    pub line: LineNumber,
    pub start_of_line: usize,
}

impl fmt::Display for LineNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'src> Lexer<'src> {
    #[allow(dead_code)]
    pub fn new(source: &'src str, interner: Rc<DefaultInterner>) -> Self {
        let end = source.len();
        let end_of_file = Token {
            kind: TokenKind::Tok_Eof,
            span: (end..end).into(),
        };

        let mut lex = Self {
            source,
            inner: TokenKind::lexer_with_extras(source, (0, 0, interner.clone())),
            interner,
            previous: end_of_file.clone(),
            current: end_of_file.clone(),
            end_of_file,
        };
        lex.bump();

        lex
    }

    #[inline]
    #[allow(dead_code)]
    pub fn previous(&self) -> &Token {
        &self.previous
    }

    #[inline]
    pub fn current(&self) -> &Token {
        &self.current
    }

    #[inline]
    pub fn lexeme(&self, token: &Token) -> &'src str {
        &self.source[Range::from(token.span)]
    }

    #[inline]
    pub fn bump(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);

        self.current = self
            .next_token()
            .unwrap_or_else(|| self.end_of_file.clone());
    }

    fn next_token(&mut self) -> Option<Token> {
        let lexer = &mut self.inner;
        while let Some(kind) = lexer.next() {
            let _lexeme = lexer.slice();
            let span = lexer.span();

            match kind {
                Ok(
                    TokenKind::_Tok_Comment
                    | TokenKind::_Tok_MultiLineComment(_)
                    | TokenKind::_Newline(_),
                ) => continue,
                Ok(kind) => {
                    let token = Token {
                        kind,
                        span: (span.start..span.end).into(),
                    };
                    return Some(token);
                }
                Err(_) => {
                    let token = Token {
                        kind: TokenKind::Tok_Error,
                        span: (span.start..span.end).into(),
                    };
                    return Some(token);
                }
            }
        }

        None
    }

    pub fn get_current_line(&self) -> LineInfo {
        let line_number = self.inner.extras.0;
        let start_of_line = self.inner.extras.1;

        LineInfo {
            line: line_number.into(),
            start_of_line,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
#[logos(extras = (usize, usize, Rc<DefaultInterner>))]
// TODO: Add more whitespaces
#[logos(skip r"[ \x07\x08\x0b\x0c\x1b\t\f]+")] // Ignore this regex pattern between tokens
pub enum TokenKind {
    #[token("\n\r", callbacks::newline_callback)]
    #[token("\r\n", callbacks::newline_callback)]
    #[token("\r", callbacks::newline_callback)]
    #[token("\n", callbacks::newline_callback)]
    _Newline(usize),

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
    BitXor,
    #[token("&")]
    BitAnd,
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

    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*", callbacks::interner_identifier_callback)]
    Lit_Identifier(InternedString),
    #[cfg(not(feature = "32-bit"))]
    #[regex("[0-9][0-9_]*", |lex| lex.slice().parse().ok(), priority = 5)]
    #[regex("0x[0-9a-fA-F_]+", util::hex_to_integer)]
    Lit_Integer(i64),
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
    #[regex(r"[0-9]+(\.[0-9]+)?([Ee][+-]?[0-9]+)?", |lex| lex.slice().parse().ok())]
    #[regex(
        r"0[xX]([0-9a-fA-F][0-9a-fA-F]*)?(\.[0-9a-fA-F][0-9a-fA-F]*)?([pP][+-]?[0-9]{1,2})?",
        util::hex_to_float
    )]
    #[token("NaN", |_| f64::NAN)]
    Lit_Float(f64),
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
    #[regex(r#""([^"\\]|\\.)*""#, callbacks::interner_callback)]
    #[regex(r#"'([^'\\]|\\.)*'"#, callbacks::interner_callback)]
    Lit_String(InternedString),
    #[regex(r#"\[(=*)\["#, callbacks::long_string_callback)]
    Lit_MultiLineString(InternedString),
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Lit_Bool(bool),

    #[doc(hidden)]
    #[regex(r"--\[(=*)\[", callbacks::multiline_comment_callback)]
    _Tok_MultiLineComment(InternedString),
    #[doc(hidden)]
    #[regex(r"--[^\[][^\n|\r|\n\r]*", |_|  Skip)]
    _Tok_Comment, // TODO: intern string

    Tok_Error,
    /// Token for end of file
    Tok_Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            TokenKind::Pound => write!(f, "Len"),
            TokenKind::BitXor => write!(f, "BitXor"),
            TokenKind::BitAnd => write!(f, "BitAnd"),
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
            TokenKind::Lit_Identifier(id) => write!(
                f,
                "Identifier({:p}: {})",
                *id,
                String::from_utf8_lossy(id.as_ref())
            ),
            TokenKind::Lit_Integer(number) => write!(f, "Integer({})", number),
            TokenKind::Lit_Float(float) => write!(f, "Float({})", float),
            TokenKind::Lit_String(string) => write!(
                f,
                "String({:p}: {})",
                *string,
                String::from_utf8_lossy(string.as_ref()) // TODO: check if this is the correct way to get the string
            ),
            TokenKind::Lit_MultiLineString(string) => write!(
                f,
                "String({:p}: {})",
                *string,
                String::from_utf8_lossy(string.as_ref())
            ),
            TokenKind::Lit_Bool(value) => write!(f, "Bool({})", value),
            TokenKind::_Tok_MultiLineComment(comment) => write!(
                f,
                "MultiLineComment({:?})",
                String::from_utf8_lossy(comment.as_ref())
            ),
            TokenKind::_Tok_Comment => write!(f, "Comment"),
            TokenKind::_Newline(linenumber) => write!(f, "Newline({})", linenumber),
            TokenKind::Tok_Error => write!(f, "Error"),
            TokenKind::Tok_Eof => write!(f, "Eof"),
        }
    }
}

pub struct Tokens<'source>(pub Lexer<'source>);

impl<'source> Iterator for Tokens<'source> {
    type Item = (&'source str, Token);

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.0.current().clone();
        self.0.bump();
        if !token.is(TokenKind::Tok_Eof) {
            Some((self.0.lexeme(&token), token))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub struct DisplayToken<'source>(pub Token, pub &'source str);

impl<'source> fmt::Debug for DisplayToken<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self.0.kind.clone();
        let span = self.0.span;
        write!(f, "(>{kind:?} @{span})")
    }
}

impl<'source> fmt::Display for DisplayToken<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self.0.kind.clone();
        let span = self.0.span;
        write!(f, "(>{kind} @{span})")
    }
}

pub struct TokenVec<'a>(pub Vec<DisplayToken<'a>>);

impl<'a> fmt::Debug for TokenVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl<'a> fmt::Display for TokenVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comma_separated = String::new();

        for token in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(format!("{}", token).as_str());
            comma_separated.push_str(",\n");
        }

        comma_separated.push_str(format!("{}", &self.0[self.0.len() - 1]).as_str());
        write!(f, "{}", comma_separated)
    }
}

#[derive(Default, Debug, Error, Clone, PartialEq)]
#[allow(dead_code)]
pub enum LexingError {
    #[error("short string not finished, expected matching")]
    UnterminatedShortString,
    #[error("unexpected character")]
    UnexpectedCharacter,
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
#[allow(dead_code)]
pub enum InvalidNumber {
    Empty,
    Invalid,
    Overflow,
    Zero,
    Unknown,
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        match err.kind() {
            IntErrorKind::Empty => unreachable!("lexer should not produce this error"),
            IntErrorKind::InvalidDigit => {
                LexingError::InvalidNumber(InvalidNumber::Invalid, err.to_string())
            }
            IntErrorKind::PosOverflow => {
                LexingError::InvalidNumber(InvalidNumber::Overflow, err.to_string())
            }
            IntErrorKind::NegOverflow => {
                LexingError::InvalidNumber(InvalidNumber::Overflow, err.to_string())
            }
            IntErrorKind::Zero => LexingError::InvalidNumber(InvalidNumber::Zero, err.to_string()),
            _ => todo!(),
        }
    }
}

impl From<ParseFloatError> for LexingError {
    fn from(_: ParseFloatError) -> Self {
        LexingError::Unknown
    }
}

#[cfg(test)]
mod tests;
