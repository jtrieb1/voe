use super::statement::parse_statement;
use super::Statement;
use crate::parser::Rule;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pairs;

#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Block {
        Block { statements }
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n{}\n}}",
            self.statements
                .iter()
                .map(|s| format!("    {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub fn parse_block(pairs: Pairs<Rule>) -> Result<Block, Error<Rule>> {
    let mut statements = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                statements.push(parse_statement(pair.into_inner().next().unwrap())?);
            }
            _ => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError {
                        message: "expected statement".to_string(),
                    },
                    pair.as_span(),
                ))
            }
        }
    }
    Ok(Block::new(statements))
}
