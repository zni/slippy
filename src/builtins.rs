use std::ops::Neg;

use crate::eval::eval;
use crate::env::Env;
use crate::ast::{Expr, Literal};

/*
 * Numerical built-ins
 */

pub fn add(list: &[Expr], env: &mut Env) -> Result<Expr, &'static str> {
    let mut result: i32 = 0;
    for val in list.iter() {
        let val = eval(val.clone(), env);
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

pub fn mul(list: &[Expr], env: &mut Env) -> Result<Expr, &'static str> {
    let mut result: i32 = 1;
    for val in list.iter() {
        let val = eval(val.clone(), env);
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

pub fn sub(list: &[Expr], env: &mut Env) -> Result<Expr, &'static str> {
    if list.is_empty() { return Err("- requires at least one argument") }

    let val = &list[0];
    let val = eval(val.clone(), env);
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
        let val = eval(val.clone(), env);
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

pub fn list(list: &[Expr], env: &mut Env) -> Result<Expr, &'static str> {
    let mut new_list = Vec::new();
    for val in list.iter() {
        let val = eval(val.clone(), env);
        if val.is_err() { return val }
        let val = val.unwrap();

        new_list.push(val);
    }

    Ok(Expr::List(new_list))
}

pub fn car(list: &[Expr], _env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    let val = &list[0];
    match val {
        Expr::List(l) => {
            if l.is_empty() { return Err("called with incorrect type ()") }
            Ok(l[0].clone())
        },
        _ => Err("called with incorrect type")
    }
}

pub fn cdr(list: &[Expr], _env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    let val = &list[0];
    match val {
        Expr::List(l) => {
            if l.is_empty() { return Err("called with incorrect type ()") }
            Ok(Expr::List(l[1..l.len()].to_vec()))
        },
        _ => Err("called with incorrect type")
    }
}

pub fn cons(list: &[Expr], _env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() != 2 { return Err("called with incorrect number of arguments") }

    let val = &list[0];
    let cell = &list[1];
    match cell {
        Expr::List(l) => {
            let mut cell = l.clone();
            cell.insert(0, val.clone());
            Ok(Expr::List(cell))
        },
        _ => Err("called with incorrect type")
    }
}

pub fn listp(list: &[Expr], _env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() != 1 { return Err("called with incorrect number of arguments") }

    match &list[0] {
        Expr::List(_) => Ok(Expr::Literal(Literal::Bool(true))),
        _             => Ok(Expr::Literal(Literal::Bool(false))),
    }
}

pub fn nullp(list: &[Expr], _env: &mut Env) -> Result<Expr, &'static str> {
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
