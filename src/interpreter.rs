use std::fmt;

use crate::{
    error::LoxError,
    expr::{Binary, Expr, ExprVisitor, Grouping, LiteralExpr, Unary},
    token::{Literal, Token},
    token_type::TokenType,
};

pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

pub struct Interpreter;

fn is_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Nil, _) => false,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        _ => false,
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Bool(b) => *b,
        _ => true,
    }
}

fn check_number_operand(operator: &Token, operand: &Value) -> Result<(), LoxError> {
    match operand {
        Value::Number(_) => Ok(()),
        _ => Err(LoxError::RuntimeError {
            line: operator.line(),
            message: String::from("Operand must be a number"),
        }),
    }
}

fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> Result<(), LoxError> {
    match (left, right) {
        (Value::Number(_), Value::Number(_)) => Ok(()),
        _ => Err(LoxError::RuntimeError {
            line: operator.line(),
            message: String::from("Operands must be numbers"),
        }),
    }
}

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) -> Result<Value, LoxError> {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, LoxError> {
        expr.accept(self)
    }
}

impl ExprVisitor<Result<Value, LoxError>> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Value, LoxError> {
        match &expr.value {
            None => Ok(Value::Nil),
            Some(Literal::Number(n)) => Ok(Value::Number(*n)),
            Some(Literal::String(s)) => Ok(Value::String(s.clone())),
        }
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> Result<Value, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&self, expr: &Unary) -> Result<Value, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => {
                check_number_operand(&expr.operator, &right)?;
                match right {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => unreachable!(),
                }
            }
            TokenType::Bang => Ok(Value::Bool(!is_truthy(&right))),
            _ => unreachable!(),
        }
    }

    fn visit_binary_expr(&self, expr: &Binary) -> Result<Value, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => {
                check_number_operands(&expr.operator, &left, &right)?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                    _ => unreachable!(),
                }
            }
            TokenType::Slash => {
                check_number_operands(&expr.operator, &left, &right)?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                    _ => unreachable!(),
                }
            }
            TokenType::Star => {
                check_number_operands(&expr.operator, &left, &right)?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                    _ => unreachable!(),
                }
            }
            TokenType::Plus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                _ => Err(LoxError::RuntimeError {
                    line: expr.operator.line(),
                    message: String::from("Operands must be two numbers or two strings"),
                }),
            },
            TokenType::Greater => {
                check_number_operands(&expr.operator, &left, &right)?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                    _ => unreachable!(),
                }
            }
            TokenType::GreaterEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                    _ => unreachable!(),
                }
            }
            TokenType::Less => {
                check_number_operands(&expr.operator, &left, &right)?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                    _ => unreachable!(),
                }
            }
            TokenType::LessEqual => {
                check_number_operands(&expr.operator, &left, &right)?;
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                    _ => unreachable!(),
                }
            }
            TokenType::BangEqual => Ok(Value::Bool(!is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Value::Bool(is_equal(&left, &right))),
            _ => unreachable!(),
        }
    }
}
