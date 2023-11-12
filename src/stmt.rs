use std::{rc::Rc, fmt};

use crate::{expr::Expr, interpreter::Interpretable, token::Token};

pub trait StmtLike: fmt::Display + Interpretable {}
pub trait IntoStmt {
    fn into_stmt(self) -> Stmt;
}

// ========== Stmt ==========

#[derive(Clone)]
pub struct Stmt(pub Rc<dyn StmtLike>);

impl StmtLike for Stmt {}

// ========== ExpressionStmt ==========

#[derive(Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

impl ExpressionStmt {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl StmtLike for ExpressionStmt {}

impl IntoStmt for ExpressionStmt {
    fn into_stmt(self) -> Stmt {
        Stmt(Rc::new(self))
    }
}

// ========== PrintStmt ==========

#[derive(Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}

impl PrintStmt {
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
}

impl StmtLike for PrintStmt {}

impl IntoStmt for PrintStmt {
    fn into_stmt(self) -> Stmt {
        Stmt(Rc::new(self))
    }
}

// ========== VarStmt ==========

#[derive(Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initialiser: Option<Expr>,
}

impl VarStmt {
    pub fn new(name: Token, initialiser: Option<Expr>) -> Self {
        Self { name, initialiser }
    }
}

impl StmtLike for VarStmt {}

impl IntoStmt for VarStmt {
    fn into_stmt(self) -> Stmt {
        Stmt(Rc::new(self))
    }
}
