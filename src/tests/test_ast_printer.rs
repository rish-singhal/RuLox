use crate::ast::node;
use crate::ast::node::*;
use crate::ast::ast_printer::AstPrinter;
use crate::token::token::Token;
use crate::token::token_type;
use crate::token::token_type::TokenType;

pub fn test_ast_printer() {
    let mut ast_printer = AstPrinter {};
    let expr = Expr::Binary(Binary {
        left: Box::new(
                Expr::Literal(
                    node::Literal { 
                        value: Token {
                            token_type: token_type::TokenType::LITERAL(
                                            token_type::Literal::NUMBER(1.0)),
                            lexeme: "1".to_string(),
                            line: 1
                        }
                    }
                    )
                ),
        operator: Token { 
            token_type: TokenType::PLUS,
            lexeme: "+".to_string(),
            line: 1
        },
        right: Box::new(
                   Expr::Literal(
                       node::Literal {
                           value: Token{
                               token_type: TokenType::LITERAL(
                                               token_type::Literal::NUMBER(
                                                   2.0
                                                   )
                                               ),
                               lexeme: "2".to_string(),
                               line: 2
                           }
                       }
                       )
                   ),
    });

    if let Expr::Binary(binary) = expr {
        println!("{}", binary.accept(&mut ast_printer));
    } else {
        panic!("Expected Expr::Binary");
    }
}

