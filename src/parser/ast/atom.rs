use pest::{error::Error, iterators::Pair};

use crate::parser::Rule;

use super::{expression::parse_expression, Expression, Type};

#[derive(PartialEq, Debug, Clone)]
pub struct Atom {
    pub negative: bool,
    pub value: AtomValue,
    pub ty: Option<Type>,
}

impl Atom {
    pub fn new(negative: bool, value: AtomValue, ty: Option<Type>) -> Atom {
        Atom {
            negative,
            value,
            ty,
        }
    }

    pub fn get_type(&self) -> Option<Type> {
        self.ty.clone()
    }

    pub fn set_type(&mut self, ty: Type) {
        self.ty = Some(ty);
    }

    pub fn from_i128(i: i128, ty: Option<Type>) -> Atom {
        Atom {
            negative: i < 0,
            value: AtomValue::Integer(i),
            ty,
        }
    }

    pub fn from_f64(f: f64, ty: Option<Type>) -> Atom {
        Atom {
            negative: f < 0.0,
            value: AtomValue::Float(f),
            ty,
        }
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            if self.negative { "-" } else { "" },
            match &self.value {
                AtomValue::Integer(i) => i.abs().to_string(),
                AtomValue::Float(f) => f.abs().to_string(),
                AtomValue::String(s) => s.to_string(),
                AtomValue::Boolean(b) => b.to_string(),
                AtomValue::Identity(i) => i.to_string(),
                AtomValue::ParExpr(e) => format!("({})", e),
            },
            if self.ty.is_some() {
                format!("{}", self.ty.as_ref().unwrap())
            } else {
                "".to_string()
            }
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AtomValue {
    Integer(i128),
    Float(f64),
    String(String),
    Boolean(bool),
    Identity(String),
    ParExpr(Box<Expression>),
}

fn parse_integer(pair: Pair<Rule>) -> Result<AtomValue, Error<Rule>> {
    let repr = pair.as_str();
    let var: i128 = repr.parse().unwrap();
    Ok(AtomValue::Integer(var))
}

fn parse_float(pair: Pair<Rule>) -> Result<AtomValue, Error<Rule>> {
    let repr = pair.as_str();
    let var: f64 = repr.parse().unwrap();
    Ok(AtomValue::Float(var))
}

pub fn parse_atom(pair: Pair<Rule>) -> Result<Atom, Error<Rule>> {
    let mut pair = pair.into_inner();
    let mut next = pair.next().unwrap();

    let negative = next.as_rule() == Rule::unary_minus;
    if negative {
        next = pair.next().unwrap();
    }

    let mut ty = None;
    let value = match next.as_rule() {
        Rule::numeric => {
            let mut inner = next.into_inner();
            let next = inner.next().unwrap();
            let val = match next.as_rule() {
                Rule::integer => parse_integer(next)?,
                Rule::decimal => parse_float(next)?,
                _ => Err(Error::new_from_span(
                    pest::error::ErrorVariant::CustomError {
                        message: "expected numeric".to_string(),
                    },
                    next.as_span(),
                ))?,
            };
            ty = inner.next().map(|t| Type::parse_type(t.as_str()));
            val
        }
        Rule::string => AtomValue::String(next.as_str().to_string()),
        Rule::bool => AtomValue::Boolean(next.as_str() == "true"),
        Rule::ident => AtomValue::Identity(next.as_str().to_string()),
        Rule::expression => AtomValue::ParExpr(Box::new(parse_expression(next)?)),
        _ => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: "expected atom".to_string(),
            },
            next.as_span(),
        ))?,
    };

    Ok(Atom {
        negative,
        value,
        ty: ty.flatten(),
    })
}
