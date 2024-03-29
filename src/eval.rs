use crate::ast::{Expr, Literal};
use crate::env::Env;

use std::cell::RefCell;
use std::rc::Rc;


pub fn eval(program: &Expr, env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    match program {
        Expr::List(list) => {
            if list.is_empty() {
                return Ok(Expr::List(list.to_vec()))
            }

            let head = &list[0];
            match head {
                Expr::Var(atom) => {
                    match atom.as_str() {
                        "lambda" => lambda(&list, env),
                        "define" => define(&list, env),
                        "if"     => ifexpr(&list, env),
                        "quote"  => quote(&list, env),
                        "set!"   => set(&list, env),
                        "begin"  => begin(&list, env),
                        "let"    => let_(&list, env),
                        "cond"   => cond(&list, env),
                        _ => {
                            let mut args = Vec::new();
                            for op in &list[1..list.len()] {
                                let result = eval(op, env.clone());
                                if result.is_err() { return result; }

                                args.push(result.unwrap());
                            }

                            let var = env.borrow().get(&atom);
                            if var.is_none() { return Err("undefined variable"); }
                            let proc = var.unwrap();
                            apply(&proc, args, env)
                        },
                    }
                },
                Expr::Literal(_) => {
                    Err("not applicable")
                },
                Expr::List(_) => {
                    let proc = eval(head, env.clone());
                    if proc.is_err() { return proc }

                    let mut args = Vec::new();
                    for op in &list[1..list.len()] {
                        let result = eval(op, env.clone());
                        if result.is_err() { return result; }

                        args.push(result.unwrap());
                    }

                    apply(&proc.unwrap(), args, env)
                }
                _ => Err("not implemented")
            }
        },

        Expr::Var(atom) => {
            let var = env.borrow().get(&atom);
            match var {
                Some(val) => Ok(val.clone()),
                None => {
                    Err("undefined variable")
                },
            }
        },

        Expr::Literal(l) => Ok(Expr::Literal(l.clone())),
        _ => Err("not implemented"),
    }
}

fn lambda(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if let Expr::List(args) = &list[1] {
        let body = &list[2..list.len()];

        Ok(Expr::Lambda(args.to_vec(), body.to_vec()))
    } else {
        Err("not implemented")
    }
}

fn define(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() < 3 { return Err("invalid define statement"); }

    match &list[1] {
        Expr::List(vars) => {
            if vars.is_empty() { return Err("define vars cannot be empty") }

            let args = if vars.len() == 1 {
                Vec::new()
            } else {
                vars[1..vars.len()].to_vec()
            };

            env.borrow_mut().insert(
                vars[0].to_string(),
                Expr::Lambda(args, list[2..list.len()].to_vec())
            );

            Ok(Expr::Unspecified)
        },
        Expr::Var(atom) => {
            let val = &list[2];
            let val = eval(val, env.clone());
            if val.is_err() { return val; }
            let val = val.unwrap();
            if val.is_unspecified() {
                return Err("unspecified value cannot be used as an expression")
            }

            env.borrow_mut().insert(atom.to_string(), val.clone());

            Ok(Expr::Unspecified)
        },
        _ => Err("invalid define statement")
    }
}

fn ifexpr(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let test = eval(&list[1], env.clone());
    if test.is_err() { return test }
    let test = test.unwrap();

    if let Expr::Literal(Literal::Bool(false)) = test {
        if list.len() == 3 { return Ok(Expr::Unspecified) }

        let alternate = &list[3];
        return eval(alternate, env);
    } else {
        let consequent = &list[2];
        return eval(consequent, env);
    }
}

fn quote(list: &[Expr], _env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 2 { return Err("invalid quote syntax") }
    Ok(list[1].clone())
}

fn set(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    if list.len() != 3 { return Err("invalid set syntax") }

    let var = &list[1];
    match var {
        Expr::Var(atom) => {
            let val = &list[2];
            let val = eval(val, env.clone());
            if val.is_err() { return val; }
            let val = val.unwrap();

            let result = env.borrow_mut().set(atom.to_string(), val);
            if result.is_ok() {
                Ok(Expr::Unspecified)
            } else {
                Err("variable is not bound")
            }
        },
        _ => Err("first parameter must be an atom"),
    }
}

fn begin(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let mut result = Ok(Expr::Unspecified);
    for expr in list.iter().skip(1) {
        result = eval(expr, env.clone());
    }

    result
}

fn let_(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let decs = &list[1];
    if !decs.is_list() { return Err("expecting list of declarations") }
    let decs = decs.to_vec().unwrap();
    let let_env = env.borrow_mut().extend_env(env.clone());
    for dec in decs.iter() {
        if !dec.is_list() { return Err("expecting a pair") }
        let dec = dec.to_vec().unwrap();
        let var = &dec[0];

        let val = &dec[1];
        let val = eval(val, env.clone());
        if val.is_err() { return val }
        let val = val.unwrap();

        if !var.is_var() { return Err("expecting an atom in let declaration pair") }
        let var = var.from_var().unwrap();
        let_env.borrow_mut().insert(var, val.clone());
    }

    let mut result = Ok(Expr::Unspecified);
    let exprs = &list[2..list.len()];
    for expr in exprs.iter() {
        result = eval(expr, let_env.clone());
    }

    result
}

fn cond(list: &[Expr], env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    let conditions = &list[1..list.len()];
    for condition in conditions.iter() {
        if !condition.is_list() { return Err("expecting a list in cond") }
        let list = condition.to_vec().unwrap();
        if list.len() != 2 { return Err("invalid format in cond") }

        let pred = &list[0];
        if pred.is_var() {
            let else_ = pred.from_var().unwrap();
            if else_ == "else" {
                return eval(&list[1], env.clone());
            } else {
                return Err("expecting else in cond");
            }
        }

        let pred_result = eval(pred, env.clone());
        if pred_result.is_err() { return pred_result }
        if pred_result.unwrap().is_true() {
            return eval(&list[1], env)
        }
    }

    Ok(Expr::Unspecified)
}

pub fn apply(proc: &Expr, args: Vec<Expr>, env: Rc<RefCell<Env>>) -> Result<Expr, &'static str> {
    match proc {
        Expr::Lambda(parms, body) => {
            if parms.len() != args.len() { return Err("applied to incorrect number of args") }

            let proc_env = env.borrow_mut().extend_env(env.clone());
            for (p, a) in parms.iter().zip(args) {
                proc_env.borrow_mut().insert(p.from_var().unwrap(), a);
            }

            let mut result = Ok(Expr::Unspecified);
            for expr in body {
                result = eval(&expr, proc_env.clone());
            }

            result
        },
        Expr::Builtin(builtin) => {
            builtin(&args, env)
        }
        _ => Err("unable to apply"),
    }
}
