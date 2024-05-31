use std::env;
use std::fs::File;
use std::io::prelude::*;

mod lexer;
mod token;
mod error;
mod ast;
mod parser;
mod parser_tests;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <filename>");
        return;
    }

    let filename = &args[1];
    let mut file = File::open(filename).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lexer = lexer::Lexer::new(&contents);
    lexer.tokenize();

    if !lexer.errors.is_empty() {
        for error in lexer.errors {
            eprintln!("Lexer error: {} at line: {}, column: {}", error.message, error.line, error.column);
        }
        return;
    }

    // Print tokens for now
    for token in lexer.tokens {
        println!("{:?}", token);
    }
}