use crate::token::token::Token;
use crate::token::value::Value;

use std::collections::HashMap;

pub struct Environment {
    pub values: HashMap<String, Value>,
}

pub struct RunTimeError {
    pub name: Token,
    pub message: String,
}

impl Default for Environment {
    fn default() -> Self {
        Environment { values: HashMap::new() }
    }
} 

impl Environment {
    pub fn new() -> Self {
       Environment { values: HashMap::new() } 
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), RunTimeError>{
        match self.values.get(&name.lexeme) {
            Some(_) => {
                *self.values.get_mut(&name.lexeme).unwrap() = value;
                Ok(())
            },
            None => return Err(RunTimeError {
                name: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            }),
        }
    }

    pub fn define_var(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get_var(&mut self, name: &Token) -> Result<Value, RunTimeError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok((*v).clone()),
            None => return Err(RunTimeError {
                name: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            }),
        }
    }
}
