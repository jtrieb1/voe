use pest::{error::Error, iterators::Pair};

use crate::parser::Rule;

use super::{statement::parse_inputs, VariableDeclaration};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Unit,
    Custom(String),
    Dependent(DType),
    Generic(GType),
}

impl Type {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "i8" => Some(Type::I8),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "f32" => Some(Type::F32),
            "f64" => Some(Type::F64),
            "bool" => Some(Type::Bool),
            "string" => Some(Type::String),
            "()" => Some(Type::Unit),
            _ => None,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Unit => write!(f, "()"),
            Type::Custom(name) => write!(f, "{}", name),
            Type::Dependent(dtype) => write!(f, "{}", dtype),
            Type::Generic(gtype) => write!(f, "{}", gtype),
        }
    }
}

pub fn parse_type(pair: Pair<Rule>) -> Result<Type, Error<Rule>> {
    match pair.as_rule() {
        Rule::ident => {
            let s = pair.as_str();
            Ok(Type::Custom(s.to_string()))
        }
        Rule::primitive_type => {
            let s = pair.as_str();
            Type::from_str(s).ok_or(Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: format!("unknown type: {}", s),
                },
                pair.as_span(),
            ))
        }
        Rule::gtype => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap().as_str();
            let fst_par = pair.next().unwrap();
            let mut fields = vec![parse_type(fst_par)?];
            for inner in pair {
                fields.push(parse_type(inner)?);
            }
            Ok(Type::Generic(GType {
                name: name.to_string(),
                fields,
            }))
        }
        Rule::dtype => {
            let mut pair = pair.into_inner();
            let params = parse_inputs(pair.next().unwrap())?;
            let name = pair.next().unwrap().as_str();
            Ok(Type::Dependent(DType {
                name: name.to_string(),
                fields: params,
            }))
        }
        _ => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: "expected type".to_string(),
            },
            pair.as_span(),
        )),
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct DType {
    name: String,
    fields: Vec<VariableDeclaration>,
}

impl std::fmt::Display for DType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "forall ")?;
        for (i, dec) in self.fields.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", dec)?;
        }
        write!(f, ". {}", self.name)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct GType {
    name: String,
    fields: Vec<Type>,
}

impl std::fmt::Display for GType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<", self.name)?;
        for (i, ty) in self.fields.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", ty)?;
        }
        write!(f, ">")
    }
}
