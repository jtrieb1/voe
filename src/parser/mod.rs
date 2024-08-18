use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

mod ast;
use ast::*;
use program::parse_program;

#[derive(Parser)]
#[grammar = "voe.pest"]
pub struct VoeParser;

pub fn parse(source: &str) -> Result<Program, Error<Rule>> {
    let mut pairs = VoeParser::parse(Rule::program, source)?;
    parse_program(pairs.next().ok_or(pest::error::Error::new_from_pos(
        pest::error::ErrorVariant::CustomError {
            message: "expected program".to_string(),
        },
        pest::Position::from_start(source),
    ))?)
}