use crate::{
    error::{self, ParseError},
    expr::*,
    interpreter::Interpreter,
    stmt::*,
    token::{Object, Token, TokenType as TT},
};

#[macro_use]
mod macros {
    macro_rules! binary {
        (($fun: tt, $prev_fun: tt, [h $x:expr$(, $y:expr)*])) => {
            fn $fun(&mut self) -> Result<Expr, ParseError> {
                let mut left = self.$prev_fun()?.clone();

                let mut valid = self.cond_advance(vec![$x$(, $y)*]).cloned();

                while let Some(operator) = valid {
                    let right = self.$prev_fun()?.clone();
                    left = Expr::new_binary(
                        operator.to_owned(),
                        Box::new(left),
                        Box::new(right),
                    );
                    valid = self.cond_advance(vec![$x$(, $y)*]).cloned();
                }
                Ok(left)
            }
        };
    }
}

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

    pub fn parse(&mut self) -> (Vec<Stmt>, Result<(), ()>) {
        let mut statements = Vec::new();
        let mut had_error = false;

        while !self.is_at_end() {
            match self.statement() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(_error) => {
                    had_error = true;
                    self.synchronise();
                    // TODO: yeah...
                }
            }
        }

        (statements, if had_error { Err(()) } else { Ok(()) })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        // TODO: other kinds of statements

        let next = self.peek();

        if next.ttype == TT::Semicolon {
            self.blank_statement()
        } else if next.ttype == TT::LBrace {
            self.block_statement()
        } else if next.ttype == TT::Return {
            self.return_statement()
        } else if next.ttype == TT::Fun {
            self.function_statement()
        } else if next.ttype == TT::For {
            self.for_statement()
        } else if next.ttype == TT::Do {
            self.do_while_statement()
        } else if next.ttype == TT::While {
            self.while_statement()
        } else if next.ttype == TT::If {
            self.if_statement()
        } else if next.ttype == TT::Var {
            self.var_statement()
        } else if next.ttype == TT::Print {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn blank_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        Ok(Stmt::Blank {})
    }

    fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let mut statements = Vec::new();

        while !self.check(TT::RBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(TT::RBrace, "Expected '}' after block.")?;

        Ok(Stmt::new_block(statements))
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.advance().clone();
        let mut value = None;
        if !self.check(TT::Semicolon) {
            value = Some(self.expression()?);
        }

        self.consume(TT::Semicolon, "Expected ';' after return value.")?;
        Ok(Stmt::new_return(keyword, value))
    }

    fn function_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let name = self
            .consume(TT::Identifier, "Expected function name.")?
            .clone();
        self.consume(TT::LParen, "Expected '(' after function name")?;
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(TT::RParen) {
            parameters.push(
                self.consume(TT::Identifier, "Expected parameter name.")?
                    .clone(),
            );
            while self.cond_advance(vec![TT::Comma]).is_some() {
                parameters.push(
                    self.consume(TT::Identifier, "Expected parameter name.")?
                        .clone(),
                );
                if parameters.len() > 255 {
                    Interpreter::error(
                        &self.peek().clone(),
                        "Can't have more than 255 parameters.".to_owned(),
                    );
                }
            }
        }

        self.consume(TT::RParen, "Expected ')' after parameters.")?;

        let body = self.block_statement()?;

        Ok(Stmt::new_function(name.clone(), parameters, Box::new(body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        self.consume(TT::LParen, "Expected '(' after 'for'.")?;

        let pre_initialiser: Option<Stmt>;
        if self.peek().ttype == TT::Semicolon {
            pre_initialiser = None;
        } else if self.peek().ttype == TT::Var {
            pre_initialiser = Some(self.var_statement()?);
        } else {
            pre_initialiser = Some(self.expression_statement()?);
        }

        let mut pre_condition: Option<Expr> = None;
        if !self.check(TT::Semicolon) {
            pre_condition = Some(self.expression()?);
        }
        self.consume(TT::Semicolon, "Expected ';' after loop condition.")?;

        let mut pre_increment: Option<Expr> = None;
        if !self.check(TT::RParen) {
            pre_increment = Some(self.expression()?);
        }
        self.consume(TT::RParen, "Expected ')' after for clauses.")?;

        let mut body = self.block_statement()?;

        if let Some(increment) = pre_increment {
            body = Stmt::new_block(vec![body, Stmt::new_expression(increment)])
        }

        let condition: Expr = pre_condition.unwrap_or(Expr::new_literal(Object::Boolean(true)));
        body = Stmt::new_while(condition, Box::new(body));

        if let Some(initialiser) = pre_initialiser {
            body = Stmt::new_block(vec![initialiser, body]);
        }

        return Ok(body);
    }

    fn do_while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let body = Box::new(self.block_statement()?);
        self.consume(TT::While, "Expected 'while'.")?;
        let condition = self.expression()?;

        self.consume(TT::Semicolon, "Expected ';' after loop condition.")?;

        let body = Stmt::new_block(vec![*body.clone(), Stmt::new_while(condition, body)]);

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.expression()?;
        let body = Box::new(self.block_statement()?);

        // self.consume(TT::Semicolon, "Expected ';' after while loop.")?;

        Ok(Stmt::new_while(condition, body))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.expression()?;

        let then_branch = Box::new(self.block_statement()?);
        let else_branch;

        if self.cond_advance(vec![TT::Else]).is_some() {
            else_branch = Some(Box::new(self.block_statement()?));
        } else {
            else_branch = None
        }

        // self.consume(TT::Semicolon, "Expected ';' after if statement.")?;

        Ok(Stmt::new_if(condition, then_branch, else_branch))
    }

    fn var_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let name = self
            .consume(TT::Identifier, "Expected variable name.")?
            .to_owned();
        let mut initialiser = None;
        if self.cond_advance(vec![TT::Equals]).is_some() {
            initialiser = Some(self.expression()?);
        }

        self.consume(TT::Semicolon, "Expected ';' after variable declaration.")?;
        Ok(Stmt::new_var(name, initialiser))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let value = self.expression()?;
        self.consume(TT::Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::new_print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TT::Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::new_expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logic_or()?;

        if let Some(equals) = self.cond_advance(vec![TT::Equals]).cloned() {
            let value = self.assignment()?;

            match &expr {
                Expr::Variable { name } => {
                    return Ok(Expr::new_assign(name.to_owned(), Box::new(value)))
                }
                _ => {
                    Parser::error(&equals, "Invalid assigment target.".to_owned());
                }
            }
        }

        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.logic_and()?.clone();
        let mut valid = self.cond_advance(vec![(TT::Or)]).cloned();
        while let Some(operator) = valid {
            let right = self.logic_and()?.clone();
            left = Expr::new_logical(operator.to_owned(), Box::new(left), Box::new(right));
            valid = self.cond_advance(vec![(TT::Or)]).cloned();
        }
        Ok(left)
    }
    fn logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.equality()?.clone();
        let mut valid = self.cond_advance(vec![(TT::And)]).cloned();
        while let Some(operator) = valid {
            let right = self.equality()?.clone();
            left = Expr::new_logical(operator.to_owned(), Box::new(left), Box::new(right));
            valid = self.cond_advance(vec![(TT::And)]).cloned();
        }
        Ok(left)
    }

    binary!((equality, relational, [h TT::Equals2, TT::BangEquals, TT::Equals3]));
    binary!((relational, bit_or, [h TT::LAngle, TT::LAngleEquals, TT::RAngle, TT::RAngleEquals]));
    binary!((bit_or, bit_xor, [h TT::Bar]));
    binary!((bit_xor, bit_and, [h TT::Hat]));
    binary!((bit_and, bit_shift, [h TT::Amp]));
    binary!((bit_shift, term, [h TT::LAngle2, TT::RAngle2]));
    binary!((term, factor, [h TT::Plus, TT::Minus]));
    binary!((factor, unary, [h TT::Ast, TT::Slash, TT::Percent, TT::Slash2]));

    fn unary(&mut self) -> Result<Expr, ParseError> {
        match self
            .cond_advance(vec![TT::Plus, TT::Minus, TT::Tilde, TT::Not])
            .cloned()
        {
            Some(operator) => {
                let right = self.unary()?;
                Ok(Expr::new_unary(operator.to_owned(), Box::new(right)))
            }
            None => self.exponential(),
        }
    }

    fn exponential(&mut self) -> Result<Expr, ParseError> {
        let left = self.call()?;

        match self.cond_advance(vec![TT::Ast2]).cloned() {
            Some(operator) => {
                let save = self.current;
                match self.exponential() {
                    Ok(right) => Ok(Expr::new_binary(
                        operator.to_owned(),
                        Box::new(left),
                        Box::new(right),
                    )),
                    Err(_) => {
                        self.current = save;
                        let right = self.unary()?;
                        Ok(Expr::new_binary(
                            operator.to_owned(),
                            Box::new(left),
                            Box::new(right),
                        ))
                    }
                }
            }
            None => Ok(left),
        }
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.cond_advance(vec![TT::LParen]).is_some() {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();

        if !self.check(TT::RParen) {
            arguments.push(self.expression()?);
            while self.cond_advance(vec![TT::Comma]).is_some() {
                arguments.push(self.expression()?);
                if arguments.len() > 255 {
                    Parser::error(
                        self.peek(),
                        "Can't have more than 255 arguments.".to_owned(),
                    );
                }
            }
        }

        let paren = self
            .consume(TT::RParen, "Expected ')' after arguments.")?
            .clone();

        Ok(Expr::new_call(Box::new(callee), paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().ttype {
            TT::False => {
                self.advance();
                Ok(Expr::new_literal(Object::Boolean(false)))
            }
            TT::True => {
                self.advance();
                Ok(Expr::new_literal(Object::Boolean(true)))
            }
            TT::Null => {
                self.advance();
                Ok(Expr::new_literal(Object::Null))
            }

            TT::Number | TT::String => {
                let token = self.advance();
                Ok(Expr::new_literal(token.literal.clone().unwrap()))
            }

            TT::Identifier => {
                let name = self.advance();
                Ok(Expr::new_variable(name.to_owned()))
            }

            TT::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TT::RParen, "Expected ')' after expression.")?;
                Ok(Expr::new_grouping(Box::new(expr)))
            }
            _ => Err(Self::error(self.peek(), "Expected expression.".to_owned())),
        }
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().ttype {
                TT::Semicolon => return,
                TT::RBrace => return,
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
            Err(Parser::error(self.peek(), message.to_owned()))
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn cond_advance(&mut self, ttypes: Vec<TT>) -> Option<&Token> {
        for ttype in ttypes.into_iter() {
            if self.check(ttype) {
                return Some(self.advance());
            }
        }

        None
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

    #[allow(unused)]
    fn peek_n(&self, n: usize) -> &Token {
        self.tokens.get(self.current + n).unwrap()
    }

    fn error(token: &Token, message: String) -> ParseError {
        error::parsing_error(token, message.to_owned());
        ParseError {
            token: token.clone(),
            message,
        }
    }
}
