use std::{cell::RefCell, collections::HashMap, rc::{Weak, Rc}};

use crate::{error::RuntimeError, token::*};

pub struct Environment {
    enclosing: Weak<RefCell<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: Weak::new(),
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Rc::downgrade(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.to_owned());
        } else if let Some(enclosing) = self.enclosing.upgrade() {
            enclosing.borrow().get(name)
        } else {
            Err(RuntimeError {
                token: name.to_owned(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            })
        }
    }

    pub fn assign(&mut self, name: Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else if let Some(enclosing) = self.enclosing.upgrade() {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError {
                token: name.to_owned(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            })
        }
    }
}
