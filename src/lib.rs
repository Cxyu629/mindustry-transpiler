use std::{error::Error, fs};

use token::Token;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub mod environment;
pub mod error;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod stmt;
pub mod token;

pub mod macros;

pub fn run_file(filepath: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read(filepath)?;

    let _ = run(contents);

    Ok(())
}

pub fn run(source: Vec<u8>) -> Result<(), ()> {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    eprintln!("scanned");
    // eprintln!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let (statements, success) = parser.parse();
    println!("statements: {}", statements.len());
    for statement in (&statements).into_iter() {
        println!("{}", statement);
    }
    eprintln!("parsed");

    if let Err(_) = success {
        return Err(());
    }

    Interpreter::new().interpret(statements);
    eprintln!("interpreted");

    Ok(())
}
