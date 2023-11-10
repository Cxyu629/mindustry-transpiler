use std::collections::HashMap;
use std::default;

use crate::error::error;
use crate::token::Object;
use crate::token::Position;
use crate::token::Token;
use crate::token::TokenType as TT;

#[derive(Clone)]
pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    startln: usize,
    current: usize,
    ln: usize,
}

impl default::Default for Scanner {
    fn default() -> Self {
        Self {
            source: vec![],
            tokens: vec![],
            start: 0,
            startln: 1,
            current: 0,
            ln: 1,
        }
    }
}

impl Scanner {
    pub fn from_string(source: String) -> Self {
        Self {
            source: source.into_bytes(),
            ..Default::default()
        }
    }

    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !(self.is_at_end()) {
            self.start = self.current;
            self.startln = self.ln;

            // scan_token
            {
                let c: char = self.advance();
                let c_peek: char = self.peek();
                match (c, c_peek) {
                    ('(', _) => self.add_token(TT::LParen),
                    (')', _) => self.add_token(TT::RParen),
                    ('{', _) => self.add_token(TT::LBrace),
                    ('}', _) => self.add_token(TT::RBrace),
                    (',', _) => self.add_token(TT::Comma),
                    ('.', _) => self.add_token(TT::Dot),
                    ('+', _) => self.add_token(TT::Plus),
                    ('-', _) => self.add_token(TT::Minus),
                    (';', _) => self.add_token(TT::Semicolon),
                    ('^', _) => self.add_token(TT::Hat),
                    ('~', _) => self.add_token(TT::Tilde),
                    ('|', _) => self.add_token(TT::Bar),
                    ('&', _) => self.add_token(TT::Amp),
                    ('%', _) => self.add_token(TT::Percent),
                    ('!', '=') => {
                        self.advance();
                        self.add_token(TT::BangEquals)
                    }
                    ('=', _) => {
                        let res = self.cond_advance('=');
                        let mut res2 = false;
                        if res {
                            res2 = self.cond_advance('=');
                        }
                        let ty = if res2 {
                            TT::Equals3
                        } else if res {
                            TT::Equals2
                        } else {
                            TT::Equals
                        };
                        self.add_token(ty)
                    }
                    ('<', _) => {
                        let res = self.cond_advance('=');
                        let mut res2 = false;
                        if !res {
                            res2 = self.cond_advance('<');
                        }
                        let ty = if !(res || res2) {
                            TT::LAngle
                        } else if res {
                            TT::LAngleEquals
                        } else {
                            TT::LAngle2
                        };
                        self.add_token(ty)
                    }
                    ('>', _) => {
                        let res = self.cond_advance('=');
                        let mut res2 = false;
                        if !res {
                            res2 = self.cond_advance('>');
                        }
                        let ty = if !(res || res2) {
                            TT::RAngle
                        } else if res {
                            TT::RAngleEquals
                        } else {
                            TT::RAngle2
                        };
                        self.add_token(ty)
                    }
                    ('*', _) => {
                        let res = self.cond_advance('*');
                        let ty = if res { TT::Ast2 } else { TT::Ast };
                        self.add_token(ty)
                    }

                    ('/', _) => {
                        let res = self.cond_advance('/');
                        let ty = if res { TT::Slash2 } else { TT::Slash };
                        self.add_token(ty)
                    }
                    ('#', _) => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }

                    (' ', _) => {}
                    ('\r', _) => {}
                    ('\t', _) => {}
                    ('\n', _) => self.ln += 1,

                    ('"', _) => {
                        self.string();
                    }

                    _ => {
                        if c.is_ascii_digit() {
                            self.number();
                        } else if c.is_ascii_alphabetic() || c == '_' {
                            self.identifier();
                        } else {
                            error(self.ln, self.start + 1, "Unexpected character.".to_owned())
                        }
                    }
                }
            }
        }

        self.tokens
            .push(Token::eof(Position::new(self.ln, self.current + 1)));

        self.tokens.clone()
    }

    fn identifier(&mut self) {
        let mut keywords = HashMap::new();
        keywords.insert("and", TT::And);
        keywords.insert("or", TT::Or);
        keywords.insert("not", TT::Not);
        keywords.insert("while", TT::While);
        keywords.insert("for", TT::For);
        keywords.insert("if", TT::If);
        keywords.insert("else", TT::Else);
        keywords.insert("null", TT::Null);
        keywords.insert("true", TT::True);
        keywords.insert("false", TT::False);
        keywords.insert("fun", TT::Fun);
        keywords.insert("return", TT::Return);
        keywords.insert("class", TT::Class);
        keywords.insert("this", TT::This);
        keywords.insert("super", TT::Super);
        keywords.insert("var", TT::Var);

        keywords.insert("num", TT::Num);
        keywords.insert("deg", TT::Deg);

        // keywords.insert("max", TT::Max);
        // keywords.insert("min", TT::Min);
        // keywords.insert("angle", TT::And);
        // keywords.insert("angleDiff", TT::And);
        // keywords.insert("len", TT::And);
        // keywords.insert("noise", TT::And);
        // keywords.insert("abs", TT::And);
        // keywords.insert("log", TT::And);
        // keywords.insert("log10", TT::And);
        // keywords.insert("floor", TT::And);
        // keywords.insert("ceil", TT::And);
        // keywords.insert("sqrt", TT::And);
        // keywords.insert("rand", TT::And);
        // keywords.insert("sin", TT::And);
        // keywords.insert("cos", TT::And);
        // keywords.insert("tan", TT::And);
        // keywords.insert("asin", TT::And);
        // keywords.insert("acos", TT::And);
        // keywords.insert("atan", TT::And);

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme: Vec<u8> = self.source[self.start..self.current].into();
        let ty = if let Some(ty) = keywords.get(&(String::from_utf8(lexeme).unwrap().as_str())) {
            *ty
        } else {
            TT::Identifier
        };

        self.add_token(ty)
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_n(1).is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let ttype: TT;
        let lexeme: String;
        let literal: Option<Object>;
        let position: Position;

        position = Position::new(self.startln, self.start + 1);

        if self.peek() == 'd'
            && self.peek_next() == 'e'
            && self.peek_n(2) == 'g'
            && !(self.peek_n(3).is_ascii_alphanumeric() || self.peek_n(3) == '_')
        {
            self.advance();
            self.advance();
            self.advance();

            lexeme = String::from_utf8(self.source[self.start..self.current].into()).unwrap();
            literal = if let Ok(value) = lexeme[0..(lexeme.len() - 3)].parse::<f32>() {
                Some(Object::Degree(value))
            } else {
                None
            };
            ttype = TT::Degree;
        } else {
            lexeme = String::from_utf8(self.source[self.start..self.current].into()).unwrap();
            literal = if let Ok(value) = lexeme.parse::<f32>() {
                Some(Object::Number(value))
            } else {
                None
            };
            ttype = TT::Number;
        }

        self.tokens.push(Token {
            ttype,
            lexeme,
            literal,
            position,
        });
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.ln += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            error(self.ln, self.current, "Unterminated string.".to_owned());
        } else {
            self.advance();
            let lexeme =
                String::from_utf8(self.source[(self.start + 1)..(self.current - 1)].into())
                    .unwrap();
            let position = Position::new(self.startln, self.start + 1);

            self.tokens.push(Token {
                ttype: TT::String,
                lexeme,
                literal: None,
                position,
            })
        }
    }

    fn add_token(&mut self, ty: TT) {
        let text = String::from_utf8(self.source[self.start..self.current].into()).unwrap();
        let position = Position::new(self.startln, self.start + 1);
        self.tokens.push(Token {
            ttype: ty,
            lexeme: text,
            literal: None,
            position,
        })
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        (*self.source.get(self.current - 1).unwrap_or(&0u8)).into()
    }

    fn cond_advance(&mut self, expected: char) -> bool {
        if let Some(c) = self.source.get(self.current) {
            if *c as char != expected {
                return false;
            }

            self.current += 1;
        } else {
            return false;
        }

        return true;
    }

    fn peek(&self) -> char {
        *self.source.get(self.current).unwrap_or(&0u8) as char
    }

    fn peek_next(&self) -> char {
        *self.source.get(self.current + 1).unwrap_or(&0u8) as char
    }

    fn peek_n(&self, n: usize) -> char {
        *self.source.get(self.current + n).unwrap_or(&0u8) as char
    }
}
