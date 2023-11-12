use std::{collections::HashMap, rc::Weak, cell::RefCell};

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

    pub fn new_enclosed(enclosing: Weak<RefCell<Environment>>) -> Self {
        Self {
            enclosing,
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
        if let Some(prev_value) = self.values.get_mut(&name.lexeme) {
            *prev_value = value;
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
