use std::{cell::RefCell, rc::Rc};

use anyhow::Result;

use crate::{
    environment::Environment,
    error::RuntimeError,
    interpreter::Interpreter,
    stmt::Stmt,
    token::{Object, Token},
};

// pub trait Callable {
//     fn call(
//         &mut self,
//         interpreter: &mut Interpreter,
//         // environment: &Rc<RefCell<Environment>>,
//         arguments: Vec<Object>,
//     ) -> Result<Object, RuntimeError>;
//     fn arity(&self) -> usize;
// }

pub trait TryCallable {
    fn try_callable(self) -> Option<CCallable>;
}

pub struct CCallable {
    pub params: Vec<Token>,
    pub statement: Option<Box<Stmt>>,
    pub call: fn(
        &mut Interpreter,
        Rc<RefCell<Environment>>,
        &Vec<Token>,
        Vec<Object>,
        &Stmt,
    ) -> Result<Object, RuntimeError>,
    pub arity: usize,
    pub outer_environment: Option<Rc<RefCell<Environment>>>,
}

impl CCallable {
    pub fn new(
        params: Vec<Token>,
        statement: Option<Box<Stmt>>,
        call: fn(
            &mut Interpreter,
            Rc<RefCell<Environment>>,
            &Vec<Token>,
            Vec<Object>,
            &Stmt,
        ) -> Result<Object, RuntimeError>,
        arity: usize,
        outer_environment: Option<Rc<RefCell<Environment>>>,
    ) -> Self {
        Self {
            params,
            statement,
            call,
            arity,
            outer_environment,
        }
    }
}
