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
            TT::Equals3 => match (&eval_left, &eval_right) {
                // bin_match_left!(Number, Number) => bin_match_right!(Boolean, eq),
                (Number(val_left), Number(val_right)) => Ok(Boolean(val_left == val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Boolean(val_left == val_right)),
                (String(val_left), String(val_right)) => Ok(Boolean(val_left == val_right)),
                (Boolean(val_left), Boolean(val_right)) => Ok(Boolean(val_left == val_right)),
                (Null, Null) => Ok(Boolean(true)),
                _ => Ok(Boolean(false)),
            },

            TT::LAngle => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Boolean(val_left < val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Boolean(val_left < val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::LAngleEquals => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Boolean(val_left <= val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Boolean(val_left <= val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::RAngle => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Boolean(val_left > val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Boolean(val_left > val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::RAngleEquals => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Boolean(val_left >= val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Boolean(val_left >= val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },

            TT::Plus => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Number(val_left + val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Degree(val_left + val_right)),
                (String(val_left), String(val_right)) => {
                    Ok(String(format!("{val_left}{val_right}")))
                }
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },

            TT::Minus => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Number(val_left - val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Degree(val_left - val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::Slash => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Number(val_left / val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Number(val_left / val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::Ast => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Number(val_left * val_right)),
                (Number(val_left), Degree(val_right)) => Ok(Degree(val_left * val_right)),
                (Degree(val_left), Number(val_right)) => Ok(Degree(val_left * val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::Percent => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Number(val_left % val_right)),
                (Degree(val_left), Degree(val_right)) => Ok(Degree(val_left % val_right)),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },

            TT::Slash2 => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => {
                    Ok(Number(val_left.div_euclid(*val_right)))
                }
                (Degree(val_left), Degree(val_right)) => {
                    Ok(Degree(val_left.div_euclid(*val_right)))
                }
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },

            TT::Ast2 => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => Ok(Number(val_left.powf(*val_right))),
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },

            TT::LAngle2 => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => {
                    Ok(Number(((*val_left as i32) << (*val_right as u32)) as f32))
                }
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::RAngle2 => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => {
                    Ok(Number(((*val_left as i32) >> (*val_right as u32)) as f32))
                }
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::Amp => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => {
                    Ok(Number(((*val_left as i32) & (*val_right as i32)) as f32))
                }
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::Bar => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => {
                    Ok(Number(((*val_left as i32) | (*val_right as i32)) as f32))
                }
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },
            TT::Hat => match (&eval_left, &eval_right) {
                (Number(val_left), Number(val_right)) => {
                    Ok(Number(((*val_left as i32) ^ (*val_right as i32)) as f32))
                }
                _ => Err(self.report_error(
                    &self.operator,
                    bin_err_msg!(self.operator, eval_left, eval_right),
                )),
            },

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
