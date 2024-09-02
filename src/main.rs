use std::{env::args, io::stdout, io::Write};
mod error;
mod expr;
pub mod parser;
mod scanner;
mod token;
mod token_type;

mod ast_printer;

use scanner::Scanner;
use std::io::{self, BufRead};
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => {
            run_prompt();
        }
        2 => {
            run_file(&args[1]).expect("could not run file");
        }
        _ => {
            println!("Incorrect Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => (),
        Err(_) => {
            std::process::exit(65);
        }
    }

    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line);
        }
        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run(source: String) -> Result<(), error::LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Some(expr) => {
            let printer = ast_printer::AstPrinter{};
            println!("{}\n", printer.print(&expr)?);
        },
        None => return Ok(()),
    }
    Ok(())
}
