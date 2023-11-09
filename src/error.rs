use std::{error, fmt};

use crate::token::Token;

#[derive(Debug)]
pub struct EvaluationError {
    pub token: Token,
    pub message: String,
}

impl EvaluationError {
    pub fn new(token: &Token, message: String) -> Self {
        Self { token: token.to_owned(), message }
    }
}

impl error::Error for EvaluationError {}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {f, "Evaluation error"}
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl ParseError {
    pub fn new(token: &Token, message: String) -> Self {
        Self { token: token.to_owned(), message }
    }
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error.")
    }
}
