mod interpreter;

use crate::interpreter::lexer::Lexer;
use std::fs;

fn main() {
    let source = fs::read_to_string("ai_compile.txt")
        .expect("Failed to read ai_compile.txt");

    let mut lexer = Lexer::new(source);

    match lexer.tokenize() {
        Ok(tokens) => {
            println!("{:#?}", tokens);
        }
        Err(err) => {
            eprintln!("Lexer error: {}", err);
        }
    }
}