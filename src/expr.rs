#![allow(dead_code)]
use super::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(LiteralExpr),
    Unary(Unary),
}

pub trait ExprVisitor<R> {
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_grouping_expr(&self, expr: &Grouping) -> R;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> R {
        match self {
            Expr::Binary(binary) => visitor.visit_binary_expr(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping_expr(grouping),
            Expr::Literal(literal) => visitor.visit_literal_expr(literal),
            Expr::Unary(unary) => visitor.visit_unary_expr(unary),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Expr) -> Expr {
        Expr::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: Option<Literal>,
}

impl LiteralExpr {
    pub fn new(value: Option<Literal>) -> Expr {
        Expr::Literal(LiteralExpr { value })
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Unary {
            operator,
            right: Box::new(right),
        })
    }
}

mod ast_print {
    use super::*;

    pub struct AstPrinter;

    impl ExprVisitor<String> for AstPrinter {
        fn visit_binary_expr(&self, expr: &Binary) -> String {
            format!(
                "({} {} {})",
                expr.left.accept(self),
                expr.operator.lexeme,
                expr.right.accept(self)
            )
        }

        fn visit_grouping_expr(&self, expr: &Grouping) -> String {
            format!("(group {})", expr.expression.accept(self))
        }
        fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
            match &expr.value {
                Some(value) => value.to_string(),
                None => "nil".to_string(),
            }
        }

        fn visit_unary_expr(&self, expr: &Unary) -> String {
            format!("({} {})", expr.operator.lexeme, expr.right.accept(self))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenType;

    use super::{ast_print::*, *};

    #[test]
    fn test_trivial_expr() {
        let expr = Binary::new(
            LiteralExpr::new(Some(Literal::Number(1.0))),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            LiteralExpr::new(Some(Literal::Number(1.0))),
        );

        let printer = AstPrinter;
        let result = expr.accept(&printer);
        assert_eq!(result, "(1 - 1)");
    }

    #[test]
    fn test_complicated_expr() {
        let expr = Binary::new(
            Unary::new(
                Token::new(TokenType::Minus, "-".to_string(), None, 1),
                LiteralExpr::new(Some(Literal::Number(123 as f64))),
            ),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Grouping::new(LiteralExpr::new(Some(Literal::Number(45.67)))),
        );

        let printer = AstPrinter;
        let result = expr.accept(&printer);
        assert_eq!(result, "((- 123) * (group 45.67))");
    }
}
