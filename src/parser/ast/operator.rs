use super::{atom::Atom, Type};

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
    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            Operator::Equal
                | Operator::NotEqual
                | Operator::LessThan
                | Operator::LessThanOrEqual
                | Operator::GreaterThan
                | Operator::GreaterThanOrEqual
        )
    }

    pub fn return_type(&self, ctx_rhs: &Atom, ctx_lhs: &Atom) -> Option<Type> {
        match self {
            Operator::Equal
            | Operator::NotEqual
            | Operator::LessThan
            | Operator::LessThanOrEqual
            | Operator::GreaterThan
            | Operator::GreaterThanOrEqual => Some(Type::Bool),
            Operator::And | Operator::Or | Operator::LogicalAnd | Operator::LogicalOr => {
                Some(Type::Bool)
            }
            Operator::Not | Operator::Neg => None,
            _ => {
                let (tyl, tyr) = (ctx_lhs.ty.clone()?, ctx_rhs.ty.clone()?);
                if tyl.is_integral() && tyr.is_integral() || tyl.is_decimal() && tyr.is_decimal() {
                    tyl.join(&tyr)
                } else {
                    None
                }
            }
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
