use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

use crate::{
    environment::Environment,
    error::{self, RuntimeError},
    expr::*,
    stmt::*,
    token::{Object as Ob, Token, TokenType as TT},
};

#[macro_use]
mod macros;

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<impl Interpretable>) -> bool {
        let mut statements_iter = statements.iter();

        while let Some(statement) = statements_iter.next() {
            match (*statement).evaluate(self) {
                Ok(value) => {}
                Err(error) => {
                    error::runtime_error(&error.token, error.message.to_owned());
                    return false;
                }
            }
        }

        true
    }

    pub fn error(token: &Token, message: String) -> RuntimeError {
        // error::runtime_error(token, message.to_owned());
        RuntimeError::new(token, message)
    }
}

pub trait Interpretable {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError>;
}

impl Interpretable for Expr {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        self.0.evaluate(interpreter)
    }
}

impl Interpretable for UnaryExpr {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        let eval_right = self.right.evaluate(interpreter)?;
        match ((self.operator).ttype, &eval_right) {
            (TT::Plus, Ob::Degree(val_right)) => Ok(Ob::Degree(*val_right)),
            (TT::Plus, Ob::Number(val_right)) => Ok(Ob::Number(*val_right)),
            (TT::Minus, Ob::Degree(val_right)) => Ok(Ob::Degree(-val_right)),
            (TT::Minus, Ob::Number(val_right)) => Ok(Ob::Number(-val_right)),
            (TT::Tilde, Ob::Number(val_right)) => Ok(Ob::Number(-val_right.floor() - 1.)),
            (TT::Not, Ob::Boolean(val_right)) => Ok(Ob::Boolean(!val_right)),
            (TT::Plus | TT::Minus | TT::Tilde | TT::Not, Ob::Null) => Ok(Ob::Null),
            (TT::Plus | TT::Minus | TT::Tilde | TT::Not, _) => Err(Interpreter::error(
                &self.operator,
                format!(
                    "Operand type `{}` is invalid for operator '{}'.",
                    eval_right.dtype(),
                    (self.operator).lexeme,
                ),
            )),
            _ => panic!(
                "Unexpected operator '{}' in unary evaluation, should not occur.",
                self.operator.lexeme
            ),
        }
    }
}

impl Interpretable for BinaryExpr {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        let eval_left = self.left.evaluate(interpreter)?;
        let eval_right = self.right.evaluate(interpreter)?;

        use Ob::*;
        match self.operator.ttype {
            TT::Equals2 => match (&eval_left, &eval_right) {
                (String(val_left), String(val_right)) => Ok(Boolean(val_left.eq(val_right))),
                _ => Ok(Boolean(eval_left.coerce() == eval_right.coerce())),
            },
            TT::BangEquals => match (&eval_left, &eval_right) {
                (String(val_left), String(val_right)) => Ok(Boolean(val_left != val_right)),
                _ => Ok(Boolean(eval_left.coerce() != eval_right.coerce())),
            },
            TT::Equals3 => {
                bin_match!(self, eval_left, eval_right, eq, {
                    [Number, Number, Boolean],
                    [Degree, Degree, Boolean],
                    [String, String, Boolean],
                    [Boolean, Boolean, Boolean],
                }, {
                    [(Null, Null), Ok(Boolean(false))],
                })
            }

            TT::LAngle => bin_match!(self, eval_left, eval_right, lt, {
                [Number, Number, Boolean],
                [Degree, Degree, Boolean],
            }),
            TT::LAngleEquals => bin_match!(self, eval_left, eval_right, le, {
                [Number, Number, Boolean],
                [Degree, Degree, Boolean],
            }),
            TT::RAngle => bin_match!(self, eval_left, eval_right, gt, {
                [Number, Number, Boolean],
                [Degree, Degree, Boolean]
            }),
            TT::RAngleEquals => bin_match!(self, eval_left, eval_right, ge, {
                [Number, Number, Boolean],
                [Degree, Degree, Boolean]
            }),

            TT::Plus => bin_match!(self, eval_left, eval_right, add, {
                [Number, Number, Number],
                [Degree, Degree, Degree]
            }, {
                [(String(val_left), String(val_right)), Ok(String(format!("{val_left}{val_right}")))],
            }),

            TT::Minus => bin_match!(self, eval_left, eval_right, sub, {
                [Number, Number, Number],
                [Degree, Degree, Degree]
            }),
            TT::Slash => bin_match!(self, eval_left, eval_right, div, {
                [Number, Number, Number],
                [Degree, Degree, Number]
            }),
            TT::Ast => bin_match!(self, eval_left, eval_right, mul, {
                [Number, Number, Number],
                [Number, Degree, Degree],
                [Degree, Number, Degree],
            }),
            TT::Percent => bin_match!(self, eval_left, eval_right, rem, {
                [Number, Number, Number],
                [Degree, Degree, Degree],
            }),

            TT::Slash2 => bin_match_deref!(self, eval_left, eval_right, div_euclid, {
                [Number, Number, Number],
                [Degree, Degree, Degree],
            }),

            TT::Ast2 => bin_match_deref!(self, eval_left, eval_right, powf, {
                [Number, Number, Number],
            }),

            TT::LAngle2 => bin_match_iuf!(self, eval_left, eval_right, shl),
            TT::RAngle2 => bin_match_iuf!(self, eval_left, eval_right, shr),
            TT::Amp => bin_match_iif!(self, eval_left, eval_right, bitand),
            TT::Bar => bin_match_iif!(self, eval_left, eval_right, bitor),
            TT::Hat => bin_match_iif!(self, eval_left, eval_right, bitxor),

            TT::And => match (&eval_left, &eval_right) {
                (Boolean(val_left), Boolean(val_right)) => Ok(Boolean(*val_left && *val_right)),
                _ => generic_error!(self, eval_left, eval_right),
            },
            TT::Or => match (&eval_left, &eval_right) {
                (Boolean(val_left), Boolean(val_right)) => Ok(Boolean(*val_left || *val_right)),
                _ => generic_error!(self, eval_left, eval_right),
            },

            _ => panic!(
                "Unexpected {} in binary evaluation, should not occur.",
                self.operator.lexeme
            ),
        }
    }
}

impl Interpretable for GroupingExpr {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        self.expression.evaluate(interpreter)
    }
}

impl Interpretable for LiteralExpr {
    fn evaluate(&self, _interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        Ok(self.value.to_owned())
    }
}

impl Interpretable for Variable {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        interpreter.environment.get(&self.name)
    }
}

impl Interpretable for AssignExpr {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        todo!()
    }
}

// ========== Statements ==========

impl Interpretable for Stmt {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        self.0.evaluate(interpreter)
    }
}

impl Interpretable for ExpressionStmt {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        self.expression.evaluate(interpreter)
    }
}

impl Interpretable for PrintStmt {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        let value = self.expression.evaluate(interpreter)?;
        println!("{}", value);
        Ok(Ob::Null)
    }
}

impl Interpretable for VarStmt {
    fn evaluate(&self, interpreter: &mut Interpreter) -> Result<Ob, RuntimeError> {
        let value = if let Some(expr) = &self.initialiser {
            expr.evaluate(interpreter)?
        } else {
            Ob::Null
        };

        interpreter
            .environment
            .define(self.name.lexeme.to_owned(), value);

        Ok(Ob::Null)
    }
}
