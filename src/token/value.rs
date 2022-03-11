use crate::token::token_type::{TokenType, Literal};

use std::fmt;

// https://github.com/brightly-salty/rox/blob/master/src/value.rs
// The idea of Value apart from using Box<dyn Any> is good

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

impl fmt::Display for Value {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(v) => write!(f, "{}", v),
            Value::Number(v) => {
                let s = v.to_string();
                write!(
                    f,
                    "{}",
                    if s.ends_with(".0") {
                        &s[..(s.len() - 2)]
                    } else {
                        &s[..]
                    }
                )
            },
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),

        }
    }
}

impl From<TokenType> for Value {
    fn from (l: TokenType) -> Self {
        match l {
            TokenType::LITERAL(Literal::IDENTIFIER(v)) => Value::String(v),
            TokenType::LITERAL(Literal::NUMBER(v)) => Value::Number(v),
            TokenType::LITERAL(Literal::STRING(v)) => Value::String(v),
            TokenType::FALSE => Value::Bool(false),
            TokenType::TRUE => Value::Bool(true),
            _ => Value::Nil
        }
    }
} 
