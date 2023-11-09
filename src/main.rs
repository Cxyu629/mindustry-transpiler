use mindustry_transpiler::{
    expr::*,
    run,
    token::{Object, Position, Token, TokenType as TT},
};

fn main() {
    let source = r#"3**--4**1-"#.to_owned();

    run(source.into_bytes());

    // Expr expression = new Expr.Binary(
    //     new Expr.Unary(
    //         new Token(TokenType.MINUS, "-", null, 1),
    //         new Expr.Literal(123)),
    //     new Token(TokenType.STAR, "*", null, 1),
    //     new Expr.Grouping(
    //         new Expr.Literal(45.67)));

    let ex = Unary::new(
        Token::new(TT::Minus, "-".to_owned(), None, Position::new(1, 1)),
        Literal {
            value: Object::Number(123.),
        },
    );

    let bin = Binary::new(
        Token::new(TT::Ast, "*".to_owned(), None, Position::new(1, 1)),
        ex,
        Grouping::new(Literal::new(Object::Degree(45.67))),
    );

    println!("{}", bin)
}
