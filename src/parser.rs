use crate::{
    error::ParseError,
    expr::*,
    stmt::{ExpressionStmt, Stmt, IntoStmt},
    token::{Object, Token, TokenType as TT},
};

#[macro_use]
mod macros;

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> (Vec<Stmt>, bool) {
        let mut statements = Vec::new();
        let mut has_error = false;

        while !self.is_at_end() {
            match self.statement() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(error) => {
                    eprintln!("{}", error);
                    has_error = true;
                }
            }
        }

        (statements, !has_error)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        // TODO: other kinds of statements

        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TT::Semicolon, "Expected ';' after expression.");
        Ok(ExpressionStmt::new(expr).into_stmt())
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.logic_or()
    }

    binary!((logic_or, logic_and, [h TT::Or]));
    binary!((logic_and, bit_or, [h TT::And]));
    binary!((bit_or, bit_xor, [h TT::Bar]));
    binary!((bit_xor, bit_and, [h TT::Hat]));
    binary!((bit_and, equality, [h TT::Amp]));
    binary!((equality, relational, [h TT::Equals2, TT::BangEquals, TT::Equals3]));
    binary!((relational, bit_shift, [h TT::LAngle, TT::LAngleEquals, TT::RAngle, TT::RAngleEquals]));
    binary!((bit_shift, term, [h TT::LAngle2, TT::RAngle2]));
    binary!((term, factor, [h TT::Plus, TT::Minus]));
    binary!((factor, unary, [h TT::Ast, TT::Slash, TT::Percent, TT::Slash2]));

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.cond_advance(vec![TT::Plus, TT::Minus, TT::Tilde, TT::Not]) {
            let operator = self.previous().to_owned();
            let right = self.unary()?;
            Ok(UnaryExpr::new(operator, right).into_expr())
        } else {
            self.exponential()
        }
    }

    fn exponential(&mut self) -> Result<Expr, ParseError> {
        let left = self.primary()?;

        if self.cond_advance(vec![TT::Ast2]) {
            let operator = self.previous().to_owned();

            let save = self.current;
            if let Ok(right) = self.exponential() {
                Ok(BinaryExpr::new(operator, left, right).into_expr())
            } else {
                self.current = save;
                let right = self.unary()?;
                Ok(BinaryExpr::new(operator, left, right).into_expr())
            }
        } else {
            Ok(left)
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().ttype {
            TT::False => {
                self.advance();
                Ok(LiteralExpr::new(Object::Boolean(false)).into_expr())
            }
            TT::True => {
                self.advance();
                Ok(LiteralExpr::new(Object::Boolean(true)).into_expr())
            }
            TT::Null => {
                self.advance();
                Ok(LiteralExpr::new(Object::Null).into_expr())
            }

            TT::Number | TT::String => {
                let token = self.advance();
                return Ok(LiteralExpr::new(token.literal.clone().unwrap()).into_expr());
            }

            TT::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TT::RParen, "Expected ')' after expression.")?;
                Ok(GroupingExpr::new(expr).into_expr())
            }
            _ => Err(ParseError::new(
                self.peek(),
                "Expected expression.".to_owned(),
            )),
        }
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().ttype {
                TT::Semicolon => return,
                _ => {}
            }

            match self.peek().ttype {
                TT::Class | TT::Fun | TT::Var | TT::For | TT::While | TT::If | TT::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn consume(&mut self, ttype: TT, message: &str) -> Result<&Token, ParseError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(self.peek(), message.to_owned()))
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn cond_advance(&mut self, ttypes: Vec<TT>) -> bool {
        for ttype in ttypes.into_iter() {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn check(&self, ttype: TT) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ttype == ttype
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TT::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn peek_n(&self, n: usize) -> &Token {
        self.tokens.get(self.current + n).unwrap()
    }
}
