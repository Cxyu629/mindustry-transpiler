use std::{error, fmt};

use crate::token::{Token, TokenType as TT};


#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: &Token, message: String) -> Self {
        Self {
            token: token.to_owned(),
            message,
        }
    }
}

impl error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
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

fn report(ln: usize, col: usize, loc: &str, message: &str) {
    eprintln!("[ln {ln}, col {col}] Error{loc}: {message}")
}

pub fn scanning_error(ln: usize, col: usize, message: String) {
    report(ln, col, "", message.as_str())

    // TODO: had_error global variable (need to figure it out)
}

pub fn parsing_error(token: &Token, message: String) {
    if token.ttype == TT::EOF {
        report(token.position.ln, token.position.col, " at end", &message)
    } else {
        report(
            token.position.ln,
            token.position.col,
            format!(" at '{}'", token.lexeme).as_str(),
            &message,
        )
    }
}

pub fn runtime_error(token: &Token, message: String) {
    if token.ttype == TT::EOF {
        report(token.position.ln, token.position.col, " at end", &message)
    } else {
        report(
            token.position.ln,
            token.position.col,
            format!(" at '{}'", token.lexeme).as_str(),
            &message,
        )
    }
}