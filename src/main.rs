mod interpreter;
mod ai;

use crate::interpreter::jit::Jit;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::parser::Parser;
use std::fs;

fn run_compiler(source: String) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("Lexer error: {e}"); return; }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => { eprintln!("Parser error: {e}"); return; }
    };

    let mut jit = Jit::new();
    match jit.compile_and_run(&program) {
        Ok(code) => std::process::exit(code as i32),
        Err(e)   => eprintln!("JIT error: {e}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {

        Some(path) if path.ends_with(".airy") => {
            let source = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => { eprintln!("Could not read {path}: {e}"); return; }
            };

            println!("Translating {path} ...");

            let airy_core = match ai::client::translate(source) {
                Ok(code) => code,
                Err(e)   => { eprintln!("AI translation failed: {e}"); return; }
            };

            if let Err(e) = fs::write("code_wr.airy", &airy_core) {
                eprintln!("Could not write code_wr.airy: {e}");
                return;
            }

            println!("Written to code_wr.airy");
            run_compiler(airy_core);
        }

        None => {
            let source = match fs::read_to_string("code_wr.airy") {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("Usage:");
                    eprintln!("  airy file.airy   — translate via AI then run");
                    eprintln!("  airy             — run code_wr.airy directly");
                    return;
                }
            };
            run_compiler(source);
        }

        Some(other) => {
            eprintln!("Unknown argument: {other}");
            eprintln!("Usage:");
            eprintln!("  airy file.airy   — translate via AI then run");
            eprintln!("  airy             — run code_wr.airy directly");
        }
    }
}