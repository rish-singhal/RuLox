pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod tests;

use crate::lexer::scanner::Scanner;
use crate::tests::test_ast_printer::test_ast_printer;

use std::env;
use std::fs;
use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::process;

static mut HAD_ERROR: bool = false;

fn main() {
    let args_count = env::args().count();

    if args_count > 2 {
        println!("Usage: rulox [script]");
        process::exit(64); 
    } else if args_count == 2 {
        let script = env::args().nth(1).unwrap();
        if script == "test" {
            test_ast_printer();
        } else {
            run_file(env::args().nth(1).unwrap());
        }
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Error reading script");

    run(contents);

    unsafe{
        if HAD_ERROR {
            process::exit(65);
        }
    }
}

// READ-EVAL-PRINT-LOOP (REPL)
fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        if let Ok(_) = io::stdin().read_line(&mut line) {
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
    println!("source: {}", source);
    let tokens = Scanner::new(source).scan_tokens();
    println!("{:?}", tokens);
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

