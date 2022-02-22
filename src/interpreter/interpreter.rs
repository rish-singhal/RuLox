#[cfg(not_implementd)]
use crate::ast::node::*;
#[cfg(not_implementd)]
use crate::token::token_type;
#[cfg(not_implementd)]
use crate::token::token_type::TokenType;

#[cfg(not_implementd)]
use std::any::Any;

#[cfg(not_implementd)]
fn evaluate(expr: &Box<Expr>) -> Option<Box<dyn Any>> {
    (*expr).accept(&mut Interpreter {})
}

#[cfg(not_implementd)]
pub struct Interpreter {}

#[cfg(not_implementd)]
impl Visitor for Interpreter {
    type R = Option<Box<dyn Any>>;

    fn visit_binary (&self, binary: &Binary) -> Self::R {
    }

    fn visit_grouping (&self, grouping: &Grouping) -> Self::R {
        evaluate(&grouping.expression)
    }

    fn visit_literal (&self, literal: &Literal) -> Self::R {
        match literal.value.token_type {
            TokenType::LITERAL(token_type::Literal::NUMBER(_)) =>
                Some(Box::new(literal.value.lexeme.parse::<f64>().unwrap())),
            TokenType::LITERAL(token_type::Literal::IDENTIFIER(_)) =>
                Some(Box::new(literal.value.lexeme.to_string())),
            TokenType::LITERAL(token_type::Literal::STRING(_)) =>
                Some(Box::new(literal.value.lexeme.to_string())),
            TokenType::TRUE => Some(Box::new(true)),
            TokenType::FALSE => Some(Box::new(false)),
            _ => None
        }
    }

    fn visit_unary (&self, unary: &Unary) -> Self::R {
    // TODO: https://github.com/rust-lang/rust/issues/94218
    // check the github issue link 
        if let Some(right) = evaluate(&unary.right) {
            match unary.operator.token_type {
                TokenType::MINUS => 
                    Some(Box::new(-((*right).downcast::<f64>().unwrap()))),
                TokenType::BANG =>
                    Some(Box::new(!((*right).downcast::<bool>().unwrap()))),
                _ => None
            }
        } else {
            None
        }
    }
}

