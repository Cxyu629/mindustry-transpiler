use std::{error, fmt};

use crate::token::{Token, TokenType as TT};

#[derive(Debug)]
pub struct EvaluationError {
    pub token: Token,
    pub message: String,
}

impl EvaluationError {
    pub fn new(token: &Token, message: String) -> Self {
        Self {
            token: token.to_owned(),
            message,
        }
    }
}

impl error::Error for EvaluationError {}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ln = self.token.position.ln;
        let col = self.token.position.col;
        report(
            ln,
            col,
            format!(" at '{}'", self.token.lexeme).as_str(),
            self.message.as_str(),
        );

        write! {f, ""}
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl ParseError {
    pub fn new(token: &Token, message: String) -> Self {
        Self {
            token: token.to_owned(),
            message,
        }
    }
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ln = self.token.position.ln;
        let col = self.token.position.col;
        if self.token.ttype == TT::EOF {
            report(ln, col, " at end", self.message.as_str());
        } else {
            report(
                ln,
                col,
                format!(" at '{}'", self.token.lexeme).as_str(),
                self.message.as_str(),
            )
        }
        write!(f, "")
    }
}

fn report(ln: usize, col: usize, lexeme: &str, message: &str) {
    eprintln!("[ln {ln}, col {col}] Error{lexeme}: {message}")
}

pub fn error(ln: usize, col: usize, message: String) {
    report(ln, col, "", message.as_str())

    // TODO: had_error global variable (need to figure it out)
}