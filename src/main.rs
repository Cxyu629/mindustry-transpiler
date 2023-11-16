use mindustry_transpiler::{run_file, run_prompt};

fn main() {
    // let source = r#"3**--1**-1;"#.to_owned();

    // run_prompt();

    let _ = run_file("source.txt");

    // Expr expression = new Expr.Binary(
    //     new Expr.Unary(
    //         new Token(TokenType.MINUS, "-", null, 1),
    //         new Expr.Literal(123)),
    //     new Token(TokenType.STAR, "*", null, 1),
    //     new Expr.Grouping(
    //         new Expr.Literal(45.67)));

    // let ex = UnaryExpr::new(
    //     Token::new(TT::Minus, "-".to_owned(), None, Position::new(1, 1)),
    //     LiteralExpr {
    //         value: Object::Number(123.),
    //     },
    // );

    // let bin = BinaryExpr::new(
    //     Token::new(TT::Ast, "*".to_owned(), None, Position::new(1, 1)),
    //     ex,
    //     GroupingExpr::new(LiteralExpr::new(Object::Degree(45.67))),
    // );

    // println!("{}", bin)
}
