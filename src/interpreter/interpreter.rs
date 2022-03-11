use crate::runtime_error;
use crate::ast::node::*;
use crate::token::token::Token;
use crate::token::token_type::TokenType;
use crate::token::value::Value;

use super::environment::*;

// NOTE: struct A; works too apart from struct A {} 

// https://www.youtube.com/watch?v=juIINGuZyBc : Rust Lifetime
pub struct Interpreter<'a> {
    environment: &'a mut Environment,
}

impl<'a> From<&'a mut Environment> for Interpreter<'a> {
    fn from(environment: &'a mut Environment) -> Self {
        Interpreter { environment }
    }
}

impl Interpreter<'_> {

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        (*expr).accept(self)
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            if let Err(error) = self.execute(stmt) {
                runtime_error(&error.token, error.message)
            }
        }
    }

    fn is_truthy(&self, value: &Value) -> Value {
        match *value {
            Value::Nil => Value::Bool(false),
            Value::Bool(b) => Value::Bool(b),
            _ => Value::Bool(true)
        }
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b)  {
            (Value::Nil, Value::Nil) => true,
            (Value::Nil, _) => false,
            (Value::Number(u), Value::Number(v)) => (u - v).abs() <= 1e-6, 
            _ => a == b,
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Value, InterpreterError>{
        stmt.accept(self)
    }
}


pub struct InterpreterError {
    pub token: Token,
    pub message: String,
}

impl From<RunTimeError> for InterpreterError {
    fn from(error: RunTimeError) -> Self {
        InterpreterError {
            token: error.name,
            message: error.message,
        }
    }
}


impl Visitor for Interpreter<'_> {
    type R = Result<Value, InterpreterError>;

    fn visit_assign(&mut self, assign: &Assign) -> Self::R {
        let value = self.evaluate(&assign.value)?;
        self.environment.assign(assign.name.clone(), value.clone())?; 
        return Ok(value);
    }

    fn visit_binary (&mut self, binary: &Binary) -> Self::R {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary.operator.token_type {
            TokenType::BangEqual => 
                Ok(Value::Bool(!self.is_equal(&left, &right))),
            TokenType::EqualEqual =>
                Ok(Value::Bool(self.is_equal(&left, &right))),
            _ => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        match binary.operator.token_type {
                            TokenType::MINUS => Ok(Value::Number(a - b)),
                            TokenType::SLASH => Ok(Value::Number(a / b)),
                            TokenType::STAR => Ok(Value::Number(a * b)),
                            TokenType::PLUS => Ok(Value::Number(a + b)),
                            TokenType::GREATER => Ok(Value::Bool(a > b)),
                            TokenType::GreaterEqual => Ok(Value::Bool(a >= b)),
                            TokenType::LESS => Ok(Value::Bool(a < b)),
                            TokenType::LessEqual => Ok(Value::Bool(a <= b)),
                            _ => Err(InterpreterError {
                                token: binary.operator.clone(),
                                message:
                                    String::from(
                                        "Operator not supported on type \
                                        Numbers"
                                    )
                            })
                        }
                    },
                    (Value::String(a), Value::String(b)) => {
                        match binary.operator.token_type {
                            TokenType::PLUS => Ok(Value::String(a + &b)),
                            _ => Err(InterpreterError {
                                token: binary.operator.clone(),
                                message:
                                    String::from(
                                        "Operator not supported on type \
                                        Strings"
                                    )
                            })
                        }
                    },
                    _ => Err(InterpreterError {
                        token: binary.operator.clone(),
                        message:
                            String::from(
                                "Invalid operation"
                            )
                        }),
                    }
            }
        }

    }

    fn visit_grouping (&mut self, grouping: &Grouping) -> Self::R {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal (&mut self, literal: &Literal) -> Self::R {
        Ok(Value::from(literal.value.token_type.clone()))
    }

    fn visit_unary (&mut self, unary: &Unary) -> Self::R {
        let right = self.evaluate(&unary.right)?;

        // https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
        // my previous implementation^ used Box<dyn Any>
        match unary.operator.token_type {
            TokenType::MINUS => {
                match right {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err(InterpreterError {
                        token: unary.operator.clone(),
                        message: String::from("Only number can be operated \
                                              on with unary MINUS opeartor")
                    }),
                }
            },
            TokenType::BANG => {
                if let Value::Bool(b) = self.is_truthy(&right) {
                    Ok(Value::Bool(!b))
                } else {
                    // always returns a type of Value::Bool
                    Err(InterpreterError {
                        token: unary.operator.clone(),
                        message: String::from("Bang Operator only works on \
                                              literal")
                    })
                }
            },
            _ => Err(InterpreterError {
                token: unary.operator.clone(),
                message: String::from("Unary Operator must be MINUS/BANG.")
            })
        }
    }

    fn visit_variable (&mut self, var: &Variable) -> Self::R {
        Ok(self.environment.get_var(&var.name)?)
    }
}

impl StmtVisitor for Interpreter<'_> {
    type R = Result<Value, InterpreterError>;

    fn visit_expression (&mut self, expression: &Expression) -> Self::R {
        self.evaluate(&expression.expression)?;
        Ok(Value::Nil)
    }

    fn visit_print (&mut self, print: &Print) -> Self::R {
        let value = self.evaluate(&print.expression)?;
        println!("{} ", value);
        Ok(Value::Nil)
    }

    fn visit_var (&mut self, var: &Var) -> Self::R {
        let mut val = Value::Nil;
        if let Some(ini) = &var.initializer {
            val = self.evaluate(ini)?;
        }

        self.environment.define_var(var.name.lexeme.clone(), val);
        Ok(Value::Nil)
    }
}

