use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub ln: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    Number(f32),
    Degree(f32),
    String(String),
    Boolean(bool),
    Null,
}

impl Object {
    pub fn dtype(&self) -> &str {
        match self {
            Object::Number(_) => "Number",
            Object::Degree(_) => "Degree",
            Object::String(_) => "String",
            Object::Boolean(_) => "Boolean",
            Object::Null => "Null",
        }
    }

    pub fn coerce(&self) -> Self {
        match self {
            Object::Number(x) => Object::Number(*x),
            Object::Degree(x) => Object::Number(*x),
            Object::String(_) => Object::Number(1.),
            Object::Boolean(x) => Object::Number(if *x { 1. } else { 0. }),
            Object::Null => Object::Number(0.),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(value) => write!(f, "{value}"),
            Object::Degree(value) => write!(f, "{value}deg"),
            Object::String(value) => write!(f, "{value}"),
            Object::Boolean(value) => write!(f, "{value}"),
            Object::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single character tokens
    /// `(` token.
    LParen,
    /// `)` token.
    RParen,
    /// `{` token.
    LBrace,
    /// `}` token.
    RBrace,
    /// `,` token.
    Comma,
    /// `.` token.
    Dot,
    /// `=` token.
    Equals,

    /// `-` token.
    Minus,
    /// `+` token.
    Plus,
    /// `;` token.
    Semicolon,
    /// `/` token.
    Slash,
    /// `//` token.
    Slash2,
    /// `*` token.
    Ast,
    /// `**` token.
    Ast2,
    /// `%` token.
    Percent,
    /// `<<` token.
    LAngle2,
    /// `>>` token.
    RAngle2,
    /// `&` token.
    Amp,
    /// `|` token.
    Bar,
    /// `^` token.
    Hat,
    /// `~` token.
    Tilde,

    /// `!=` token.
    BangEquals,
    /// `==` token.
    Equals2,
    /// `===` token.
    Equals3,
    /// `<` token.
    LAngle,
    /// `>` token.
    RAngle,
    /// `<=` token.
    LAngleEquals,
    /// `>=` token.
    RAngleEquals,

    // Literals
    Identifier,
    String,
    Number,
    Degree,
    Colour,

    // Keywords
    Do,
    While,
    For,
    If,
    Else,
    Null,
    And,
    Or,
    True,
    False,
    Fun,
    Return,
    Class,
    This,
    Super,
    Var,
    Not,
    Print,

    // Built-in functions
    Num,
    Deg,

    // Last thing
    EOF,
}

impl Token {
    pub fn new(
        ttype: TokenType,
        lexeme: String,
        literal: Option<Object>,
        position: Position,
    ) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            position,
        }
    }

    pub fn eof(position: Position) -> Self {
        Token::new(TokenType::EOF, "".to_owned(), None, position)
    }
}

impl Position {
    pub fn new(ln: usize, col: usize) -> Position {
        Position { ln, col }
    }
}
