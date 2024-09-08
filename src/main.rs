use std::{env::args, io::stdout, io::Write};
mod error;
mod expr;
pub mod parser;
mod scanner;
mod token;
mod token_type;

mod ast_printer;
mod object;
mod interpreter;
mod stmt;
mod environment;

use scanner::Scanner;
use std::io::{self, BufRead};
use crate::error::LoxError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = args().collect();
    let lox = Lox::new();
    match args.len() {
        1 => {
            lox.run_prompt();
        }
        2 => {
            lox.run_file(&args[1]).expect("could not run file");
        }
        _ => {
            println!("Incorrect Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            interpreter: Interpreter::new(),
        }
    }


    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        match self.run(buf) {
            Ok(_) => (),
            Err(_) => {
                std::process::exit(65);
            }
        }

        Ok(())
    }
    pub fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        stdout().flush().unwrap();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                let _ = self.run(line);
            }
            print!("> ");
            stdout().flush().unwrap();
        }
    }

    fn run(&self, source: String) -> Result<(), error::LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse()?;
        println!("{}", parser.success());
        if parser.success() && self.interpreter.interpret(&stmts) {
            Ok(())
        } else {
            Err(LoxError::new(0, "could not interpret"))
        }
    }
}
