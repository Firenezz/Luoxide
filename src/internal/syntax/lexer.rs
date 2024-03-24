use std::{
    borrow::Borrow,
    fmt,
    mem::discriminant,
    num::{IntErrorKind, ParseFloatError, ParseIntError},
    ops::Range,
    rc::Rc,
};

use logos::{Logos, Skip};
use thiserror::Error;

use crate::{
    intern::{DefaultInterner, StringInterner},
    span::Span,
};

mod callbacks;

type InternedString = Rc<[u8]>;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind<InternedString>,
    pub span: Span,
}

impl Token {
    pub fn is(&self, kind: impl Borrow<TokenKind<InternedString>>) -> bool {
        discriminant(&self.kind) == discriminant(kind.borrow())
    }
}

pub struct Lexer<'src> {
    source: &'src str,
    inner: logos::Lexer<'src, TokenKind<InternedString>>,
    #[allow(dead_code)] // TODO: Remove when the parser is done
    interner: Rc<DefaultInterner>, // CHECK: Could we make this generic?
    previous: Token,
    current: Token,
    end_of_file: Token,
}

impl<'src> Lexer<'src> {
    #[allow(dead_code)]
    pub fn new(source: &'src str, interner: Rc<DefaultInterner>) -> Self {
        let end = source.len();
        let end_of_file = Token {
            kind: TokenKind::<InternedString>::Tok_Eof,
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
                    | TokenKind::_Tok_Newline(_),
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
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
#[logos(extras = (usize, usize, Rc<DefaultInterner>))]
#[logos(type S = Rc<[u8]>)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum TokenKind<S> {
    #[token("\n\r", callbacks::newline_callback)]
    #[token("\r\n", callbacks::newline_callback)]
    #[token("\r", callbacks::newline_callback)]
    #[token("\n", callbacks::newline_callback)]
    _Tok_Newline(usize),

    // Keywords
    #[token("break")]
    Kw_Break,
    #[token("do")]
    Kw_Do,
    #[token("else")]
    Kw_Else,
    #[token("elseif")]
    Kw_ElseIf,
    #[token("end")]
    Kw_End,
    #[token("function")]
    Kw_Function,
    #[token("goto")]
    Kw_Goto,
    #[token("if")]
    Kw_If,
    #[token("in")]
    Kw_In,
    #[token("local")]
    Kw_Local,
    #[token("nil")]
    Kw_Nil,
    #[token("for")]
    Kw_For,
    #[token("while")]
    Kw_While,
    #[token("repeat")]
    Kw_Repeat,
    #[token("until")]
    Kw_Until,
    #[token("return")]
    Kw_Return,
    #[token("then")]
    Kw_Then,
    #[token("not")]
    Kw_Not,
    #[token("and")]
    Kw_And,
    #[token("or")]
    Kw_Or,

    // Brackets
    #[token("{")]
    Brk_LeftCurly,
    #[token("}")]
    Brk_RightCurly,
    #[token("[")]
    Brk_LeftSquare,
    #[token("]")]
    Brk_RightSquare,
    #[token("(")]
    Brk_LeftParen,
    #[token(")")]
    Brk_RightParen,

    // Misc characters
    #[token(";")]
    Tok_SemiColon,
    #[token(":")]
    Tok_Colon,
    #[token("::")]
    Tok_DoubleColon,
    #[token(",")]
    Tok_Comma,

    // Operators
    #[token("-")]
    Op_Minus,
    #[token("+")]
    Op_Add,
    #[token("*")]
    Op_Mul,
    #[token("/")]
    Op_Div,
    #[token("//")]
    Op_IDiv,
    #[token("^")]
    Op_Pow,
    #[token("%")]
    Op_Mod,
    #[token("#")]
    Op_Len,
    #[token("~")]
    Op_BitXor,
    #[token("&")]
    Op_BitAnd,
    #[token("|")]
    Op_BitOr,
    #[token(">>")]
    Op_ShiftRight,
    #[token("<<")]
    Op_ShiftLeft,
    #[token("=")]
    Op_Assign,
    #[token(".")]
    Op_Dot,
    #[token("..")]
    Op_Concat,
    #[token("...")]
    Op_Dots,

    // Equality operators
    #[token("<")]
    Op_LessThan,
    #[token("<=")]
    Op_LessEqual,
    #[token(">")]
    Op_GreaterThan,
    #[token(">=")]
    Op_GreaterEqual,
    #[token("==")]
    Op_Equal,
    #[token("~=")]
    Op_NotEqual,

    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*", callbacks::interner_identifier_callback)]
    Lit_Identifier(S),
    #[cfg(not(feature = "32-bit"))]
    #[regex("[0-9][0-9_]*", |lex| lex.slice().parse().ok(), priority = 5)]
    #[regex("0x[0-9a-fA-F_]+", callbacks::hex_to_integer)]
    Lit_Integer(i64),
    /*#[cfg(feature = "32-bit")]
    #[regex("[0-9][0-9_]*", |lex| lex.slice().parse(), priority = 10)]
    Lit_Integer(i32),*/
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
        callbacks::hex_to_float
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
    #[regex(r#"\[(=*)\["#, callbacks::long_string_callback)]
    Lit_String(S),
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Lit_Bool(bool),

    #[doc(hidden)]
    #[regex(r"--\[(=*)\[", callbacks::multiline_comment_callback)]
    _Tok_MultiLineComment(S),
    #[doc(hidden)]
    #[regex(r"--[^\[][^\n|\r|\n\r]*", |_|  Skip)]
    _Tok_Comment, // TODO: intern string

    Tok_Error,
    /// Token for end of file
    Tok_Eof,
}

impl fmt::Display for TokenKind<Rc<[u8]>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Kw_Break => write!(f, "Break"),
            TokenKind::Kw_Do => write!(f, "Do"),
            TokenKind::Kw_Else => write!(f, "Else"),
            TokenKind::Kw_ElseIf => write!(f, "ElseIf"),
            TokenKind::Kw_End => write!(f, "End"),
            TokenKind::Kw_Function => write!(f, "Function"),
            TokenKind::Kw_Goto => write!(f, "Goto"),
            TokenKind::Kw_If => write!(f, "If"),
            TokenKind::Kw_In => write!(f, "In"),
            TokenKind::Kw_Local => write!(f, "Local"),
            TokenKind::Kw_Nil => write!(f, "Nil"),
            TokenKind::Kw_For => write!(f, "For"),
            TokenKind::Kw_While => write!(f, "While"),
            TokenKind::Kw_Repeat => write!(f, "Repeat"),
            TokenKind::Kw_Until => write!(f, "Until"),
            TokenKind::Kw_Return => write!(f, "Return"),
            TokenKind::Kw_Then => write!(f, "Then"),
            TokenKind::Kw_Not => write!(f, "Not"),
            TokenKind::Kw_And => write!(f, "And"),
            TokenKind::Kw_Or => write!(f, "Or"),
            TokenKind::Brk_LeftCurly => write!(f, "LeftCurly"),
            TokenKind::Brk_RightCurly => write!(f, "RightCurly"),
            TokenKind::Brk_LeftSquare => write!(f, "LeftSquare"),
            TokenKind::Brk_RightSquare => write!(f, "RightSquare"),
            TokenKind::Brk_LeftParen => write!(f, "LeftParen"),
            TokenKind::Brk_RightParen => write!(f, "RightParen"),
            TokenKind::Tok_SemiColon => write!(f, "SemiColon"),
            TokenKind::Tok_Colon => write!(f, "Colon"),
            TokenKind::Tok_DoubleColon => write!(f, "DoubleColon"),
            TokenKind::Tok_Comma => write!(f, "Comma"),
            TokenKind::Op_Minus => write!(f, "Minus"),
            TokenKind::Op_Add => write!(f, "Add"),
            TokenKind::Op_Mul => write!(f, "Mul"),
            TokenKind::Op_Div => write!(f, "Div"),
            TokenKind::Op_IDiv => write!(f, "IDiv"),
            TokenKind::Op_Pow => write!(f, "Pow"),
            TokenKind::Op_Mod => write!(f, "Mod"),
            TokenKind::Op_Len => write!(f, "Len"),
            TokenKind::Op_BitXor => write!(f, "BitXor"),
            TokenKind::Op_BitAnd => write!(f, "BitAnd"),
            TokenKind::Op_BitOr => write!(f, "BitOr"),
            TokenKind::Op_ShiftRight => write!(f, "ShiftRight"),
            TokenKind::Op_ShiftLeft => write!(f, "ShiftLeft"),
            TokenKind::Op_Assign => write!(f, "Assign"),
            TokenKind::Op_Dot => write!(f, "Dot"),
            TokenKind::Op_Concat => write!(f, "Concat"),
            TokenKind::Op_Dots => write!(f, "Dots"),
            TokenKind::Op_LessThan => write!(f, "LessThan"),
            TokenKind::Op_LessEqual => write!(f, "LessEqual"),
            TokenKind::Op_GreaterThan => write!(f, "GreaterThan"),
            TokenKind::Op_GreaterEqual => write!(f, "GreaterEqual"),
            TokenKind::Op_Equal => write!(f, "Equal"),
            TokenKind::Op_NotEqual => write!(f, "NotEqual"),
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
                String::from_utf8_lossy(string.as_ref())
            ),
            TokenKind::Lit_Bool(value) => write!(f, "Bool({})", value),
            TokenKind::_Tok_MultiLineComment(comment) => write!(
                f,
                "MultiLineComment({:?})",
                String::from_utf8_lossy(comment.as_ref())
            ),
            TokenKind::_Tok_Comment => write!(f, "Comment"),
            TokenKind::_Tok_Newline(linenumber) => write!(f, "Newline({})", linenumber),
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
