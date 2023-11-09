use std::{error::Error, fs};

use error::EvaluationError;
use error::ParseError;
use parser::Parser;
use scanner::Scanner;
use token::Token;
use token::TokenType as TT;

use crate::interpreter::Interpretable;

pub mod error;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

pub fn run_file(filepath: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read(filepath)?;

    let _ = run(contents);

    Ok(())
}

pub fn run(source: Vec<u8>) -> Result<(), ParseError> {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    eprintln!("scanned");
    let mut parser = Parser::new(tokens);
    let expression = parser.parse()?;
    eprintln!("parsed");

    println!("{}", expression);

    println!("{:?}", expression.evaluate());

    Ok(())
}

pub fn error(ln: usize, col: usize, message: String) {
    report(ln, col, "", message.as_str())

    // TODO: had_error global variable (need to figure it out)
}

pub fn parse_error(error: &ParseError) {
    let ln = error.token.position.ln;
    let col = error.token.position.col;
    if error.token.ttype == TT::EOF {
        report(ln, col, " at end", error.message.as_str());
    } else {
        report(
            ln,
            col,
            format!(" at '{}'", error.token.lexeme).as_str(),
            error.message.as_str(),
        )
    }
}

pub fn runtime_error(error: &EvaluationError) {
    let ln = error.token.position.ln;
    let col = error.token.position.col;
    report(
        ln,
        col,
        format!(" at '{}'", error.token.lexeme).as_str(),
        error.message.as_str(),
    )
}

fn report(ln: usize, col: usize, lexeme: &str, message: &str) {
    eprintln!("[ln {ln}, col {col}] Error{lexeme}: {message}")
}
