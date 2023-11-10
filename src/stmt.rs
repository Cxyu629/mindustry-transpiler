use std::{cell::RefCell, fmt, rc::Rc};

use crate::{expr::Expr, interpreter::Interpretable, token::Token};

pub trait StmtLike: Interpretable + IntoStmt {}
pub trait IntoStmt {
    fn into_stmt(self) -> Stmt;
}

// ========== Stmt ==========

#[derive(Clone)]
pub struct Stmt(pub Rc<RefCell<dyn StmtLike>>);

impl StmtLike for Stmt {}

impl IntoStmt for Stmt {
    fn into_stmt(self) -> Stmt {
        todo!()
    }
}

// ========== ExpressionStmt ==========

#[derive(Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

impl ExpressionStmt {
    pub fn new(expression: Expr) -> Self {
        todo!()
    }
}

impl StmtLike for ExpressionStmt {}

impl IntoStmt for ExpressionStmt {
    fn into_stmt(self) -> Stmt {
        todo!()
    }
}

// ========== VarStmt ==========

#[derive(Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initialiser: Expr,
}

impl StmtLike for VarStmt {}

impl IntoStmt for VarStmt {
    fn into_stmt(self) -> Stmt {
        todo!()
    }
}
