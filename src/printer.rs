use std::fmt;

use crate::{expr::*, stmt::*};

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", operator.lexeme, left, right),
            Expr::Grouping { expression } => write!(f, "({})", expression),
            Expr::Literal { value } => write!(f, "{}", value),
            Expr::Variable { name } => write!(f, "{}", name.lexeme),
            Expr::Assign { name, value } => write!(f, "({} = {})", name.lexeme, value),
            Expr::Logical {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", left, operator.lexeme, right),
            Expr::Call {
                callee, arguments, ..
            } => write!(f, "call({})({:?})", callee, arguments),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Expression { expression } => write!(f, "expr({})", expression),
            Stmt::Print { expression } => write!(f, "print({})", expression),
            Stmt::Var { name, initialiser } => {
                write!(f, "var({}", name.lexeme)?;
                if let Some(x) = &initialiser {
                    write!(f, ", {x}")?;
                }
                write!(f, ")")
            }
            Stmt::Block { statements } => {
                write!(f, "{{")?;
                for statement in (&statements).into_iter() {
                    write!(
                        f,
                        "\n{}",
                        "  ".to_owned() + &statement.to_string().replace("\n", "\n  ")
                    )?;
                }
                write!(f, "\n}}")
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(f, "if({})", condition)?;
                write!(f, "{}", then_branch)?;
                match else_branch {
                    Some(x) => write!(f, "{}", x),
                    None => write!(f, ""),
                }
            }
            Stmt::While { condition, body } => write!(f, "while({}){{{}}}", condition, body),
            Stmt::Function { name, .. } => write!(f, "fun({})", name.lexeme),
            Stmt::Return { value, .. } => write!(f, "return({:?})", value),
            Stmt::Blank {} => write!(f, ""),
        }
    }
}
