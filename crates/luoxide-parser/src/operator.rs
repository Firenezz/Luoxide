use luoxide_ast::operator::BinaryOperator;

use crate::{parser::precedence::Precedence, token::TokenKind};



/* Predecedence table from https://github.com/fnuecke/eris/blob/master/src/lparser.c
   {6, 6}, {6, 6}, {7, 7}, {7, 7}, {7, 7},  /* `+' `-' `*' `/' `%' */
   {10, 9}, {5, 4},                 /* ^, .. (right associative) */
   {3, 3}, {3, 3}, {3, 3},          /* ==, <, <= */
   {3, 3}, {3, 3}, {3, 3},          /* ~=, >, >= */
   {2, 2}, {1, 1}                   /* and, or */

   UNARY_PRIORITY	12
*/

impl From<TokenKind> for BinaryOperator {
    fn from(kind: TokenKind) -> Self {
        match kind {
            token!("+") => BinaryOperator::Add,
            token!("-") => BinaryOperator::Sub,
            token!("*") => BinaryOperator::Mul,
            token!("/") => BinaryOperator::Div,
            token!("%") => BinaryOperator::Mod,
            token!("&") => BinaryOperator::BitAnd,
            token!("|") => BinaryOperator::BitOr,
            token!("^") => BinaryOperator::BitXor,
            token!("==") => BinaryOperator::Equal,
            //token!("!=") => BinaryOperator::Ne, // TODO: Not equal
            token!(">") => BinaryOperator::GreaterThan,
            token!(">=") => BinaryOperator::GreaterThanEqual,
            token!("<") => BinaryOperator::LessThan,
            token!("<=") => BinaryOperator::LessThanEqual,
            _ => todo!(),
        }
    }
}

pub enum UnaryOperator {
    Neg,
    BitNot,
    Len,
    Not,
}