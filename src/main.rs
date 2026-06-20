mod interpreter;

use crate::interpreter::lexer::Lexer;
use crate::interpreter::parser::Parser;
use std::fs;

fn main() {
    let source = fs::read_to_string("ai_compile.txt")
        .expect("Failed to read ai_compile.txt");

    let mut lexer = Lexer::new(source);

    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);

            match parser.parse() {
                Ok(program) => {
                    println!("Parsed successfully. AST:\n{:#?}", program);
                }
                Err(err) => {
                    eprintln!("Parser error: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Lexer error: {}", err);
        }
    }
}