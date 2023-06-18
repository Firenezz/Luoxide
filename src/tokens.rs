use core::fmt;



#[derive(Clone, PartialEq)]
pub enum Token<S> {
    // Literals
    True,
    False,
    Nil,
    Number(S),
    String(S),

    // Identifiers
    Identifier(S),

    // Arithmetic Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,

    // Logical Operators
    Not,
    And,
    Or,

    // Bitwise Operators
    BitwiseNotXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,

    // Relational Operators
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Other Operators
    Concatenate,
    Length,
    Assignment,

    // Delimiters
    Comma,
    Dot,
    Dots,
    Colon,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Control Structures
    Break,
    Then,
    If,
    Else,
    ElseIf,
    End,
    Do,
    For,
    Return,
    While,
    Repeat,
    Until,

    // Keywords
    Function,
    Local,
    In,
    Goto,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo<S> {
    pub token: Token<S>,
    pub span: Span,
}

impl<S: fmt::Debug> fmt::Debug for Token<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Nil => write!(f, "Nil"),
            Token::Number(value) => write!(f, "Number({:?})", value),
            Token::String(value) => write!(f, "String({:?})", value),
            Token::Identifier(value) => write!(f, "Identifier({:?})", value),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Multiply => write!(f, "Multiply"),
            Token::Divide => write!(f, "Divide"),
            Token::Modulo => write!(f, "Modulo"),
            Token::Power => write!(f, "Power"),
            Token::Concatenate => write!(f, "Concatenate"),
            Token::Length => write!(f, "Length"),
            Token::Not => write!(f, "Not"),
            Token::And => write!(f, "And"),
            Token::Or => write!(f, "Or"),
            Token::Assignment => write!(f, "Assignment"),
            Token::BitwiseNotXor => write!(f, "BitwiseNotXor"),
            Token::BitwiseAnd => write!(f, "BitwiseAnd"),
            Token::BitwiseOr => write!(f, "BitwiseOr"),
            Token::BitwiseXor => write!(f, "BitwiseXor"),
            Token::BitwiseNot => write!(f, "BitwiseNot"),
            Token::LeftShift => write!(f, "LeftShift"),
            Token::RightShift => write!(f, "RightShift"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThan => write!(f, "LessThan"),
            Token::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            Token::Comma => write!(f, "Comma"),
            Token::Dot => write!(f, "Dot"),
            Token::Dots => write!(f, "Dots"),
            Token::Colon => write!(f, "Colon"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::LeftParenthesis => write!(f, "LeftParenthesis"),
            Token::RightParenthesis => write!(f, "RightParenthesis"),
            Token::LeftBrace => write!(f, "LeftBrace"),
            Token::RightBrace => write!(f, "RightBrace"),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::Break => write!(f, "Break"),
            Token::Then => write!(f, "Then"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::ElseIf => write!(f, "ElseIf"),
            Token::End => write!(f, "End"),
            Token::Do => write!(f, "Do"),
            Token::For => write!(f, "For"),
            Token::Return => write!(f, "Return"),
            Token::While => write!(f, "While"),
            Token::Repeat => write!(f, "Repeat"),
            Token::Until => write!(f, "Until"),
            Token::Function => write!(f, "Function"),
            Token::Local => write!(f, "Local"),
            Token::In => write!(f, "In"),
            Token::Goto => write!(f, "Goto"),
        }
    }
}