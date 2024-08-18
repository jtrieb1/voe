#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pow,
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Not,
    Neg,
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Or | Operator::LogicalOr => 1,
            Operator::And | Operator::LogicalAnd => 2,
            Operator::Equal
            | Operator::NotEqual
            | Operator::LessThan
            | Operator::LessThanOrEqual
            | Operator::GreaterThan
            | Operator::GreaterThanOrEqual => 3,
            Operator::Add | Operator::Subtract => 4,
            Operator::Multiply | Operator::Divide | Operator::Modulo | Operator::Pow => 5,
            Operator::Not | Operator::Neg => 6,
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Modulo => write!(f, "%"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::And => write!(f, "&"),
            Operator::Or => write!(f, "|"),
            Operator::LogicalAnd => write!(f, "&&"),
            Operator::LogicalOr => write!(f, "||"),
            Operator::Not => write!(f, "!"),
            Operator::Neg => write!(f, "-"),
            Operator::Pow => write!(f, "^"),
        }
    }
}
