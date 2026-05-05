use crate::{
    environment::Environment,
    error::LoxError,
    expr::{Assign, Binary, Expr, ExprVisitor, Grouping, LiteralExpr, Unary, Variable},
    stmt::{BlockStmt, ExpressionStmt, PrintStmt, Stmt, StmtVisitor, VarStmt},
    token::{Literal, Token},
    token_type::TokenType,
    value::Value,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
}

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
    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
        for stmt in statements {
            stmt.accept(self)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxError> {
        expr.accept(self)
    }
}

impl StmtVisitor<Result<(), LoxError>> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), LoxError> {
        let previous = std::mem::replace(&mut self.environment, Environment::new());
        self.environment = Environment::new_enclosed(previous);

        let result: Result<(), LoxError> = stmt.statements.iter().try_for_each(|s| s.accept(self));

        let block_env = std::mem::replace(&mut self.environment, Environment::new());
        self.environment = block_env.take_enclosing().expect("block env must have enclosing");

        result
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxError> {
        let value = match &stmt.initializer {
            Some(expr) => self.evaluate(expr)?,
            None => Value::Nil,
        };
        self.environment.define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

impl ExprVisitor<Result<Value, LoxError>> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<Value, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Value, LoxError> {
        match &expr.value {
            None => Ok(Value::Nil),
            Some(Literal::Number(n)) => Ok(Value::Number(*n)),
            Some(Literal::String(s)) => Ok(Value::String(s.clone())),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<Value, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<Value, LoxError> {
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

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<Value, LoxError> {
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

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Value, LoxError> {
        self.environment.get(&expr.name)
    }
}
