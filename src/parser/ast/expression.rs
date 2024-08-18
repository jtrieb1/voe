use pest::error::Error;
use pest::iterators::Pair;
use once_cell::sync::Lazy;
use super::{atom::{AtomValue, parse_atom, Atom}, Operator};
use crate::parser::Rule;

use pest::pratt_parser::{Assoc::*, Op, PrattParser};

pub static EXPRESSION_PARSER: Lazy<PrattParser<Rule>> = Lazy::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::bitwise_and, Left) | Op::infix(Rule::bitwise_or, Left) | Op::infix(Rule::logical_and, Left) | Op::infix(Rule::logical_or, Left))
        .op(Op::infix(Rule::eq, Left) | Op::infix(Rule::ne, Left) | Op::infix(Rule::lt, Left) | Op::infix(Rule::le, Left) | Op::infix(Rule::gt, Left) | Op::infix(Rule::ge, Left))
        .op(Op::infix(Rule::add, Left) | Op::infix(Rule::sub, Left))
        .op(Op::infix(Rule::mul, Left) | Op::infix(Rule::div, Left) | Op::infix(Rule::r#mod, Left) | Op::infix(Rule::pow, Right))
        .op(Op::prefix(Rule::unary_minus) | Op::prefix(Rule::not))
});

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
    Atom(Atom),
}

impl Expression {
    pub fn binary(lhs: Expression, op: Operator, rhs: Expression) -> Expression {
        Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs))
    }

    pub fn atom(atom: Atom) -> Expression {
        Expression::Atom(atom)
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BinaryOperation(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Expression::Atom(atom) => write!(f, "{}", atom),
        }
    }
}

pub fn parse_expression(pair: Pair<Rule>) -> Result<Expression, Error<Rule>> {
    EXPRESSION_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::atom => Ok(Expression::Atom(parse_atom(primary)?)),
            _ => Err(Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: "expected atom".to_string(),
                },
                primary.as_span(),
            )),
        })
        .map_prefix(|pf, t| match pf.as_rule() {
            Rule::unary_minus => {
                let expr = t?;
                match expr {
                    Expression::Atom(atom) => {
                        Ok(Expression::Atom(Atom::new(!atom.negative, atom.value, atom.ty)))
                    }
                    Expression::BinaryOperation(..) => Ok(Expression::Atom(Atom::new(
                        true,
                        AtomValue::ParExpr(Box::new(expr)),
                        None
                    ))),
                }
            }
            _ => Err(Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: "expected unary operator".to_string(),
                },
                pf.as_span(),
            )),
        })
        .map_infix(|lhs, op, rhs| {
            let lhs = lhs?;
            let rhs = rhs?;
            let op = match op.as_rule() {
                Rule::add => Operator::Add,
                Rule::sub => Operator::Subtract,
                Rule::mul => Operator::Multiply,
                Rule::div => Operator::Divide,
                Rule::r#mod => Operator::Modulo,
                Rule::pow => Operator::Pow,
                Rule::eq => Operator::Equal,
                Rule::ne => Operator::NotEqual,
                Rule::lt => Operator::LessThan,
                Rule::le => Operator::LessThanOrEqual,
                Rule::gt => Operator::GreaterThan,
                Rule::ge => Operator::GreaterThanOrEqual,
                Rule::not => Operator::Not,
                _ => unreachable!(),
            };
            Ok(Expression::BinaryOperation(
                Box::new(lhs),
                op,
                Box::new(rhs),
            ))
        })
        .parse(pair.into_inner())
}
