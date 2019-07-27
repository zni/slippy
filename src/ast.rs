#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Number,
    Float,
    Identifier,
    Lambda,
    If,
    Cond,
    Quote,
    Begin,
    Set,
    EOF,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Float(f64),
    Number(i32),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(ttype: TokenType,
               lexeme: String,
               line: u32,
               literal: Option<Literal>) -> Token {
        Token { ttype, lexeme, line, literal }
    }
}

#[derive(Debug)]
pub enum Expr {
    DottedPair(Vec<Expr>, Box<Expr>),
    List(Vec<Expr>),
    Lambda(Vec<Expr>, Box<Expr>),
    Var(Token),
    Literal(Token),
}
