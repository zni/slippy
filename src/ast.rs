use std::fmt;

use crate::env::Env;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Bool,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Literal {
    Float(f64),
    Number(i32),
    Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Float(d) => write!(f, "{}", d),
            Literal::Number(d) => write!(f, "{}", d),
            Literal::Bool(b) => {
                if *b {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
        }
    }
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

#[derive(Clone)]
pub enum Expr {
    DottedPair(Vec<Expr>, Box<Expr>),
    List(Vec<Expr>),
    Lambda(Vec<Expr>, Vec<Expr>, Env),
    Var(String),
    Literal(Literal),
    Quote(Box<Expr>),
    Builtin(fn(&[Expr], &mut Env) -> Result<Expr, &'static str>),
    Unspecified,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::DottedPair(lexpr, rexpr) => {
                write!(f, "(").unwrap();
                for (i, l) in lexpr.iter().enumerate() {
                    if i < lexpr.len() - 1 {
                        write!(f, "{} ", l).unwrap();
                    } else {
                        write!(f, "{}", l).unwrap();
                    }
                }
                write!(f, ". {})", rexpr)
            },
            Expr::List(lexpr) => {
                write!(f, "(").unwrap();
                for (i, l) in lexpr.iter().enumerate() {
                    if i < lexpr.len() - 1 {
                        write!(f, "{} ", l).unwrap();
                    } else {
                        write!(f, "{}", l).unwrap();
                    }
                }
                write!(f, ")")
            },
            Expr::Lambda(_, _, _) => {
                write!(f, "<procedure>")
            },
            Expr::Var(t) => write!(f, "{}", t),
            Expr::Literal(t) => write!(f, "{}", t),
            Expr::Quote(t) => write!(f, "(quote {})", t),
            Expr::Builtin(_) => {
                write!(f, "<built-in procedure>")
            },
            Expr::Unspecified => write!(f, "#unspecified"),
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::DottedPair(lexpr, rexpr) => {
                write!(f, "(").unwrap();
                for (i, l) in lexpr.iter().enumerate() {
                    if i < lexpr.len() - 1 {
                        write!(f, "{} ", l).unwrap();
                    } else {
                        write!(f, "{}", l).unwrap();
                    }
                }
                write!(f, ". {})", rexpr)
            },
            Expr::List(lexpr) => {
                write!(f, "(").unwrap();
                for (i, l) in lexpr.iter().enumerate() {
                    if i < lexpr.len() - 1 {
                        write!(f, "{} ", l).unwrap();
                    } else {
                        write!(f, "{}", l).unwrap();
                    }
                }
                write!(f, ")")
            },
            Expr::Lambda(_, _, _) => {
                write!(f, "<procedure>")
            },
            Expr::Var(t) => write!(f, "{}", t),
            Expr::Literal(t) => write!(f, "{}", t),
            Expr::Quote(t) => write!(f, "(quote {})", t),
            Expr::Builtin(_) => {
                write!(f, "<built-in procedure>")
            },
            Expr::Unspecified => write!(f, "#unspecified"),
        }
    }
}
