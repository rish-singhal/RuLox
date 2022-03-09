use crate::ast::node::*;

pub struct AstPrinter {}

fn paranthesize(name: String, exprs: &[&Expr]) -> String {
    let mut ast_printer = AstPrinter {};
    let mut builder: String = String::new();
    builder.push_str(&("(".to_string() + &name));
    for expr in exprs {
        builder.push_str(&(" ".to_string()));
        builder.push_str(&(*expr).accept(&mut ast_printer));
    }
    builder.push_str(&(")".to_string()));
    builder
}

impl Visitor for AstPrinter {
    type R = String;

    // can clone() be avoided?
    fn visit_binary (&self, binary: &Binary) -> Self::R {
        paranthesize(
            binary.operator.lexeme.clone(),
            &[&binary.left, &binary.right]
        ) 
    }

    fn visit_grouping (&self, grouping: &Grouping) -> Self::R {
        paranthesize("group".to_string(), &[&grouping.expression])
    }

    fn visit_literal (&self, literal: &Literal) -> Self::R {
        literal.value.lexeme.clone()
    }

    fn visit_unary (&self, unary: &Unary) -> Self::R {
       paranthesize(unary.operator.lexeme.clone(), &[&unary.right])
    }
}

