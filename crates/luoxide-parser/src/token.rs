use core::fmt;
use std::{borrow::Borrow, mem::discriminant, num::ParseIntError};

use logos::{Logos, Skip};
use luoxide_text::{range::TextSpan, traits::Ranged};

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
#[logos(utf8 = true)]
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

    /// Returns tokens that are likely to be typed accidentally instead of the current token.
    /// Enables better error recovery when the wrong token is found.
    pub fn similar_tokens(&self) -> &[TokenKind] {
        match self {
            TokenKind::Comma => &[TokenKind::Dot, TokenKind::SemiColon],
            TokenKind::Dot => &[TokenKind::Comma, TokenKind::SemiColon],
            TokenKind::SemiColon => &[TokenKind::Comma, TokenKind::Dot],
            _ => &[],
        }
    }

    pub fn is_delimiter(&self) -> bool {
        self.open_delimiter().is_some() || self.close_delimiter().is_some()
    }

    pub fn open_delimiter(&self) -> Option<TokenKind> {
        match self {
            TokenKind::LeftCurly => Some(TokenKind::RightCurly),
            TokenKind::LeftSquare => Some(TokenKind::RightSquare),
            TokenKind::LeftParen => Some(TokenKind::RightParen),
            _ => None,
        }
    }
    
    pub fn close_delimiter(&self) -> Option<TokenKind> {
        match self {
            TokenKind::RightCurly => Some(TokenKind::LeftCurly),
            TokenKind::RightSquare => Some(TokenKind::LeftSquare),
            TokenKind::RightParen => Some(TokenKind::LeftParen),
            _ => None,
        }
    }

    /// Lua spelling, or a placeholder for tokens that are not a fixed lexeme.
    pub const fn as_lua(self) -> &'static str {
        match self {
            TokenKind::Break => "break",
            TokenKind::Do => "do",
            TokenKind::Else => "else",
            TokenKind::ElseIf => "elseif",
            TokenKind::End => "end",
            TokenKind::Function => "function",
            TokenKind::Goto => "goto",
            TokenKind::If => "if",
            TokenKind::In => "in",
            TokenKind::Local => "local",
            TokenKind::Nil => "nil",
            TokenKind::For => "for",
            TokenKind::While => "while",
            TokenKind::Repeat => "repeat",
            TokenKind::Until => "until",
            TokenKind::Return => "return",
            TokenKind::Then => "then",
            TokenKind::Not => "not",
            TokenKind::And => "and",
            TokenKind::Or => "or",
            TokenKind::Enum => "enum",
            TokenKind::Const => "const",
            TokenKind::Auto => "auto",
            TokenKind::Global => "global",
            TokenKind::Defer => "defer",
            TokenKind::Switch => "switch",
            TokenKind::Case => "case",
            TokenKind::Fallthrough => "fallthrough",
            TokenKind::LeftCurly => "{",
            TokenKind::RightCurly => "}",
            TokenKind::LeftSquare => "[",
            TokenKind::RightSquare => "]",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::SemiColon => ";",
            TokenKind::Colon => ":",
            TokenKind::DoubleColon => "::",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Dots => "...",
            TokenKind::At => "@",
            TokenKind::Minus => "-",
            TokenKind::Add => "+",
            TokenKind::Mul => "*",
            TokenKind::Div => "/",
            TokenKind::IDiv => "//",
            TokenKind::Pow => "^",
            TokenKind::Mod => "%",
            TokenKind::Pound => "#",
            TokenKind::Tilde => "~",
            TokenKind::Amper => "&",
            TokenKind::BitOr => "|",
            TokenKind::ShiftRight => ">>",
            TokenKind::ShiftLeft => "<<",
            TokenKind::Assign => "=",
            TokenKind::Concat => "..",
            TokenKind::LessThan => "<",
            TokenKind::LessEqual => "<=",
            TokenKind::GreaterThan => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Equal => "==",
            TokenKind::NotEqual => "~=",
            TokenKind::Lit_Identifier => "name",
            TokenKind::Lit_Number => "integer",
            TokenKind::Lit_HexNumber => "hex integer",
            TokenKind::Lit_Float => "number",
            TokenKind::Lit_HexFloat => "hex number",
            TokenKind::Lit_String => "string",
            TokenKind::Lit_MultilineString => "long string",
            TokenKind::Lit_True => "true",
            TokenKind::Lit_False => "false",
            TokenKind::NaN => "NaN",
            TokenKind::_Tok_MultilineComment => "comment",
            TokenKind::_Tok_Comment => "comment",
            TokenKind::_Newline => "newline",
            TokenKind::Tok_Error => "invalid token",
            TokenKind::Tok_Eof => "end of file",
            TokenKind::_Unknown => "unknown token",
        }
    }

    /// Variant name plus Lua spelling, for compiler-facing diagnostics.
    pub fn describe(self) -> String {
        format!("{self:?} ({})", self.as_lua())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum LexingError {
    UnterminatedMultiline(MultilineKind),
    InvalidUtf8Char,
    InvalidInteger {
        err: ParseIntError,
        source: TextSpan,
    },
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
    use logos::{FilterResult, Lexer as LogosLexer};

    /// Consumes the body and closing bracket of a `[=*[ ... ]=*]` sequence.
    ///
    /// The opening bracket (with `level` `=` characters) has already been
    /// matched by the token regex; this scans the remaining source for the
    /// first closing bracket of the same level and extends the token over it.
    /// Line-count extras are updated for every newline consumed.
    fn consume_long_bracket(
        lex: &mut LogosLexer<TokenKind>,
        level: usize,
        kind: MultilineKind,
    ) -> FilterResult<(), LexingError> {
        let remainder = lex.remainder();
        let bytes = remainder.as_bytes();

        let mut newlines = 0usize;
        let mut i = 0usize;
        while i < bytes.len() {
            match bytes[i] {
                b']' => {
                    let equals_start = i + 1;
                    let mut j = equals_start;
                    while j < bytes.len() && bytes[j] == b'=' {
                        j += 1;
                    }
                    if j - equals_start == level && j < bytes.len() && bytes[j] == b']' {
                        // Include the closing bracket in the token.
                        lex.bump(j + 1);
                        lex.extras.0 += newlines;
                        lex.extras.1 = lex.span().end;
                        return FilterResult::Emit(());
                    }
                    i += 1;
                }
                b'\n' => {
                    newlines += 1;
                    i += 1;
                }
                _ => i += 1,
            }
        }

        // Unterminated: consume everything so lexing terminates.
        lex.bump(bytes.len());
        lex.extras.0 += newlines;
        FilterResult::Error(LexingError::UnterminatedMultiline(kind))
    }

    /// Callback for `Lit_MultilineString`; the regex matched `[=*[`.
    pub(super) fn long_string_callback(
        lex: &mut LogosLexer<TokenKind>,
    ) -> FilterResult<(), LexingError> {
        let level = lex.slice().len() - 2;
        consume_long_bracket(lex, level, MultilineKind::String)
    }

    /// Callback for `_Tok_MultilineComment`; the regex matched `--[=*[`.
    pub(super) fn multiline_comment_callback(
        lex: &mut LogosLexer<TokenKind>,
    ) -> FilterResult<(), LexingError> {
        let level = lex.slice().len() - 4;
        consume_long_bracket(lex, level, MultilineKind::Comment)
    }

    pub(super) fn increment_line_number(lexer: &mut logos::Lexer<'_, TokenKind>) {
        let extras = &mut lexer.extras;
        extras.0 += 1;
        extras.1 = 0;
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
