use super::TokenType;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
