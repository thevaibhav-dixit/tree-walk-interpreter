mod error;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

use expr::*;
use parser::*;
use scanner::*;
use token::*;
use token_type::*;

fn main() {
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
                    Ok(expr) => {
                        let printer = ast_print::AstPrinter;
                        println!("{}", expr.accept(&printer));
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
