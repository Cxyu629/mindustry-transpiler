macro_rules! binary {
    (($fun: tt, $prev_fun: tt, [h $x:expr$(, $y:expr)*])) => {
        fn $fun(&mut self) -> Result<Expr, ParseError> {
            // eprintln!("hi from {:?} {:?}", $x, self.current);
            let mut left = self.$prev_fun()?.clone();

            let mut valid = self.cond_advance(vec![$x$(, $y)*]);

            while let Some(operator) = valid {
                let right = self.$prev_fun()?.clone();
                left = BinaryExpr::new(operator.to_owned(), left, right).into_expr();
                valid = self.cond_advance(vec![$x$(, $y)*]);
            }
            Ok(left)
        }
    };
}
