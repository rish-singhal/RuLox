use crate::runtime_error;
use crate::ast::node::*;
use crate::token::token::Token;
use crate::token::token_type;
use crate::token::token_type::TokenType;

use std::any::{Any, TypeId};

fn evaluate(expr: &Box<Expr>) -> Result<TokenType, InterpreterError> {
    (*expr).accept(&mut Interpreter {})
}


// TODO: to implement printing of string type
pub fn interpret(expr: &Box<Expr>) {
    match evaluate(expr) {
        Ok(value) => {
            let value_type = (&*value).type_id();

             //TODO: simplify printing concrete type behind dyn Any by deconstructing? 
             if value_type == TypeId::of::<String>() {
                 println!("{}", value.downcast_ref::<String>().unwrap());
             } else if value_type == TypeId::of::<bool>() {
                 println!("{}", value.downcast_ref::<bool>().unwrap());
             } else if value_type == TypeId::of::<f64>() {
                 println!("{}", value.downcast_ref::<f64>().unwrap());
             } else {
                 println!("nil");
             }
        },
        Err(error) => {
            runtime_error(&error.token, error.message);
        }
    }
}

fn is_truthy (
    value: Result<Box<dyn Any>, InterpreterError>
) -> bool {
    match value {
        Ok(v) => { 
            if (&*v).type_id() == TypeId::of::<bool>() {
                return *v.downcast_ref::<bool>().unwrap();
            } else {
                true
            }
        }
        Err(_) => false
    }
}


// NOTE: struct A; works too apart from struct A {} 
pub struct Interpreter;


pub struct InterpreterError {
    pub token: Token,
    pub message: String,
}

impl Visitor for Interpreter {
    type R = Result<TokenType, InterpreterError>;

    fn visit_binary (&self, binary: &Binary) -> Self::R {
        let left = evaluate(&binary.left)?;
        let right = evaluate(&binary.right)?;

        // TODO: BangEqual and EqualEqual need to be implemented for all
        // types.
        //
        // TODO: can this be simplified?
        // adding different types?
        if let Some(left_str) = left.downcast_ref::<String>() {
            if let Some(right_str) = right.downcast_ref::<String>() {
                match binary.operator.token_type {
                    TokenType::PLUS =>
                        return Ok(Box::new(left_str.clone()
                                           + &right_str.clone())),
                    TokenType::BangEqual => 
                        return Ok(Box::new(left_str != right_str)),
                    TokenType::EqualEqual => 
                        return Ok(Box::new(left_str == right_str)),
                    _ => {
                        return Err(InterpreterError {
                            token: binary.operator.clone(),
                            message:
                                String::from(
                                    "Operator not supported on type Strings"
                                )
                        });
                    }
                }
            } else {
                return Err(InterpreterError {
                    token: binary.operator.clone(),
                    message:
                        String::from(
                            "Only 2 strings can be operated on together"
                        )
                });
            };
        };


        let left_value =  match left.downcast_ref::<f64>() {
            Some(f) => *f,
            None => {
                return Err(InterpreterError {
                    token: binary.operator.clone(),
                    message:
                        String::from("Operands must be numbers")
                });
            }
        };

        let right_value = match right.downcast_ref::<f64>() {
            Some(f) => *f,
            None => {
                return Err(InterpreterError {
                    token: binary.operator.clone(),
                    message:
                        String::from("Operands must be numbers")
                });
            }
        };

        // TODO: Implement BangEqual and EqualEqual for other types
        // https://craftinginterpreters.com/evaluating-expressions.html
        match binary.operator.token_type {
            TokenType::MINUS => Ok(Box::new(left_value - right_value)),
            TokenType::SLASH => Ok(Box::new(left_value / right_value)),
            TokenType::STAR => Ok(Box::new(left_value * right_value)),
            TokenType::PLUS => Ok(Box::new(left_value + right_value)),
            TokenType::GREATER => Ok(Box::new(left_value > right_value)),
            TokenType::GreaterEqual =>
                Ok(Box::new(left_value >= right_value)),
            TokenType::LESS => Ok(Box::new(left_value < right_value)),
            TokenType::LessEqual => Ok(Box::new(left_value <= right_value)),
            TokenType::BangEqual => Ok(Box::new(left_value != right_value)),
            TokenType::EqualEqual => Ok(Box::new(left_value == right_value)),
            _ => Err(InterpreterError {
                token: binary.operator.clone(),
                message: String::from("Unsupported operator")
            })
        }
    }

    fn visit_grouping (&self, grouping: &Grouping) -> Self::R {
        evaluate(&grouping.expression)
    }

    fn visit_literal (&self, literal: &Literal) -> Self::R {
        return literal.value.token_type.clone()
        match literal.value.token_type {
            TokenType::LITERAL(token_type::Literal::NUMBER(_)) =>
                Ok(
                    literal.value.lexeme
                        .parse::<f64>()
                        .map(Box::new)
                        .map_err(
                                |_| InterpreterError {
                                    token: literal.value.clone(),
                                    message: String::from("Literal NUMBER\
                                                          contains a not-\
                                                          float value.")
                                }
                        )?
                  ),
            TokenType::LITERAL(token_type::Literal::IDENTIFIER(_)) =>
                Ok(Box::new(literal.value.lexeme.to_string())),
            TokenType::LITERAL(token_type::Literal::STRING(_)) =>
                Ok(Box::new(literal.value.lexeme.to_string())),
            TokenType::TRUE => Ok(Box::new(true)),
            TokenType::FALSE => Ok(Box::new(false)),
            _ => Err(InterpreterError {
                token: literal.value.clone(),
                message: String::from("Literal not supported.")
            })
        }
    }

    fn visit_unary (&self, unary: &Unary) -> Self::R {
        let right = evaluate(&unary.right);
        // https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
        match unary.operator.token_type {
            TokenType::MINUS => {
                match right?.downcast_ref::<f64>() {
                    Some(f) => Ok(Box::new(-f)),
                    None => Err(InterpreterError {
                        token: unary.operator.clone(),
                        message: String::from("Operand must be a number.")
                    })
                }
            },
            TokenType::BANG => {
                match right {
                    Ok(right_value) => {
                        match right_value.downcast_ref::<bool>() {
                            Some(f) => Ok(Box::new(!f)),
                            None => Ok(Box::new(true))
                        }
                    },
                    Err(_) => Ok(Box::new(false))
                }
            },
            _ => Err(InterpreterError {
                token: unary.operator.clone(),
                message: String::from("Operator must be MINUS/BANG.")
            })
        }
    }
}

