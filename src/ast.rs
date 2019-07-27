use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::DottedPair(lexpr, rexpr) => write!(f, "({:?} . {})", lexpr, rexpr),
            Expr::List(lexpr) => write!(f, "({:?})", lexpr),
            Expr::Lambda(vars, body) => write!(f, "(lambda ({:?}) {})", vars, body),
            Expr::Var(t) => write!(f, "{}", t.lexeme),
            Expr::Literal(t) => write!(f, "{}", t.lexeme),
        }
    }
}
