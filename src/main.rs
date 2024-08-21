use std::env::args;
mod error;
mod scanner;
mod token;
mod token_type;
use scanner::Scanner;
use std::io::{self, BufRead};
use token::Token;
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: lox-ast [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => (),
        Err(err) => {
            err.report("".to_string());
            std::process::exit(65);
        }
    }

    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    print!(">");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => (),
                Err(err) => err.report("".to_string()),
            }
        }
    }
}

fn run(source: String) -> Result<(), error::LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
