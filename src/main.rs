pub mod ast;
pub mod interpreter;
pub mod parser;
pub mod tests;
pub mod token;
pub mod lexer;

// use ast::ast_printer::AstPrinter;
// use ast::node::*;

use interpreter::interpreter::Interpreter;
use parser::parser::Parser;
use token::token::Token;
use token::token_type::TokenType;
use lexer::scanner::Scanner;
use tests::test_ast_printer::test_ast_printer;

use std::env;
use std::fs;
use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::process;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

fn main() {
    let args_count = env::args().count();

    match args_count {
        1 => run_prompt(),
        2 => {
            let script = env::args().nth(1).unwrap();
            if script == "test" {
                test_ast_printer();
            } else {
                run_file(env::args().nth(1).unwrap());
            }
        }
        _ => {
            eprintln!("Usage: rulox [script]");
            process::exit(64);
        }
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Error reading script");

    run(contents);

    unsafe{
        if HAD_ERROR {
            process::exit(65);
        } else if HAD_RUNTIME_ERROR {
            process::exit(70);
        }
    }
}

// READ-EVAL-PRINT-LOOP (REPL)
fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        if io::stdin().read_line(&mut line).is_ok() {
            run(line);
            unsafe {
                HAD_ERROR = false;
            }
        } else {
            break;
        }
    }
}

fn run(source: String) {
    // println!("source: {}", source);

    let tokens = Scanner::new(source).scan_tokens();

    unsafe {
        if HAD_ERROR {
            return;
        }
    }

    // println!("tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    if let Some(expr) = parser.parse() {
        // match &(*expr) {
        //     Expr::Binary(n) => println!("AST: {}", n.accept(&mut AstPrinter {})),
        //     Expr::Grouping(n)=> println!("AST: {}", n.accept(&mut AstPrinter {})),
        //     Expr::Literal(n) => println!("AST: {}", n.accept(&mut AstPrinter {})),
        //     Expr::Unary(n)=> println!("AST: {}", n.accept(&mut AstPrinter {})),
        // };
        let interpreter = Interpreter {};
        interpreter.interpret(&expr);
    }
}

pub fn runtime_error(token: &Token, message: String) {
    eprintln!("{}", message);
    eprintln!("[line {}]", token.line);

    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
}

pub fn error_token(token: &Token, message: String) {
    match token.token_type {
        TokenType::EOF => report(token.line, " at end".to_string(), message),
        _ => report(token.line,
                    format!(" at '{}'", token.lexeme).to_string(),
                    message)
    }
}

pub fn error(line: u32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: u32, where_error: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, where_error, message);
    unsafe {
        HAD_ERROR = true;
    }
}

