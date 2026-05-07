use crate::{Token, expr::Expr};

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    If(IfStmt),
}

pub trait StmtVisitor<R> {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> R;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> R;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> R;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> R;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> R;
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        match self {
            Stmt::Block(s) => visitor.visit_block_stmt(s),
            Stmt::Expression(s) => visitor.visit_expression_stmt(s),
            Stmt::Print(s) => visitor.visit_print_stmt(s),
            Stmt::Var(s) => visitor.visit_var_stmt(s),
            Stmt::If(s) => visitor.visit_if_stmt(s),
        }
    }
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
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

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
