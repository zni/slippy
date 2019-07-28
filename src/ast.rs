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
    Quote(Box<Expr>),
    App(Box<Expr>, Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::DottedPair(lexpr, rexpr) => {
                write!(f, "(").unwrap();
                for l in lexpr {
                    write!(f, " {} ", l).unwrap();
                }
                write!(f, ". {})", rexpr)
            },
            Expr::List(lexpr) => {
                write!(f, "(").unwrap();
                for l in lexpr {
                    write!(f, " {} ", l).unwrap();
                }
                write!(f, ")")
            },
            Expr::Lambda(vars, body) => {
                write!(f, "(lambda (").unwrap();
                for v in vars {
                    write!(f, " {} ", v).unwrap();
                }
                write!(f, ") ").unwrap();
                write!(f, "{})", body)
            },
            Expr::Var(t) => write!(f, "{}", t.lexeme),
            Expr::Literal(t) => write!(f, "{}", t.lexeme),
            Expr::Quote(t) => write!(f, "(quote {})", t),
            Expr::App(e, op) => write!(f, "(app {} {:?})", e, op),
        }
    }
}
