use crate::parser::Rule;

use super::statement::parse_statement;
use super::Statement;
use pest::error::Error;
use pest::iterators::Pair;

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        Program { statements }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.statements
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub fn parse_program(pairs: Pair<Rule>) -> Result<Program, Error<Rule>> {
    let mut statements = vec![];
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::statement => {
                statements.push(parse_statement(pair.into_inner().next().unwrap())?);
            }
            Rule::EOI => {
                return Ok(Program::new(statements));
            }
            _ => {
                return Err(Error::new_from_span(
                    pest::error::ErrorVariant::CustomError {
                        message: "expected statement".to_string(),
                    },
                    pair.as_span(),
                ))
            }
        }
    }
    Ok(Program::new(statements))
}
