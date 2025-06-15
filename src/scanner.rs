use std::collections::*;

use super::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        let source = source.chars().collect::<Vec<char>>();
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ret = self.source[self.current];
        self.current += 1;
        ret
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.tokens.push(Token::new(
                TokenType::LeftParen,
                String::from("("),
                None,
                self.line,
            )),
            ')' => self.tokens.push(Token::new(
                TokenType::RightParen,
                String::from(")"),
                None,
                self.line,
            )),
            '{' => self.tokens.push(Token::new(
                TokenType::LeftBrace,
                String::from("{"),
                None,
                self.line,
            )),
            '}' => self.tokens.push(Token::new(
                TokenType::RightBrace,
                String::from("}"),
                None,
                self.line,
            )),
            ',' => self.tokens.push(Token::new(
                TokenType::Comma,
                String::from(","),
                None,
                self.line,
            )),
            '.' => self.tokens.push(Token::new(
                TokenType::Dot,
                String::from("."),
                None,
                self.line,
            )),
            '-' => self.tokens.push(Token::new(
                TokenType::Minus,
                String::from("-"),
                None,
                self.line,
            )),
            '+' => self.tokens.push(Token::new(
                TokenType::Plus,
                String::from("+"),
                None,
                self.line,
            )),
            ';' => self.tokens.push(Token::new(
                TokenType::Semicolon,
                String::from(";"),
                None,
                self.line,
            )),
            '*' => self.tokens.push(Token::new(
                TokenType::Star,
                String::from("*"),
                None,
                self.line,
            )),
            '!' => {
                if self.match_next('=') {
                    self.tokens.push(Token::new(
                        TokenType::BangEqual,
                        String::from("!="),
                        None,
                        self.line,
                    ));
                } else {
                    self.tokens.push(Token::new(
                        TokenType::Bang,
                        String::from("!"),
                        None,
                        self.line,
                    ));
                }
            }

            '=' => {
                if self.match_next('=') {
                    self.tokens.push(Token::new(
                        TokenType::EqualEqual,
                        String::from("=="),
                        None,
                        self.line,
                    ));
                } else {
                    self.tokens.push(Token::new(
                        TokenType::Equal,
                        String::from("="),
                        None,
                        self.line,
                    ));
                }
            }

            '>' => {
                if self.match_next('=') {
                    self.tokens.push(Token::new(
                        TokenType::GreaterEqual,
                        String::from(">="),
                        None,
                        self.line,
                    ));
                } else {
                    self.tokens.push(Token::new(
                        TokenType::Greater,
                        String::from(">"),
                        None,
                        self.line,
                    ));
                }
            }

            '<' => {
                if self.match_next('=') {
                    self.tokens.push(Token::new(
                        TokenType::LessEqual,
                        String::from("<="),
                        None,
                        self.line,
                    ));
                } else {
                    self.tokens.push(Token::new(
                        TokenType::Less,
                        String::from("<"),
                        None,
                        self.line,
                    ));
                }
            }

            '/' => {
                if self.match_next('/') {
                    // for comment skip till end of line
                    while !self.is_at_end() && self.source[self.current] != '\n' {
                        self.advance();
                    }
                } else {
                    self.tokens.push(Token::new(
                        TokenType::Slash,
                        String::from("/"),
                        None,
                        self.line,
                    ));
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }

            '\n' => {
                self.line += 1;
            }

            '"' => self.handle_string(),

            _ => {
                if c.is_ascii_digit() {
                    self.handle_digit();
                } else if c.is_ascii_alphabetic() {
                    self.handle_identifier();
                } else {
                    todo!("Unexpected character: '{}'", c);
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'; // Return null character if at end
        }
        self.source[self.current]
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // Handle unterminated string error
            eprintln!("Error: Unterminated string at line {}", self.line);
            return;
        }

        self.advance(); // Consume the closing quote
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(TokenType::String, value, None, self.line));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn handle_digit(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // Consume the '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        self.tokens
            .push(Token::new(TokenType::Number, value, None, self.line));
    }

    fn handle_identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        match value.as_str() {
            "and" => {
                self.tokens
                    .push(Token::new(TokenType::And, value, None, self.line));
            }
            "class" => {
                self.tokens
                    .push(Token::new(TokenType::Class, value, None, self.line));
            }
            "else" => {
                self.tokens
                    .push(Token::new(TokenType::Else, value, None, self.line));
            }
            "false" => {
                self.tokens
                    .push(Token::new(TokenType::False, value, None, self.line));
            }
            "for" => {
                self.tokens
                    .push(Token::new(TokenType::For, value, None, self.line));
            }
            "fun" => {
                self.tokens
                    .push(Token::new(TokenType::Fun, value, None, self.line));
            }
            "if" => {
                self.tokens
                    .push(Token::new(TokenType::If, value, None, self.line));
            }
            "nil" => {
                self.tokens
                    .push(Token::new(TokenType::Nil, value, None, self.line));
            }
            "or" => {
                self.tokens
                    .push(Token::new(TokenType::Or, value, None, self.line));
            }
            "print" => {
                self.tokens
                    .push(Token::new(TokenType::Print, value, None, self.line));
            }
            "return" => {
                self.tokens
                    .push(Token::new(TokenType::Return, value, None, self.line));
            }
            "super" => {
                self.tokens
                    .push(Token::new(TokenType::Super, value, None, self.line));
            }
            "this" => {
                self.tokens
                    .push(Token::new(TokenType::This, value, None, self.line));
            }
            "true" => {
                self.tokens
                    .push(Token::new(TokenType::True, value, None, self.line));
            }
            "var" => {
                self.tokens
                    .push(Token::new(TokenType::Var, value, None, self.line));
            }
            "while" => {
                self.tokens
                    .push(Token::new(TokenType::While, value, None, self.line));
            }
            _ => {
                self.tokens
                    .push(Token::new(TokenType::Identifier, value, None, self.line));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_with_single_token() {
        let source = String::from("(");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let expected_token = Token::new(TokenType::LeftParen, String::from("("), None, 1);

        assert_eq!(tokens.len(), 2); // One token and one EOF
        assert_eq!(tokens[0].token_type, expected_token.token_type);
    }

    #[test]
    fn source_with_multiple_tokens() {
        let source = String::from("() <=");

        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, String::from("("), None, 1),
            Token::new(TokenType::RightParen, String::from(")"), None, 1),
            Token::new(TokenType::LessEqual, String::from("<="), None, 1),
            Token::new(TokenType::Eof, String::new(), None, 1),
        ];

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, expected_tokens[i].token_type);
        }
    }

    #[test]
    fn source_with_string_literal() {
        let source = String::from("\"Hello, World!\"");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let expected_token = Token::new(TokenType::String, String::from("Hello, World!"), None, 1);
        assert_eq!(tokens[0].token_type, expected_token.token_type);
    }

    #[test]
    fn source_with_integer() {
        let source = String::from("12345");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let expected = Token::new(TokenType::Number, String::from("12345"), None, 1);
        assert_eq!(tokens[0].token_type, expected.token_type);
    }

    #[test]
    fn source_with_float() {
        let source = String::from("123.45");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let expected = Token::new(TokenType::Number, String::from("123.45"), None, 1);
        assert_eq!(tokens[0].token_type, expected.token_type);
    }

    #[test]
    fn source_with_keyowrd_and_literal() {
        let source = String::from("var x = 10;");
        let mut scanner = Scanner::new(source);

        let expected = vec![
            Token::new(TokenType::Var, String::from("var"), None, 1),
            Token::new(TokenType::Identifier, String::from("x"), None, 1),
            Token::new(TokenType::Equal, String::from("="), None, 1),
            Token::new(TokenType::Number, String::from("10"), None, 1),
            Token::new(TokenType::Semicolon, String::from(";"), None, 1),
            Token::new(TokenType::Eof, String::new(), None, 1),
        ];

        for (i, token) in scanner.scan_tokens().iter().enumerate() {
            assert_eq!(token.token_type, expected[i].token_type);
        }
    }
}
