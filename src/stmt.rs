use crate::{expr::Expr, token::Token};

#[macro_use]
mod macros {
    macro_rules! stmt {
        {$([$name: ident, $func: ident, [$($field: tt: $t: ty, )+]], )+} => {
            #[derive(Clone, PartialEq)]
            pub enum Stmt {
                $($name {
                    $(
                        $field: $t,
                    )+
                },)+
            }

            impl Stmt {
                $(
                    pub fn $func($($field: $t, )+) -> Stmt {
                        Stmt::$name {
                            $(
                                $field,
                            )+
                        }
                    }
                )+
            }
        };
    }
}

stmt! {
    [Block, new_block, [statements: Vec<Stmt>,]],
    // [Function, new_function, [name: Token, params: Vec<Token>, body: Box<Stmt>,]],
    [DoWhile, new_do_while, [condition: Expr, body: Box<Stmt>,]],
    [While, new_while, [condition: Expr, body: Box<Stmt>,]],
    [If, new_if, [condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>,]],
    [Var, new_var, [name: Token, initialiser: Option<Expr>,]],
    [Print, new_print, [expression: Expr,]],
    [Expression, new_expression, [expression: Expr,]],
}
