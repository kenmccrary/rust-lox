use crate::scanner::Scanner;
use std::io::{BufRead, Write};
use std::{env, fs, io};
use crate::parser::Parser;

mod expr;
mod parser;
mod scanner;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Usage: rlox [script]"),
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().expect("IO flush failed");
        let mut line = String::new();
        let stdin = io::stdin();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Could not read line");

        run(line);
    }
}

fn run_file(path: &String) {
    let data = fs::read_to_string(path).expect("Unable to read file");
    run(data);
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens.clone());
    let expression = parser.parse();

    // For now, just print the tokens
    for token in tokens {
        println!("{:?}", token);
    }
}


