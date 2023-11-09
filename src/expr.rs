use std::{
    cell::RefCell,
    fmt,
    rc::Rc,
};

use crate::{
    interpreter::Interpretable,
    token::{Object, Token},
};

pub trait ExprLike: fmt::Display + Interpretable {}
pub trait IntoExpr {
    fn into_expr(self) -> Expr;
}

// pub trait ExprClone {
//     fn expr_clone(&self) -> Expr;
// }

#[derive(Clone)]
pub struct Expr(pub Rc<RefCell<dyn ExprLike>>);

impl ExprLike for Expr {}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow())
    }
}
// ========== Unary ==========

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn ExprLike>,
}

impl Unary {
    pub fn new(operator: Token, right: impl ExprLike + 'static) -> Unary {
        Unary {
            operator,
            right: Box::new(right),
        }
    }
}

impl ExprLike for Unary {}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

impl IntoExpr for Unary {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}

// ========== Binary ==========

pub struct Binary {
    pub operator: Token,
    pub left: Box<dyn ExprLike>,
    pub right: Box<dyn ExprLike>,
}

impl Binary {
    pub fn new(
        operator: Token,
        left: impl ExprLike + 'static,
        right: impl ExprLike + 'static,
    ) -> Binary {
        Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl ExprLike for Binary {}
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

impl IntoExpr for Binary {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}

// ========== Grouping ==========

pub struct Grouping {
    pub expression: Box<dyn ExprLike>,
}

impl Grouping {
    pub fn new<'a>(expression: impl ExprLike + 'static) -> Grouping {
        Grouping {
            expression: Box::new(expression),
        }
    }
}

impl ExprLike for Grouping {}
impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.expression)
    }
}

impl IntoExpr for Grouping {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}

// ===== Literal =====

pub struct Literal {
    pub value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

impl ExprLike for Literal {}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl IntoExpr for Literal {
    fn into_expr(self) -> Expr {
        Expr(Rc::new(RefCell::new(self)))
    }
}
