use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

use crate::{
    error::EvaluationError,
    expr::{Binary, Expr, Grouping, Literal, Unary},
    runtime_error,
    token::{Object as Ob, Token, TokenType as TT},
};

#[macro_use]
mod macros;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(expr: impl Interpretable) {
        match expr.evaluate() {
            Ok(value) => println!("{}", value),
            Err(error) => runtime_error(&error),
        }
    }
}

pub trait Interpretable {
    fn evaluate(&self) -> Result<Ob, EvaluationError>;
    fn report_error(&self, token: &Token, message: String) -> EvaluationError {
        let error = EvaluationError::new(token, message);
        runtime_error(&error);
        error
    }
}

impl Interpretable for Expr {
    fn evaluate(&self) -> Result<Ob, EvaluationError> {
        self.0.borrow().evaluate()
    }
}

impl Interpretable for Unary {
    fn evaluate(&self) -> Result<Ob, EvaluationError> {
        let eval_right = self.right.evaluate()?;
        match ((self.operator).ttype, &eval_right) {
            (TT::Plus, Ob::Degree(val_right)) => Ok(Ob::Degree(*val_right)),
            (TT::Plus, Ob::Number(val_right)) => Ok(Ob::Number(*val_right)),
            (TT::Minus, Ob::Degree(val_right)) => Ok(Ob::Degree(-val_right)),
            (TT::Minus, Ob::Number(val_right)) => Ok(Ob::Number(-val_right)),
            (TT::Tilde, Ob::Number(val_right)) => Ok(Ob::Number(-val_right.floor() - 1.)),
            (TT::Not, Ob::Boolean(val_right)) => Ok(Ob::Boolean(!val_right)),
            (TT::Plus | TT::Minus | TT::Tilde | TT::Not, Ob::Null) => Ok(Ob::Null),
            (TT::Plus | TT::Minus | TT::Tilde | TT::Not, _) => Err(EvaluationError::new(
                &self.operator,
                un_err_msg!(self.operator, eval_right),
            )),
            _ => panic!(
                "Unexpected {} in unary evaluation, should not occur.",
                self.operator.lexeme
            ),
        }
    }
}

impl Interpretable for Binary {
    fn evaluate(&self) -> Result<Ob, EvaluationError> {
        let eval_left = self.left.evaluate()?;
        let eval_right = self.right.evaluate()?;

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
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::Or => match (&eval_left, &eval_right) {
                (Boolean(val_left), Boolean(val_right)) => Ok(Boolean(*val_left || *val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },

            _ => panic!(
                "Unexpected {} in binary evaluation, should not occur.",
                self.operator.lexeme
            ),
        }
    }
}

impl Interpretable for Grouping {
    fn evaluate(&self) -> Result<Ob, EvaluationError> {
        self.expression.evaluate()
    }
}

impl Interpretable for Literal {
    fn evaluate(&self) -> Result<Ob, EvaluationError> {
        Ok(self.value.to_owned())
    }
}
