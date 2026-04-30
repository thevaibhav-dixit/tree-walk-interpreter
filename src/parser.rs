use crate::{TokenType, error::LoxError, expr::*, token::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Literal::String("false".to_string())),
            }));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Literal::String("true".to_string())),
            }));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr { value: None }));
        }
        if self.match_token(&[TokenType::Number]) {
            let value = match &self.previous().literal {
                Some(Literal::Number(n)) => *n,
                _ => 0.0,
            };
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Literal::Number(value)),
            }));
        }
        if self.match_token(&[TokenType::String]) {
            let value = match &self.previous().literal {
                Some(Literal::String(s)) => s.clone(),
                _ => String::new(),
            };
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Literal::String(value)),
            }));
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(expr),
            }));
        }

        Err(LoxError::ParseError {
            line: self.peek().line(),
            lexeme: self.peek().lexeme.clone(),
            message: String::from("Expected expression"),
        })
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), LoxError> {
        if self.check(&token_type) {
            self.advance();
            Ok(())
        } else {
            Err(LoxError::ParseError {
                line: self.peek().line(),
                lexeme: self.peek().lexeme.clone(),
                message: message.to_string(),
            })
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
