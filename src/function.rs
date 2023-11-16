use std::{cell::RefCell, iter::zip, rc::Rc, time::SystemTime};

use crate::{
    environment::Environment,
    error::RuntimeError,
    interpreter::Interpreter,
    stmt::Stmt,
    token::{Object, Token},
};

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

// pub fn deg(
//     _interpreter: &mut Interpreter,
//     _environment: Rc<RefCell<Environment>>,
//     _params: &Vec<Token>,
//     _arguments: Vec<Object>,
//     _statement: &Stmt,
// ) -> Result<Object, RuntimeError> {
//     ;
// }

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
