mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;

use interpreter::*;
use parser::*;
use scanner::*;
use token::*;
use token_type::*;

fn main() {
    let mut interpreter = Interpreter;

    while let Some(input) = std::io::stdin().lines().next() {
        match input {
            Ok(line) => {
                let (tokens, scan_errors) = Scanner::new(line).scan_tokens();

                for e in &scan_errors {
                    eprintln!("{}", e);
                }

                if !scan_errors.is_empty() {
                    continue;
                }

                match Parser::new(tokens).parse() {
                    Ok(stmts) => {
                        if let Err(e) = interpreter.interpret(&stmts) {
                            eprintln!("{}", e);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
