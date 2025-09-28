use std::ops::Neg;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use std::cell::RefCell;
use std::rc::Rc;

use crate::eval;
use crate::eval::eval;
use crate::env::Env;
use crate::ast::{Expr, Literal};
use crate::lexer::Lexer;
use crate::parser::Parser;

/*
 * Numerical built-ins
 */

pub fn equal(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let previous = match &list[0] {
        Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
        Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
        _ => return Err("must be a number"),
    };

    for val in list.iter().skip(1) {
        let current = match val {
            Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
            Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
            _ => return Err("must be a number"),
        };

        if previous != current {
            return Ok(Expr::Literal(Literal::Bool(false)))
        }
    }
    Ok(Expr::Literal(Literal::Bool(true)))
}

pub fn lt(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let previous = match &list[0] {
        Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
        Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
        _ => return Err("must be a number"),
    };

    for val in list.iter().skip(1) {
        let current = match val {
            Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
            Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
            _ => return Err("must be a number"),
        };

        if !(previous < current) {
            return Ok(Expr::Literal(Literal::Bool(false)))
        }
    }
    Ok(Expr::Literal(Literal::Bool(true)))
}

pub fn lte(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let previous = match &list[0] {
        Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
        Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
        _ => return Err("must be a number"),
    };

    for val in list.iter().skip(1) {
        let current = match val {
            Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
            Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
            _ => return Err("must be a number"),
        };

        if !(previous <= current) {
            return Ok(Expr::Literal(Literal::Bool(false)))
        }
    }
    Ok(Expr::Literal(Literal::Bool(true)))
}

pub fn gt(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let previous = match &list[0] {
        Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
        Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
        _ => return Err("must be a number"),
    };

    for val in list.iter().skip(1) {
        let current = match val {
            Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
            Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
            _ => return Err("must be a number"),
        };

        if !(previous > current) {
            return Ok(Expr::Literal(Literal::Bool(false)))
        }
    }
    Ok(Expr::Literal(Literal::Bool(true)))
}

pub fn gte(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let previous = match &list[0] {
        Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
        Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
        _ => return Err("must be a number"),
    };

    for val in list.iter().skip(1) {
        let current = match val {
            Expr::Literal(Literal::Number(n)) => Literal::Number(*n),
            Expr::Literal(Literal::Float(f)) => Literal::Float(*f),
            _ => return Err("must be a number"),
        };

        if !(previous >= current) {
            return Ok(Expr::Literal(Literal::Bool(false)))
        }
    }
    Ok(Expr::Literal(Literal::Bool(true)))
}

pub fn add(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let mut result: i32 = 0;
    for val in list.iter() {
        let val = eval(val, env.clone());
        if val.is_err() { return val }
        let val = val.unwrap();
        match val {
            Expr::Literal(Literal::Number(n)) => result += n,
            Expr::Literal(Literal::Float(f)) => result += f as i32,
            _ => return Err("cannot add a non-number"),
        }
    }

    Ok(Expr::Literal(Literal::Number(result)))
}

pub fn mul(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let mut result: i32 = 1;
    for val in list.iter() {
        let val = eval(val, env.clone());
        if val.is_err() { return val }
        let val = val.unwrap();
        match val {
            Expr::Literal(Literal::Number(n)) => result *= n,
            Expr::Literal(Literal::Float(f)) => result *= f as i32,
            _ => return Err("cannot multiply a non-number"),
        }
    }

    Ok(Expr::Literal(Literal::Number(result)))
}

pub fn sub(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.is_empty() { return Err("- requires at least one argument") }

    let val = &list[0];
    let val = eval(val, env.clone());
    if val.is_err() { return val }
    let val = val.unwrap();
    let mut result = match val {
        Expr::Literal(Literal::Number(n)) => n,
        Expr::Literal(Literal::Float(f)) => f as i32,
        _ => return Err("cannot subtract a non-number"),
    };

    if list.len() == 1 {
        return Ok(Expr::Literal(Literal::Number(result.neg())))
    }

    for val in list.iter().skip(1) {
        let val = eval(val, env.clone());
        if val.is_err() { return val }
        let val = val.unwrap();
        match val {
            Expr::Literal(Literal::Number(n)) => result -= n,
            Expr::Literal(Literal::Float(f)) => result -= f as i32,
            _ => return Err("cannot subtract a non-number"),
        }
    }

    Ok(Expr::Literal(Literal::Number(result)))
}


/*
 * List built-ins
 */

pub fn list(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let mut new_list = Vec::new();
    for val in list.iter() { new_list.push(val.clone()); }

    Ok(Expr::List(new_list))
}

pub fn car(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    let val = &list[0];
    match val {
        Expr::List(l) => {
            if l.is_empty() { return Err("called with incorrect type ()") }
            Ok(l[0].clone())
        },
        Expr::DottedPair(l, _) => {
            Ok(l[0].clone())
        },
        _ => Err("called with incorrect type")
    }
}

pub fn cdr(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    let val = &list[0];
    match val {
        Expr::List(l) => {
            if l.is_empty() { return Err("called with incorrect type ()") }
            Ok(Expr::List(l[1..l.len()].to_vec()))
        },
        Expr::DottedPair(l, r) => {
            let l = l[1..l.len()].to_vec();
            if l.is_empty() {
                Ok(*r.clone())
            } else {
                Ok(Expr::DottedPair(l, Box::new(*r.clone())))
            }
        },
        _ => Err("called with incorrect type")
    }
}

