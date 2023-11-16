use std::{
    error::Error,
    fs,
    io::{self, Write},
};

use anyhow::Result;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::Token;

pub mod callable;
pub mod environment;
pub mod error;
pub mod expr;
pub mod function;
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

pub fn run_prompt() -> Result<()> {
    loop {
        let mut buf = String::new();
        loop {
            print!("> ");
            io::stdout().flush()?;
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read line.");
            if buf.trim_end().ends_with(";;") {
                break;
            }
        }
        let code = buf.clone();
        run(code.as_bytes().to_vec());
        if code.trim_end().ends_with(";;;") {
            break;
        }
    }
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

    if success.is_err() {
        return Err(());
    }

    Interpreter::new().interpret(statements);
    eprintln!("interpreted");

    Ok(())
}
