use std::collections::HashMap;

use crate::{error::LoxError, token::Token, value::Value};

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosed(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }
        Err(LoxError::RuntimeError {
            line: name.line(),
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }
        Err(LoxError::RuntimeError {
            line: name.line(),
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }

    pub fn take_enclosing(self) -> Option<Environment> {
        self.enclosing.map(|e| *e)
    }
}
