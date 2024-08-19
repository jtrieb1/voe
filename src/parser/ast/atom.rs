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
            if self.ty.is_some() && self.value.is_simple() {
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

impl AtomValue {
    pub fn is_simple(&self) -> bool {
        matches!(
            self,
            AtomValue::Integer(_) | AtomValue::Float(_)
        )
    }
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
            ty = inner.next().map(|t| Type::parse_type(t.as_str())).flatten();
            val
        }
        Rule::string => {
            let s = next.as_str();
            let val = AtomValue::String(s[1..s.len() - 1].to_string()); // Remove quotes
            ty = Some(Type::String);
            val
        },
        Rule::bool => {
            ty = Some(Type::Bool);
            AtomValue::Boolean(next.as_str() == "true")
        }
        Rule::ident => AtomValue::Identity(next.as_str().to_string()),
        Rule::expression => {
            let expr = parse_expression(next)?;
            ty = expr.return_type();
            if let Expression::Atom(atom) = expr {
                return Ok(atom);
            }
            AtomValue::ParExpr(Box::new(expr))
        },
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
        ty,
    })
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;
    use crate::VoeParser;

    #[test]
    fn test_parse_atom() {
        let input = "123i32";
        let mut pair = VoeParser::parse(Rule::atom, input).expect("No atom recognized");
        let atom = parse_atom(pair.next().unwrap()).unwrap();
        assert_eq!(
            atom,
            Atom {
                negative: false,
                value: AtomValue::Integer(123),
                ty: Some(Type::I32)
            }
        );

        let input = "-123i32";
        let mut pair = VoeParser::parse(Rule::atom, input).expect("No atom recognized");
        let atom = parse_atom(pair.next().unwrap()).unwrap();
        assert_eq!(
            atom,
            Atom {
                negative: true,
                value: AtomValue::Integer(123),
                ty: Some(Type::I32)
            }
        );

        let input = "123.456f32";
        let mut pair = VoeParser::parse(Rule::atom, input).expect("No atom recognized");
        let atom = parse_atom(pair.next().unwrap()).unwrap();
        assert_eq!(
            atom,
            Atom {
                negative: false,
                value: AtomValue::Float(123.456),
                ty: Some(Type::F32)
            }
        );

        let input = "-123.456f64";
        let mut pair = VoeParser::parse(Rule::atom, input).expect("No atom recognized");
        let atom = parse_atom(pair.next().unwrap()).unwrap();
        assert_eq!(
            atom,
            Atom {
                negative: true,
                value: AtomValue::Float(123.456),
                ty: Some(Type::F64)
            }
        );

        let input = "\"hello\"";
        let mut pair = VoeParser::parse(Rule::atom, input).expect("No atom recognized");
        let atom = parse_atom(pair.next().unwrap()).unwrap();
        assert_eq!(
            atom,
            Atom {
                negative: false,
                value: AtomValue::String("hello".to_string()),
                ty: Some(Type::String)
            }
        );

        let input = "true";
        let mut pair = VoeParser::parse(Rule::atom, input).expect("No atom recognized");
        let atom = parse_atom(pair.next().unwrap()).unwrap();
        assert_eq!(
            atom,
            Atom {
                negative: false,
                value: AtomValue::Boolean(true),
                ty: Some(Type::Bool)
            }
        );

        let input = "false";
        let mut pair = VoeParser::parse(Rule::atom, input).expect("No atom recognized");
        let atom = parse_atom(pair.next().unwrap()).unwrap();
        assert_eq!(
            atom,
            Atom {
                negative: false,
                value: AtomValue::Boolean(false),
                ty: Some(Type::Bool)
            }
        );
    }
}