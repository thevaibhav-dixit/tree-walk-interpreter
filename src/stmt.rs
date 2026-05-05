use crate::{Token, expr::Expr};

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> R;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> R;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> R;
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        match self {
            Stmt::Expression(s) => visitor.visit_expression_stmt(s),
            Stmt::Print(s) => visitor.visit_print_stmt(s),
            Stmt::Var(s) => visitor.visit_var_stmt(s),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}
