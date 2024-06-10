macro_rules! token {
    // Keywords
    {break} => {
        $crate::internal::syntax::lexer::TokenKind::Break
    };
    {do} => {
        $crate::internal::syntax::lexer::TokenKind::Do
    };
    {else} => {
        $crate::internal::syntax::lexer::TokenKind::Else
    };
    {else if} => {
        $crate::internal::syntax::lexer::TokenKind::ElseIf
    };
    {end} => {
        $crate::internal::syntax::lexer::TokenKind::End
    };
    {function} => {
        $crate::internal::syntax::lexer::TokenKind::Function
    };
    {goto} => {
        $crate::internal::syntax::lexer::TokenKind::Goto
    };
    {if} => {
        $crate::internal::syntax::lexer::TokenKind::If
    };
    {in} => {
        $crate::internal::syntax::lexer::TokenKind::In
    };
    {local} => {
        $crate::internal::syntax::lexer::TokenKind::Local
    };
    {nil} => {
        $crate::internal::syntax::lexer::TokenKind::Nil
    };
    {for} => {
        $crate::internal::syntax::lexer::TokenKind::For
    };
    {while} => {
        $crate::internal::syntax::lexer::TokenKind::While
    };
    {repeat} => {
        $crate::internal::syntax::lexer::TokenKind::Repeat
    };
    {until} => {
        $crate::internal::syntax::lexer::TokenKind::Until
    };
    {return} => {
        $crate::internal::syntax::lexer::TokenKind::Return
    };
    {then} => {
        $crate::internal::syntax::lexer::TokenKind::Then
    };
    {not} => {
        $crate::internal::syntax::lexer::TokenKind::Not
    };
    {and} => {
        $crate::internal::syntax::lexer::TokenKind::And
    };
    {or} => {
        $crate::internal::syntax::lexer::TokenKind::Or
    };

    // Brackets
    {"{"} => {
        $crate::internal::syntax::lexer::TokenKind::LeftCurly
    };
    {"}"} => {
        $crate::internal::syntax::lexer::TokenKind::RightCurly
    };
    {"["} => {
        $crate::internal::syntax::lexer::TokenKind::LeftSquare
    };
    {"]"} => {
        $crate::internal::syntax::lexer::TokenKind::RightSquare
    };
    {"{"} => {
        $crate::internal::syntax::lexer::TokenKind::LeftParen
    };
    {"}"} => {
        $crate::internal::syntax::lexer::TokenKind::RightParen
    };

    // Misc Characters
    {";"} => {
        $crate::internal::syntax::lexer::TokenKind::Semicolon
    };
    {":"} => {
        $crate::internal::syntax::lexer::TokenKind::Colon
    };
    {"::"} => {
        $crate::internal::syntax::lexer::TokenKind::DoubleColon
    };
    {","} => {
        $crate::internal::syntax::lexer::TokenKind::Comma
    };
    {"."} => {
        $crate::internal::syntax::lexer::TokenKind::Dot
    };
    {"..."} => {
        $crate::internal::syntax::lexer::TokenKind::Dots
    };

    // Operators
    {"-"} => {
        $crate::internal::syntax::lexer::TokenKind::Minus
    };
    {"+"} => {
        $crate::internal::syntax::lexer::TokenKind::Plus
    };
    {"*"} => {
        $crate::internal::syntax::lexer::TokenKind::Mul
    };
    {"/"} => {
        $crate::internal::syntax::lexer::TokenKind::Div
    };
    {"//"} => {
        $crate::internal::syntax::lexer::TokenKind::IDiv
    };
    {"^"} => {
        $crate::internal::syntax::lexer::TokenKind::Pow
    };
    {"%"} => {
        $crate::internal::syntax::lexer::TokenKind::Mod
    };
    {"#"} => {
        $crate::internal::syntax::lexer::TokenKind::Pound
    };
    {"~"} => {
        $crate::internal::syntax::lexer::TokenKind::BitXor
    };
    {"&"} => {
        $crate::internal::syntax::lexer::TokenKind::BitAnd
    };
    {"|"} => {
        $crate::internal::syntax::lexer::TokenKind::BitOr
    };
    {">>"} => {
        $crate::internal::syntax::lexer::TokenKind::ShiftRight
    };
    {"<<"} => {
        $crate::internal::syntax::lexer::TokenKind::ShiftLeft
    };
    {"="} => {
        $crate::internal::syntax::lexer::TokenKind::Assign
    };
    {".."} => {
        $crate::internal::syntax::lexer::TokenKind::Concat
    };

    // Equality operators
    {"<"} => {
        $crate::internal::syntax::lexer::TokenKind::LessThan
    };
    {"<="} => {
        $crate::internal::syntax::lexer::TokenKind::LessEqual
    };
    {">"} => {
        $crate::internal::syntax::lexer::TokenKind::GreaterThan
    };
    {">="} => {
        $crate::internal::syntax::lexer::TokenKind::GreaterEqual
    };
    {"=="} => {
        $crate::internal::syntax::lexer::TokenKind::Equal
    };
    {"~="} => {
        $crate::internal::syntax::lexer::TokenKind::NotEqual
    };

    // Meta
    {Error} => {
        $crate::internal::syntax::lexer::TokenKind::Tok_Error
    };
    {EOF} => {
        $crate::internal::syntax::lexer::TokenKind::Tok_EOF
    };
}
