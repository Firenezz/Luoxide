#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Pow,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Concat,
    BitAnd,
    BitOr,
    BitXor,
    // TODO: Think about shifts
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    And,
    Or,
}

/* Predecedence table from https://github.com/fnuecke/eris/blob/master/src/lparser.c
   {6, 6}, {6, 6}, {7, 7}, {7, 7}, {7, 7},  /* `+' `-' `*' `/' `%' */
   {10, 9}, {5, 4},                 /* ^, .. (right associative) */
   {3, 3}, {3, 3}, {3, 3},          /* ==, <, <= */
   {3, 3}, {3, 3}, {3, 3},          /* ~=, >, >= */
   {2, 2}, {1, 1}                   /* and, or */

   UNARY_PRIORITY	12
*/

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    BitNot,
    Len,
    Not,
}