pub fn cons(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 2 { return Err("called with incorrect number of arguments") }

    let val = &list[0];
    let cell = &list[1];
    match cell {
        Expr::List(l) => {
            let mut cell = l.clone();
            cell.insert(0, val.clone());
            Ok(Expr::List(cell))
        },
        Expr::DottedPair(l, end) => {
            let mut l = l.clone();
            l.insert(0, val.clone());
            Ok(Expr::DottedPair(l.to_vec(), end.clone()))
        },
        _ => {
            Ok(Expr::DottedPair(vec![val.clone()], Box::new(cell.clone())))
        }
    }
}

pub fn append(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() < 2 { return Err("called with incorrect number of arguments") }

    let listval = &list[0];
    match listval {
        Expr::List(val) => {
            let mut val = val.clone();
            for n in list.iter().skip(1) {
                if let Expr::List(l) = n {
                    val.append(&mut l.clone());
                } else {
                    return Err("argument must be a list");
                }
            }
            Ok(Expr::List(val.to_vec()))
        },
        _ => Err("incorrect type passed to append")
    }
}

pub fn length(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    let listval = &list[0];
    if !listval.is_list() {
        return Err("length called with incorrect type")
    }

    let size = listval.to_vec().unwrap().len() as i32;
    Ok(Expr::Literal(Literal::Number(size)))
}

pub fn reverse(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    let listval = &list[0];
    if !listval.is_list() {
        return Err("reverse called with incorrect type")
    }

    let listval = listval.to_vec().unwrap();
    let mut revlist = Vec::new();
    for n in listval.iter().rev() {
        revlist.push(n.clone())
    }
    Ok(Expr::List(revlist))
}


/*
 * Tests
 */

pub fn equalp(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 2 { return Err("called with incorrect number of arguments") }

    let lval = &list[0];
    let rval = &list[1];

    Ok(Expr::Literal(Literal::Bool(lval == rval)))
}

pub fn listp(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    match &list[0] {
        Expr::List(_) => Ok(Expr::Literal(Literal::Bool(true))),
        _             => Ok(Expr::Literal(Literal::Bool(false))),
    }
}

pub fn nullp(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    match &list[0] {
        Expr::List(l) => {
            if l.is_empty() {
                Ok(Expr::Literal(Literal::Bool(true)))
            } else {
                Ok(Expr::Literal(Literal::Bool(false)))
            }
        },
        _ => Ok(Expr::Literal(Literal::Bool(false))),
    }
}

pub fn procedurep(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    match &list[0] {
        Expr::Lambda(_, _) => {
            Ok(Expr::Literal(Literal::Bool(true)))
        },
        Expr::Builtin(_) => {
            Ok(Expr::Literal(Literal::Bool(true)))
        },
        _ => Ok(Expr::Literal(Literal::Bool(false))),
    }
}

pub fn numberp(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    match &list[0] {
        Expr::Literal(Literal::Number(_)) => Ok(Expr::Literal(Literal::Bool(true))),
        Expr::Literal(Literal::Float(_)) => Ok(Expr::Literal(Literal::Bool(true))),
        _ => Ok(Expr::Literal(Literal::Bool(false))),
    }
}

pub fn symbolp(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    match &list[0] {
        Expr::Var(_) => Ok(Expr::Literal(Literal::Bool(true))),
        _ => Ok(Expr::Literal(Literal::Bool(false))),
    }
}

pub fn pairp(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    match &list[0] {
        Expr::DottedPair(_, _) => Ok(Expr::Literal(Literal::Bool(true))),
        Expr::List(l) => {
            if l.is_empty() {
                Ok(Expr::Literal(Literal::Bool(false)))
            } else {
                Ok(Expr::Literal(Literal::Bool(true)))
            }
        },
        _ => Ok(Expr::Literal(Literal::Bool(false))),
    }
}


/*
 * Other
 */

pub fn apply(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() < 2 { return Err("called with incorrect number of arguments") }

    let proc = &list[0];
    let objs = &list[1..list.len() - 1];
    let arg = &list[list.len() - 1];

    let mut objlist = Vec::new();
    for obj in objs.iter() {
        objlist.push(obj.clone());
    }

    if let Expr::List(l) = arg {
        objlist.append(&mut l.clone());
        eval::apply(proc, objlist, env)
    } else {
        Err("apply expecting a list")
    }
}

pub fn load(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    let val = &list[0];
    if !val.is_literal() {
        return Err("load called with incorrect type");
    }

    let val = val.to_literal().unwrap();
    if !val.is_string() {
        return Err("load called with incorrect type");
    }

    let file = val.to_string().unwrap();

    let path = Path::new(&file);
    let mut file = File::open(&path)
        .expect("Failed to open file");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read file");

    let mut lexer = Lexer::new(&source);
    lexer.scan();
    let mut parser = Parser::new(lexer.tokens);
    let result = parser.parse();
    if result.is_ok() {
        let exprs = result.unwrap();
        for expr in exprs.iter() {
            let eval_result = eval(expr, env.clone());
            if eval_result.is_err() {
                println!("{}", eval_result.unwrap_err());
            }
        }
    } else {
        println!("{}", result.unwrap_err());
        println!("pos: {}", parser.current);
    }

    Ok(Expr::Unspecified)
}

pub fn read(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 0 { return Err("input ports are not yet supported for read") }

    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line.");

    let line = format!("'{}", line.trim());
    let mut lexer = Lexer::new(&line);
    lexer.scan();
    let mut parser = Parser::new(lexer.tokens);
    let result = parser.parse();
    if result.is_ok() {
        let exprs = result.unwrap();
        for expr in exprs.iter() {
            return eval(expr, env);
        }
    } else {
        println!("{}", result.unwrap_err());
        println!("pos: {}", parser.current);
    }

    Err("read error")
}

pub fn display(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("output ports are not yet supported for display") }

    let expr = &list[0];
    println!("{}", expr);

    Ok(Expr::Unspecified)
}
