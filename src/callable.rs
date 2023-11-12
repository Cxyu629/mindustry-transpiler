use crate::{expr::Expr, interpreter::Interpreter, token::Object};

pub trait Callable {
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<Expr>) -> Object;
    fn arity(&self) -> usize;
}
