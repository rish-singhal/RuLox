use std::env;
use std::fs;
use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::process;

fn main() {
    let args_count = env::args().count();
    if args_count > 2 {
        println!("Usage: rulox [script]");
        process::exit(64); 
    } else if args_count == 2 {
        run_file(env::args().nth(1).unwrap());
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Error reading script");
    run(contents);
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        if let Ok(_) = io::stdin().read_line(&mut line) {
            run(line);
        } else {
            break;
        }
    }
}

fn run(lines: String) {
    println!("{}", lines);
}
