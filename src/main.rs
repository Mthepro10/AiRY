mod interpreter;

use crate::interpreter::jit::Jit;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::parser::Parser;
use std::fs;

fn main() {
    let source = fs::read_to_string("code_wr.airy").expect("Failed to read code_wr.airy");

    let mut lexer = Lexer::new(source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            return;
        }
    };

    let mut jit = Jit::new();
    match jit.compile_and_run(&program) {
        Ok(exit_code) => {
            std::process::exit(exit_code as i32);
        }
        Err(err) => {
            eprintln!("JIT compile error: {}", err);
        }
    }
}
