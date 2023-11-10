use std::{cell::RefCell, fmt, rc::Rc};

use crate::interpreter;
use crate::parser;
use crate::printer;
use crate::scanner;
use crate::token::{Object, Token};

use self::interpreter::Interpretable;

pub trait ExprLike: fmt::Display + Interpretable {}
pub trait IntoExpr {
    fn into_expr(self) -> Expr;
}

// ========== Expr ==========

#[derive(Clone)]
pub struct Expr(pub Rc<RefCell<dyn ExprLike>>);

impl ExprLike for Expr {}

// ========== Unary ==========

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<dyn ExprLike>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: impl ExprLike + 'static) -> UnaryExpr {
        UnaryExpr {
            operator,
            right: Box::new(right),
        }
    }
}

impl ExprLike for UnaryExpr {}

impl IntoExpr for UnaryExpr {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}

// ========== Binary ==========

pub struct BinaryExpr {
    pub operator: Token,
    pub left: Box<dyn ExprLike>,
    pub right: Box<dyn ExprLike>,
}

impl BinaryExpr {
    pub fn new(
        operator: Token,
        left: impl ExprLike + 'static,
        right: impl ExprLike + 'static,
    ) -> BinaryExpr {
        BinaryExpr {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl ExprLike for BinaryExpr {}

impl IntoExpr for BinaryExpr {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}

// ========== Grouping ==========

pub struct GroupingExpr {
    pub expression: Box<dyn ExprLike>,
}

impl GroupingExpr {
    pub fn new<'a>(expression: impl ExprLike + 'static) -> GroupingExpr {
        GroupingExpr {
            expression: Box::new(expression),
        }
    }
}

impl ExprLike for GroupingExpr {}

impl IntoExpr for GroupingExpr {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}

// ===== Literal =====

pub struct LiteralExpr {
    pub value: Object,
}

impl LiteralExpr {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

impl ExprLike for LiteralExpr {}

impl IntoExpr for LiteralExpr {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}

// ===== VariableExpr =====

pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}
