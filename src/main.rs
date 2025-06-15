mod expr;
mod scanner;
mod token;
mod token_type;

use scanner::*;
use token::*;
use token_type::*;

fn main() {
    while let Some(input) = std::io::stdin().lines().next() {
        match input {
            Ok(line) => {
                Scanner::new(line)
                    .scan_tokens()
                    .into_iter()
                    .for_each(|token| {
                        println!("{:?}", token);
                    });
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
