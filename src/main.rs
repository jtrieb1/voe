#![allow(clippy::result_large_err)] // For now. Will be removed later.

extern crate pest;
extern crate pest_derive;

use clap::Parser;
use pest::error::Error;
use std::fs;

pub mod ast_passes;
use ast_passes::{ASTPass, ConstantFolding};
pub mod parser;
use parser::{Program, Rule, VoeParser};

struct VoeCompiler {
    parser: VoeParser,
    ast_passes: Vec<Box<dyn ASTPass>>,
}

impl VoeCompiler {
    pub fn parse(&self, source: &str) -> Result<Program, Error<Rule>> {
        self.parser.parse_program(source)
    }
    pub fn run_ast_passes(&mut self, program: Program) -> Program {
        let mut program = program;
        for pass in &mut self.ast_passes {
            program = pass.run(program);
        }
        program
    }
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    #[arg(short, long)]
    source: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    // Fetch file string.
    let unparsed_file = fs::read_to_string(args.source).expect("cannot read file");
    if args.debug {
        println!("Original program:\n\n{}\n", unparsed_file);
    }

    // Create compiler struct
    let mut compiler = VoeCompiler {
        parser: VoeParser,
        ast_passes: vec![Box::new(ConstantFolding)],
    };

    // Create AST from file string.
    let file = compiler.parse(&unparsed_file).expect("unsuccessful parse");
    if args.debug {
        println!("Parsed program:\n\n{}\n", file);
    }

    // Run AST passes.
    let file = compiler.run_ast_passes(file);
    if args.debug {
        println!("After AST passes:\n\n{}\n", file);
    }

    fs::write(args.output, format!("{}", file)).map_err(|err| {
        println!("{}", err);
    })?;
    Ok(())
}
