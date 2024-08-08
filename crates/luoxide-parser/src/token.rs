use logos::{Logos, Skip};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
/*
    Extras:
    (last_line_number, start_of_last_line, begin_line_number, start_of_begin_line)
*/
#[logos(extras = (usize, usize, usize, usize))]
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

    Tok_Error(LexingError),
    /// Token for end of file
    Tok_Eof,
}
