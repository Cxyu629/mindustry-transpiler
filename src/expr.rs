use std::{fmt, rc::Rc};

use crate::interpreter;

use crate::token::{Object, Token};

use self::interpreter::Interpretable;

use crate::expr_like;

// pub trait Visitor<T> {
//     fn visit_unary(&mut self, expr: UnaryExpr) -> T;
//     fn visit_binary(&mut self, expr: BinaryExpr) -> T;
//     fn visit_grouping(&mut self, expr: GroupingExpr) -> T;
//     fn visit_litera(&mut self, expr: LiteralExpr) -> T;
//     fn visit_variable(&mut self, expr: Variable) -> T;
//     fn visit_assign(&mut self, expr: AssignExpr) -> T;
// }

pub trait ExprLike: fmt::Display + Interpretable {}

pub trait IntoExpr {
    fn into_expr(self) -> Expr;
}

// ========== Expr ==========

#[derive(Clone)]
pub struct Expr(pub Rc<dyn ExprLike>);

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

expr_like! {UnaryExpr}

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

expr_like! {BinaryExpr}

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

expr_like! {GroupingExpr}

// ===== Literal =====

pub struct LiteralExpr {
    pub value: Object,
}

impl LiteralExpr {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

expr_like! {LiteralExpr}

// ===== VariableExpr =====

pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

expr_like! {Variable}
// ===== AssignExpr =====

pub struct AssignExpr {
    pub name: Token,
    pub value: Box<dyn ExprLike>,
}

impl AssignExpr {
    pub fn new(name: Token, value: impl ExprLike) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}

expr_like! {AssignExpr}
