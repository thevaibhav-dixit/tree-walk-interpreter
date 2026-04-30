use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("[line {line}] Scan error: {message}")]
    ScanError { line: usize, message: String },
    #[error("[line {line}] Parse error at '{lexeme}': {message}")]
    ParseError {
        line: usize,
        lexeme: String,
        message: String,
    },
}
