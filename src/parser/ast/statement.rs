use super::r#Type;
use super::Conditional;
use super::FunctionDefinition;
use super::VariableDeclaration;
use pest::error::Error;
use pest::iterators::Pair;

use super::block::parse_block;
use super::expression::parse_expression;
use super::r#type::parse_type;
use super::Expression;
use crate::parser::Rule;

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Function(FunctionDefinition),
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Conditional(Conditional),
}

impl Statement {
    pub fn function(fi: FunctionDefinition) -> Statement {
        Statement::Function(fi)
    }

    pub fn variable_declaration(v: VariableDeclaration) -> Statement {
        Statement::VariableDeclaration(v)
    }

    pub fn expression(e: Expression) -> Statement {
        Statement::Expression(e)
    }

    pub fn conditional(c: Conditional) -> Statement {
        Statement::Conditional(c)
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Function(fi) => write!(f, "{}", fi),
            Statement::VariableDeclaration(v) => write!(f, "{};", v),
            Statement::Expression(e) => write!(f, "{};", e),
            Statement::Conditional(c) => write!(f, "{}", c),
        }
    }
}

pub fn parse_inputs(pair: Pair<Rule>) -> Result<Vec<VariableDeclaration>, Error<Rule>> {
    let mut params = vec![];
    for pair in pair.into_inner() {
        let mut pair = pair.into_inner();
        let name = pair.next().unwrap().as_str().to_string();
        let ty = parse_type(pair.next().unwrap())?;
        params.push(VariableDeclaration::new(name, Some(ty), None));
    }
    Ok(params)
}

pub fn parse_statement(pair: Pair<Rule>) -> Result<Statement, Error<Rule>> {
    match pair.as_rule() {
        Rule::function_declaration => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap().as_str().to_string();
            let inputs = parse_inputs(pair.next().unwrap())?;
            let return_type: r#Type = parse_type(pair.next().unwrap())?;
            let block = parse_block(pair.next().unwrap().into_inner())?;
            Ok(Statement::Function(FunctionDefinition::new(
                name,
                inputs,
                return_type,
                block,
            )))
        }
        Rule::variable_declaration => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap().as_str().to_string();
            let mut ty = None;
            if let Some(pair) = pair.next() {
                match pair.as_rule() {
                    Rule::expression => {
                        return Ok(Statement::VariableDeclaration(VariableDeclaration::new(
                            name, None, Some(parse_expression(pair)?),
                        )));
                    }
                    _ => ty = Some(parse_type(pair)?),
                }
            }
            if let Some(pair) = pair.next() {
                return Ok(Statement::VariableDeclaration(VariableDeclaration::new(
                    name,
                    ty,
                    Some(parse_expression(pair)?),
                )));
            }
            Ok(Statement::VariableDeclaration(VariableDeclaration::new(
                name, ty, None,
            )))
        }
        Rule::expression => Ok(Statement::Expression(parse_expression(pair)?)),
        Rule::conditional => {
            let mut pair = pair.into_inner();
            let condition = parse_expression(pair.next().unwrap())?;
            let then_block = parse_block(pair.next().unwrap().into_inner())?;
            if let Some(pair) = pair.next() {
                return Ok(Statement::Conditional(Conditional::new(
                    condition,
                    then_block,
                    Some(parse_block(pair.into_inner())?),
                )));
            }
            Ok(Statement::Conditional(Conditional::new(
                condition, then_block, None,
            )))
        }
        _ => Err(Error::new_from_span(
            pest::error::ErrorVariant::CustomError {
                message: "expected statement".to_string(),
            },
            pair.as_span(),
        )),
    }
}
