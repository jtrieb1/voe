#![allow(clippy::result_large_err)] // For now. Will be removed later.

extern crate pest;
extern crate pest_derive;

use clap::Parser;
use std::fs;

pub mod parser;
use parser::parse;

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

    // Create AST from file string.
    let file = parse(&unparsed_file).expect("unsuccessful parse");
    if args.debug {
        println!("Parsed program:\n\n{}", file);
    }


    fs::write(args.output, format!("{}", file)).map_err(|err| {
        println!("{}", err);
    })?;
    Ok(())
}
