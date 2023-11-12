use std::fmt;

use crate::{expr::*, stmt::*};

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            Expr::Logical { operator, left, right } => todo!(),
            Expr::Call { callee, paren, arguments } => todo!(),
        }
    }
}

// impl fmt::Display for Stmt {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Stmt::Expression { expression } => write!(f, "expr({})", expression),
//             Stmt::Print { expression } => write!(f, "print({})", expression),
//             Stmt::Var { name, initialiser } => {
//                 write!(f, "var({}", name.lexeme)?;
//                 if let Some(x) = &initialiser {
//                     write!(f, ", {x}")?;
//                 }
//                 write!(f, ")")
//             }
//             Stmt::Block { statements } => {
//                 write!(f, "{{")?;
//                 for statement in (&statements).into_iter() {
//                     f.pad("\n")?;
//                     statement.fmt(f)?;
//                 }
//                 write!(f, "\n}}")
//             }
//             Stmt::If {
//                 condition,
//                 then_branch,
//                 else_branch,
//             } => ,
//             Stmt::While { condition, body } => todo!(),
//             Stmt::DoWhile { condition, body } => todo!(),
//         }
//     }
// }
