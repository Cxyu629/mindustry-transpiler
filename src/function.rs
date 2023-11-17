use std::{cell::RefCell, iter::zip, rc::Rc, time::SystemTime};

use crate::{
    environment::Environment,
    error::RuntimeError,
    interpreter::Interpreter,
    stmt::Stmt,
    token::{Object, Token},
};

use Object as Ob;

pub fn clock(
    _interpreter: &mut Interpreter,
    _environment: Rc<RefCell<Environment>>,
    _params: &Vec<Token>,
    _arguments: Vec<Object>,
    _statement: &Stmt,
) -> Result<Object, RuntimeError> {
    return Ok(Object::Number(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards.")
            .as_secs_f32(),
    ));
}

pub fn deg(
    _interpreter: &mut Interpreter,
    _environment: Rc<RefCell<Environment>>,
    params: &Vec<Token>,
    arguments: Vec<Object>,
    _statement: &Stmt,
) -> Result<Object, RuntimeError> {
    if let Some(Ob::Number(x)) = arguments.get(0) {
        Ok(Ob::Degree(*x))
    } else {
        Err(RuntimeError {
            token: params.get(0).unwrap().clone(),
            message: "Expected `Number` type.".to_owned(),
        })
    }
}

pub fn run_inner(
    interpreter: &mut Interpreter,
    environment: Rc<RefCell<Environment>>,
    params: &Vec<Token>,
    arguments: Vec<Object>,
    statement: &Stmt,
) -> Result<Object, RuntimeError> {
    let inner_environment = Rc::new(RefCell::new(Environment::new_enclosed(&environment)));
    for (param, arg) in zip(params, arguments) {
        inner_environment
            .borrow_mut()
            .define(param.lexeme.clone(), arg);
    }

    let result = interpreter.execute(&inner_environment, statement)?;
    match result {
        Object::Return { value } => Ok(*value),
        _ => Ok(result),
    }
}
