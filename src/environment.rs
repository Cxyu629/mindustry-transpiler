use std::collections::HashMap;

use crate::token::*;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}
