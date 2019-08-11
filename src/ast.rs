use std::fmt;
use std::cmp::PartialEq;

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
    String,
    Identifier,
    Quote,
    EOF,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Float(f64),
    Number(i32),
    Bool(bool),
    String(String),
}

impl Literal {
    pub fn is_string(&self) -> bool {
        match self {
            Literal::String(_) => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            Literal::String(s) => Some(s.clone()),
            _ => None,
        }
    }
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
            },
            Literal::String(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Clone)]
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
        write!(f, "{:?}", self.ttype)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.ttype)
    }
}

#[derive(Clone)]
pub enum Expr {
    DottedPair(Vec<Expr>, Box<Expr>),
    List(Vec<Expr>),
    Lambda(Vec<Expr>, Vec<Expr>),
    Var(String),
    Literal(Literal),
    Builtin(fn(&[Expr], &mut Env) -> Result<Expr, &'static str>),
    Unspecified,
}

impl Expr {
    pub fn is_list(&self) -> bool {
        match self {
            Expr::List(_) => true,
            _ => false,
        }
    }

    pub fn to_vec(&self) -> Option<Vec<Expr>> {
        match self {
            Expr::List(l) => Some(l.clone()),
            _ => None,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Expr::Literal(_) => true,
            _ => false,
        }
    }

    pub fn to_literal(&self) -> Option<Literal> {
        match self {
            Expr::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    pub fn is_var(&self) -> bool {
        match self {
            Expr::Var(_) => true,
            _ => false,
        }
    }

    pub fn from_var(&self) -> Option<String> {
        match self {
            Expr::Var(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn is_dotted_pair(&self) -> bool {
        match self {
            Expr::DottedPair(_, _) => true,
            _ => false,
        }
    }

    pub fn from_dotted_pair(&self) -> Option<(Vec<Expr>, Box<Expr>)> {
        match self {
            Expr::DottedPair(car, cdr) => Some((car.clone(), cdr.clone())),
            _ => None,
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        if self.is_literal() && other.is_literal() {
            let lval = self.to_literal().unwrap();
            let rval = other.to_literal().unwrap();
            return lval == rval;
        } else if self.is_var() && other.is_var() {
            let lval = self.from_var().unwrap();
            let rval = other.from_var().unwrap();
            return lval == rval;
        } else if self.is_list() && other.is_list() {
            let lval = self.to_vec().unwrap();
            let rval = self.to_vec().unwrap();

            if lval.len() != rval.len() {
                return false;
            }

            for (l, r) in lval.iter().zip(rval) {
                if l.eq(&r) {
                    continue
                } else {
                    return false;
                }
            }

            return true;
        } else if self.is_dotted_pair() && other.is_dotted_pair() {
            let (lcar, lcdr) = self.from_dotted_pair().unwrap();
            let (rcar, rcdr) = other.from_dotted_pair().unwrap();

            if lcar.len() != rcar.len() {
                return false;
            }

            if !lcdr.eq(&rcdr) {
                return false
            }

            for (l, r) in lcar.iter().zip(rcar) {
                if l.eq(&r) {
                    continue
                } else {
                    return false;
                }
            }

            return true;
        }

        return false;
    }
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
                write!(f, " . {})", rexpr)
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
            Expr::Lambda(_, _) => {
                write!(f, "#<procedure>")
            },
            Expr::Var(t) => write!(f, "{}", t),
            Expr::Literal(t) => write!(f, "{}", t),
            Expr::Builtin(_) => {
                write!(f, "#<built-in procedure>")
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
                write!(f, " . {})", rexpr)
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
            Expr::Lambda(_, _) => {
                write!(f, "#<procedure>")
            },
            Expr::Var(t) => write!(f, "{}", t),
            Expr::Literal(t) => write!(f, "{}", t),
            Expr::Builtin(_) => {
                write!(f, "#<built-in procedure>")
            },
            Expr::Unspecified => write!(f, "#unspecified"),
        }
    }
}
