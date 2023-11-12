use crate::token::{Object, Token};

#[macro_use]
mod macros {
    macro_rules! expr {
        {$([$name: ident, $func: ident, [$($field: tt: $t: ty, )+]], )+} => {
            #[derive(Clone, PartialEq)]
            pub enum Expr {
                $($name {
                    $(
                        $field: $t,
                    )+
                },)+
            }

            impl Expr {
                $(
                    pub fn $func($($field: $t, )+) -> Expr {
                        Expr::$name {
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

expr! {
    [Unary, new_unary, [operator: Token, right: Box<Expr>,]],
    [Binary, new_binary, [operator: Token, left: Box<Expr>, right: Box<Expr>,]],
    [Call, new_call, [callee: Box<Expr>, paren: Token, arguments: Vec<Expr>,]],
    [Grouping, new_grouping, [expression: Box<Expr>,]],
    [Literal, new_literal, [value: Object,]],
    [Logical, new_logical, [operator: Token, left: Box<Expr>, right: Box<Expr>,]],
    [Variable, new_variable, [name: Token,]],
    [Assign, new_assign, [name: Token, value: Box<Expr>,]],
}
