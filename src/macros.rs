#[macro_export]
macro_rules! expr_like {
    {$x:ident} => {
        impl ExprLike for $x {}

        impl IntoExpr for $x {
            fn into_expr(self) -> Expr {
                Expr(Rc::new(self))
            }
        }
    };
}
