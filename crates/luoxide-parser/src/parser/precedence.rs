use luoxide_ast::operator::BinaryOperator;

pub const UNARY_PRIORITY: u8 = 12;

/// Precedence is a tuple of two values: the left and right precedence
/// of an operator.
///
/// Left precedence is the precedence of the operator in the expression
/// before the operator. Right precedence is the precedence of the
/// operator in the expression after the operator.
pub struct Precedence {
    pub left: u8,
    pub right: u8,
}

impl Precedence {
    pub fn new(left: u8, right: u8) -> Precedence {
        Precedence { left, right }
    }

    pub fn get_associativity(&self) -> Associativity {
        if self.left <= self.right {
            return Associativity::Left;
        }
        Associativity::Right
    }

    pub fn from_binary_operator(op: &BinaryOperator) -> Precedence {
        match op {
            BinaryOperator::Pow => (10, 9).into(),

            BinaryOperator::Mul | BinaryOperator::Div | BinaryOperator::Mod => (7, 7).into(),

            BinaryOperator::Add | BinaryOperator::Sub => (6, 6).into(),

            BinaryOperator::BitAnd | BinaryOperator::BitXor => (5, 5).into(),

            BinaryOperator::Concat => (5, 4).into(),

            BinaryOperator::BitOr => (4, 4).into(),

            BinaryOperator::Equal
            | BinaryOperator::LessThan
            | BinaryOperator::LessThanEqual
            | BinaryOperator::NotEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanEqual => (3, 3).into(),

            BinaryOperator::And => (2, 2).into(),

            BinaryOperator::Or => (1, 1).into(),
        }
    }
}

pub enum Associativity {
    Left,
    Right,
}

impl From<(u8, u8)> for Precedence {
    fn from(pair: (u8, u8)) -> Self {
        Precedence::new(pair.0, pair.1)
    }
}
