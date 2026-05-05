use std::collections::HashMap;

use crate::{error::LoxError, token::Token, value::Value};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(LoxError::RuntimeError {
                line: name.line(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            })
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(LoxError::RuntimeError {
                line: name.line(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            }),
        }
    }
}
