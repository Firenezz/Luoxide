use luoxide_text::range::TextSpan;

use expressions::*;
pub mod expressions {
    use super::*;

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub struct Expression {
        pub kind: ExpressionKind,
        pub span: TextSpan,
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub enum ExpressionKind {
        Literal(Literal),
    }

    #[cfg_attr(any(test, debug_assertions, __derive_debug), derive(Debug))]
    #[derive(Clone)]
    pub enum Literal {
        Nil,
        // Max is i32 but for ease of implementation we use i64 for now
        /// Represent a number in the range -2^31 to 2^31 - 1
        Int(i64),
        /// Represent a floating number in the range -2^63 to 2^63 - 1
        Float(f64),
        /// Represent a boolean
        /// `true` and `false`
        Bool(bool),
        /// Represent a string
        ///
        /// This comes from a interner
        String(String),
    }
}
