use std::{
    cell::RefCell,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub},
    rc::Rc,
};

use crate::{
    callable::TryCallable,
    environment::Environment,
    error::{self, RuntimeError},
    expr::*,
    function::{self, run_inner},
    stmt::*,
    token::{Object, Token, TokenType},
};

use Object as Ob;
use TokenType as TT;

#[macro_use]
mod macros;

pub struct Interpreter {
    // pub environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let me = Self {
            // environment: Rc::new(RefCell::new(Environment::new())),
            globals: Rc::new(RefCell::new(Environment::new())),
        };

        me.globals.borrow_mut().define(
            "clock".to_owned(),
            Ob::Function {
                call: function::clock,
                arity: 0,
                params: vec![],
                statement: None,
                outer_environment: None,
            },
        );
        // me.globals.borrow_mut().define(
        //     "deg".to_owned(),
        //     Ob::Function {
        //         call: function::deg,
        //         arity: 1,
        //         params: vec![Token {
        //             ttype: TT::Identifier,
        //             lexeme: "number".to_owned(),
        //             literal: None,
        //             position: todo!(),
        //         }],
        //         statement: None,
        //         outer_environment: None,
        //     },
        // );

        me
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> bool {
        let mut environment = Rc::new(RefCell::new(Environment::new_enclosed(&self.globals)));

        let mut statements_iter = statements.iter();

        while let Some(statement) = statements_iter.next() {
            match self.execute(&mut environment, statement) {
                Ok(value) => {
                    if let Ob::Return { .. } = value {
                        todo!()
                    }
                }
                Err(error) => {
                    error::runtime_error(&error.token, error.message.to_owned());
                    return false;
                }
            }
        }

        true
    }

    pub fn evaluate(
        &mut self,
        environment: &Rc<RefCell<Environment>>,
        expression: &Expr,
    ) -> Result<Ob, RuntimeError> {
        match expression {
            Expr::Unary { operator, right } => {
                let eval_right = self.evaluate(environment, right)?;
                match (operator.ttype, &eval_right) {
                    (TT::Plus, Ob::Degree(val_right)) => Ok(Ob::Degree(*val_right)),
                    (TT::Plus, Ob::Number(val_right)) => Ok(Ob::Number(*val_right)),
                    (TT::Minus, Ob::Degree(val_right)) => Ok(Ob::Degree(-val_right)),
                    (TT::Minus, Ob::Number(val_right)) => Ok(Ob::Number(-val_right)),
                    (TT::Tilde, Ob::Number(val_right)) => Ok(Ob::Number(-val_right.floor() - 1.)),
                    (TT::Not, Ob::Boolean(val_right)) => Ok(Ob::Boolean(!val_right)),
                    (TT::Plus | TT::Minus | TT::Tilde | TT::Not, Ob::Null) => Ok(Ob::Null),
                    (TT::Plus | TT::Minus | TT::Tilde | TT::Not, _) => Err(Interpreter::error(
                        &operator,
                        format!(
                            "Operand type `{}` is invalid for operator '{}'.",
                            eval_right.dtype(),
                            (operator).lexeme,
                        ),
                    )),
                    _ => panic!(
                        "Unexpected operator '{}' in unary evaluation, should not occur.",
                        operator.lexeme
                    ),
                }
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let eval_left = self.evaluate(environment, left)?;
                let eval_right = self.evaluate(environment, right)?;

                use Ob::*;
                match operator.ttype {
                    TT::Equals2 => match (&eval_left, &eval_right) {
                        (String(val_left), String(val_right)) => {
                            Ok(Boolean(val_left.eq(val_right)))
                        }
                        _ => Ok(Boolean(eval_left.coerce() == eval_right.coerce())),
                    },
                    TT::BangEquals => match (&eval_left, &eval_right) {
                        (String(val_left), String(val_right)) => Ok(Boolean(val_left != val_right)),
                        _ => Ok(Boolean(eval_left.coerce() != eval_right.coerce())),
                    },
                    TT::Equals3 => {
                        bin_match!(operator, eval_left, eval_right, eq, {
                            [Number, Number, Boolean],
                            [Degree, Degree, Boolean],
                            [String, String, Boolean],
                            [Boolean, Boolean, Boolean],
                        }, {
                            [(Null, Null), Ok(Boolean(false))],
                        })
                    }

                    TT::LAngle => bin_match!(operator, eval_left, eval_right, lt, {
                        [Number, Number, Boolean],
                        [Degree, Degree, Boolean],
                    }),
                    TT::LAngleEquals => bin_match!(operator, eval_left, eval_right, le, {
                        [Number, Number, Boolean],
                        [Degree, Degree, Boolean],
                    }),
                    TT::RAngle => bin_match!(operator, eval_left, eval_right, gt, {
                        [Number, Number, Boolean],
                        [Degree, Degree, Boolean]
                    }),
                    TT::RAngleEquals => bin_match!(operator, eval_left, eval_right, ge, {
                        [Number, Number, Boolean],
                        [Degree, Degree, Boolean]
                    }),

                    TT::Plus => bin_match!(operator, eval_left, eval_right, add, {
                        [Number, Number, Number],
                        [Degree, Degree, Degree]
                    }, {
                        [(String(val_left), String(val_right)), Ok(String(format!("{val_left}{val_right}")))],
                    }),

                    TT::Minus => bin_match!(operator, eval_left, eval_right, sub, {
                        [Number, Number, Number],
                        [Degree, Degree, Degree]
                    }),
                    TT::Slash => bin_match!(operator, eval_left, eval_right, div, {
                        [Number, Number, Number],
                        [Degree, Degree, Number]
                    }),
                    TT::Ast => bin_match!(operator, eval_left, eval_right, mul, {
                        [Number, Number, Number],
                        [Number, Degree, Degree],
                        [Degree, Number, Degree],
                    }),
                    TT::Percent => bin_match!(operator, eval_left, eval_right, rem, {
                        [Number, Number, Number],
                        [Degree, Degree, Degree],
                    }),

                    TT::Slash2 => bin_match_deref!(operator, eval_left, eval_right, div_euclid, {
                        [Number, Number, Number],
                        [Degree, Degree, Degree],
                    }),

                    TT::Ast2 => bin_match_deref!(operator, eval_left, eval_right, powf, {
                        [Number, Number, Number],
                    }),

                    TT::LAngle2 => bin_match_iuf!(operator, eval_left, eval_right, shl),
                    TT::RAngle2 => bin_match_iuf!(operator, eval_left, eval_right, shr),
                    TT::Amp => bin_match_iif!(operator, eval_left, eval_right, bitand),
                    TT::Bar => bin_match_iif!(operator, eval_left, eval_right, bitor),
                    TT::Hat => bin_match_iif!(operator, eval_left, eval_right, bitxor),

                    TT::And => match (&eval_left, &eval_right) {
                        (Boolean(val_left), Boolean(val_right)) => {
                            Ok(Boolean(*val_left && *val_right))
                        }
                        _ => generic_bin_error!(operator, eval_left, eval_right),
                    },
                    TT::Or => match (&eval_left, &eval_right) {
                        (Boolean(val_left), Boolean(val_right)) => {
                            Ok(Boolean(*val_left || *val_right))
                        }
                        _ => generic_bin_error!(operator, eval_left, eval_right),
                    },

                    _ => panic!(
                        "Unexpected {} in binary evaluation, should not occur.",
                        operator.lexeme
                    ),
                }
            }
            Expr::Grouping { expression } => self.evaluate(environment, expression),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Variable { name } => environment.borrow().get(&name),
            Expr::Assign { name, value } => {
                let new_value = self.evaluate(environment, value)?;
                environment
                    .borrow_mut()
                    .assign(name.clone(), new_value.clone())?;
                Ok(new_value)
            }
            Expr::Logical {
                operator,
                left,
                right,
            } => {
                let left_object = self.evaluate(environment, left)?;

                if operator.ttype == TT::Or {
                    match left_object {
                        Ob::Boolean(true) => Ok(left_object),
                        Ob::Boolean(false) => Ok(self.evaluate(environment, right)?),
                        _ => Err(Interpreter::error(
                            &operator,
                            "Expected `Boolean` operands.".to_owned(),
                        )),
                    }
                } else {
                    match left_object {
                        Ob::Boolean(true) => Ok(self.evaluate(environment, right)?),
                        Ob::Boolean(false) => Ok(left_object),
                        _ => Err(Interpreter::error(
                            &operator,
                            "Expected `Boolean` operands.".to_owned(),
                        )),
                    }
                }
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(environment, &callee)?;

                let mut argument_values = Vec::new();
                for argument in arguments {
                    argument_values.push(self.evaluate(environment, argument)?);
                }

                if let Some(function) = callee.try_callable() {
                    if arguments.len() != function.arity {
                        return Err(Interpreter::error(
                            paren,
                            format!(
                                "Expected {} arguments, but got {} instead.",
                                function.arity,
                                arguments.len()
                            ),
                        ));
                    }

                    let function_environment =
                        function.outer_environment.unwrap_or(environment.to_owned());

                    (function.call)(
                        self,
                        function_environment,
                        &function.params,
                        argument_values,
                        &function
                            .statement
                            .unwrap_or(Box::new(Stmt::new_block(vec![]))),
                    )
                } else {
                    Err(RuntimeError {
                        token: paren.clone(),
                        message: "Can only call functions and classes.".to_owned(),
                    })
                }
            }
        }
    }

    pub fn execute(
        &mut self,
        environment: &Rc<RefCell<Environment>>,
        statement: &Stmt,
    ) -> Result<Ob, RuntimeError> {
        match statement {
            Stmt::Expression { expression } => self.evaluate(environment, &expression),
            Stmt::Print { expression } => {
                let value = self.evaluate(environment, &expression)?;
                println!("{}", value);
                Ok(Ob::Null)
            }
            Stmt::Var { name, initialiser } => {
                let value = match &initialiser {
                    Some(expr) => self.evaluate(environment, &expr)?,
                    _ => Ob::Null,
                };

                environment
                    .borrow_mut()
                    .define(name.lexeme.to_owned(), value.clone());

                Ok(value)
            }
            Stmt::Block { statements } => self.execute_block(environment, statements.to_vec()),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if let Ok(Ob::Boolean(result)) = self.evaluate(environment, &condition) {
                    let mut value = Ob::Null;
                    if result {
                        value = self.execute(environment, then_branch)?;
                    } else if let Some(else_branch) = else_branch {
                        value = self.execute(environment, else_branch)?;
                    }

                    Ok(value)
                } else {
                    todo!()
                    // Err(Interpreter::error(, "Expected boolean condition."))
                }
            }
            Stmt::While { condition, body } => {
                let mut value = Ob::Null;
                while let Ob::Boolean(true) = self.evaluate(environment, &condition)? {
                    value = self.execute(environment, body)?;
                    if let Ob::Return { .. } = value {
                        return Ok(value);
                    }
                }

                Ok(value)
            }
            Stmt::Function { name, params, body } => {
                let function = Ob::Function {
                    call: run_inner,
                    arity: params.len(),
                    params: params.to_owned(),
                    statement: Some(body.to_owned()),
                    outer_environment: Some(environment.to_owned()),
                };
                environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), function.to_owned());
                Ok(function)
            }
            Stmt::Return { value, .. } => match value {
                Some(value) => Ok(Ob::Return {
                    value: Box::new(self.evaluate(environment, value)?),
                }),
                None => Ok(Ob::Return {
                    value: Box::new(Ob::Null),
                }),
            },
            Stmt::Blank {} => Ok(Ob::Null),
        }
    }

    pub fn execute_block(
        &mut self,
        environment: &Rc<RefCell<Environment>>,
        statements: Vec<Stmt>,
        // env: Environment,
    ) -> Result<Ob, RuntimeError> {
        let mut value: Ob = Ob::Null;
        {
            let new_env = Rc::new(RefCell::new(Environment::new_enclosed(&environment)));

            let mut statements_iter = statements.iter();

            while let Some(statement) = statements_iter.next() {
                value = self.execute(&new_env, &statement)?;
                if let Ob::Return { .. } = value {
                    return Ok(value);
                }
            }
        }

        Ok(value)
    }

    pub fn error(token: &Token, message: String) -> RuntimeError {
        // error::runtime_error(token, message.to_owned());
        RuntimeError::new(token, message)
    }
}
