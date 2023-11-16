use crate::{expr::Expr, token::Token};

#[macro_use]
mod macros {
    macro_rules! stmt {
        {$([$name: ident, $func: ident, [$($field: tt: $t: ty, )*]], )+} => {
            #[derive(Clone, Debug)]
            pub enum Stmt {
                $($name {
                    $(
                        $field: $t,
                    )*
                },)+
                // Blank,
            }

            impl Stmt {
                $(
                    pub fn $func($($field: $t, )*) -> Stmt {
                        Stmt::$name {
                            $(
                                $field,
                            )*
                        }
                    }
                )+
            }
        };
    }
}

stmt! {
    [Blank, new_blank, []],
    [Block, new_block, [statements: Vec<Stmt>,]],
    [Return, new_return, [keyword: Token, value: Option<Expr>,]],
    [Function, new_function, [name: Token, params: Vec<Token>, body: Box<Stmt>,]],
    [While, new_while, [condition: Expr, body: Box<Stmt>,]],
    [If, new_if, [condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>,]],
    [Var, new_var, [name: Token, initialiser: Option<Expr>,]],
    [Print, new_print, [expression: Expr,]],
    [Expression, new_expression, [expression: Expr,]],
}

// impl Stmt {
//     fn try_functionable(self) -> Result<FFunction, anyhow::Error> {
//         match self {
//             Stmt::Function { name, params, body } => Ok(FFunction::new(name, params, body)),
//             _ => Err(anyhow::Error::msg("Hi")),
//         }
//     }
// }
