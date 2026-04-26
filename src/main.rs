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
                let tokens = Scanner::new(line).scan_tokens();
                let mut parser = Parser::new(tokens);
                let expr = parser.parse();
                let printer = ast_print::AstPrinter;
                println!("{}", expr.accept(&printer));
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
