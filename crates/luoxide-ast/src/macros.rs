#![allow(unused_macros)]
#![allow(dead_code)]

macro_rules! token {
    // Keywords
    {break} => {
        $crate::token::TokenKind::Break
    };
    {do} => {
        $crate::token::TokenKind::Do
    };
    {else} => {
        $crate::token::TokenKind::Else
    };
    {else if} => {
        $crate::token::TokenKind::ElseIf
    };
    {end} => {
        $crate::token::TokenKind::End
    };
    {function} => {
        $crate::token::TokenKind::Function
    };
    {goto} => {
        $crate::token::TokenKind::Goto
    };
    {if} => {
        $crate::token::TokenKind::If
    };
    {in} => {
        $crate::token::TokenKind::In
    };
    {local} => {
        $crate::token::TokenKind::Local
    };
    {nil} => {
        $crate::token::TokenKind::Nil
    };
    {for} => {
        $crate::token::TokenKind::For
    };
    {while} => {
        $crate::token::TokenKind::While
    };
    {repeat} => {
        $crate::token::TokenKind::Repeat
    };
    {until} => {
        $crate::token::TokenKind::Until
    };
    {return} => {
        $crate::token::TokenKind::Return
    };
    {then} => {
        $crate::token::TokenKind::Then
    };
    {not} => {
        $crate::token::TokenKind::Not
    };
    {and} => {
        $crate::token::TokenKind::And
    };
    {or} => {
        $crate::token::TokenKind::Or
    };

    // Reserved keywords
    {enum} => {
        $crate::token::TokenKind::Enum
    };
    {const} => {
        $crate::token::TokenKind::Const
    };
    {auto} => {
        $crate::token::TokenKind::Auto
    };
    {global} => {
        $crate::token::TokenKind::Global
    };
    {defer} => {
        $crate::token::TokenKind::Defer
    };
    {switch} => {
        $crate::token::TokenKind::Switch
    };
    {case} => {
        $crate::token::TokenKind::Case
    };
    {fallthrough} => {
        $crate::token::TokenKind::Fallthrough
    };

    // Brackets
    {"{"} => {
        $crate::token::TokenKind::LeftCurly
    };
    {"}"} => {
        $crate::token::TokenKind::RightCurly
    };
    {"["} => {
        $crate::token::TokenKind::LeftSquare
    };
    {"]"} => {
        $crate::token::TokenKind::RightSquare
    };
    {"("} => {
        $crate::token::TokenKind::LeftParen
    };
    {")"} => {
        $crate::token::TokenKind::RightParen
    };

    // Misc Characters
    {";"} => {
        $crate::token::TokenKind::SemiColon
    };
    {":"} => {
        $crate::token::TokenKind::Colon
    };
    {"::"} => {
        $crate::token::TokenKind::DoubleColon
    };
    {","} => {
        $crate::token::TokenKind::Comma
    };
    {"."} => {
        $crate::token::TokenKind::Dot
    };
    {"..."} => {
        $crate::token::TokenKind::Dots
    };

    // Operators
    {"-"} => {
        $crate::token::TokenKind::Minus
    };
    {"+"} => {
        $crate::token::TokenKind::Plus
    };
    {"*"} => {
        $crate::token::TokenKind::Mul
    };
    {"/"} => {
        $crate::token::TokenKind::Div
    };
    {"//"} => {
        $crate::token::TokenKind::IDiv
    };
    {"^"} => {
        $crate::token::TokenKind::Pow
    };
    {"%"} => {
        $crate::token::TokenKind::Mod
    };
    {"#"} => {
        $crate::token::TokenKind::Pound
    };
    {"~"} => {
        $crate::token::TokenKind::Tilde
    };
    {"&"} => {
        $crate::token::TokenKind::Amper
    };
    {"|"} => {
        $crate::token::TokenKind::BitOr
    };
    {">>"} => {
        $crate::token::TokenKind::ShiftRight
    };
    {"<<"} => {
        $crate::token::TokenKind::ShiftLeft
    };
    {"="} => {
        $crate::token::TokenKind::Assign
    };
    {".."} => {
        $crate::token::TokenKind::Concat
    };

    // Equality operators
    {"<"} => {
        $crate::token::TokenKind::LessThan
    };
    {"<="} => {
        $crate::token::TokenKind::LessEqual
    };
    {">"} => {
        $crate::token::TokenKind::GreaterThan
    };
    {">="} => {
        $crate::token::TokenKind::GreaterEqual
    };
    {"=="} => {
        $crate::token::TokenKind::Equal
    };
    {"~="} => {
        $crate::token::TokenKind::NotEqual
    };

    // Literals
    {string} => {
        $crate::token::TokenKind::Lit_String
    };
    {multiline_string} => {
        $crate::token::TokenKind::Lit_MultilineString
    };
    {identifier} => {
        $crate::token::TokenKind::Lit_Identifier
    };
    {number} => {
        $crate::token::TokenKind::Lit_Number
    };
    {hex_number} => {
        $crate::token::TokenKind::Lit_HexNumber
    };
    {float} => {
        $crate::token::TokenKind::Lit_Float
    };
    {hex_float} => {
        $crate::token::TokenKind::Lit_HexFloat
    };
    {NaN} => {
        $crate::token::TokenKind::NaN
    };
    {true} => {
        $crate::token::TokenKind::Lit_True
    };
    {false} => {
        $crate::token::TokenKind::Lit_False
    };

    // Meta
    {Comment} => {
        $crate::token::TokenKind::_Tok_Comment
    };
    {MultilineComment} => {
        $crate::token::TokenKind::_Tok_MultilineComment
    };
    {Error} => {
        $crate::token::TokenKind::Tok_Error
    };
    {EOF} => {
        $crate::token::TokenKind::Tok_Eof
    };

    // Reserved
    {reserved_set} => {
        [
            token!{enum},
            token!{const},
            token!{auto},
            token!{global},
            token!{defer},
            token!{switch},
            token!{case},
            token!{fallthrough},
        ]
    };
    {reserved} => {
          token!{enum}
        | token!{const}
        | token!{auto}
        | token!{global}
        | token!{defer}
        | token!{switch}
        | token!{case}
        | token!{fallthrough}
    }
}