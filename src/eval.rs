use crate::ast::{Expr, Literal};
use crate::env::Env;


pub fn eval(program: Expr, env: &mut Env) -> Result<Expr, &'static str> {
    match program {
        Expr::List(list) => {
            if list.is_empty() {
                return Ok(Expr::List(list))
            }

            let head = &list[0];
            match head {
                Expr::Var(atom) => {
                    match atom.as_str() {
                        "lambda" => lambda(&list, env),
                        "define" => define(&list, env),
                        "if"     => ifexpr(&list, env),
                        _ => {
                            let var = env.get(atom.to_string());
                            if var.is_none() { return Err("undefined variable"); }
                            let proc = var.unwrap();

                            let mut args = Vec::new();
                            for op in &list[1..list.len()] {
                                let result = eval(op.clone(), &mut env.clone());
                                if result.is_err() { return result; }

                                args.push(result.unwrap());
                            }

                            apply(proc.clone(), args, env)
                        },
                    }
                },
                Expr::Literal(_) => {
                    return Err("not applicable")
                },
                Expr::List(_) => {
                    let proc = eval(head.clone(), env);
                    if proc.is_err() { return proc }

                    let mut args = Vec::new();
                    for op in &list[1..list.len()] {
                        let result = eval(op.clone(), env);
                        if result.is_err() { return result; }

                        args.push(result.unwrap());
                    }

                    apply(proc.unwrap(), args, env)
                }
                _ => return Err("not implemented")
            }
        },

        Expr::Var(atom) => {
            let var = env.get(atom);
            match var {
                Some(val) => return Ok(val.clone()),
                None => {
                    return Err("undefined variable");
                },
            }
        },

        Expr::Literal(l) => Ok(Expr::Literal(l)),
        _ => Err("not implemented"),
    }
}

fn lambda(list: &Vec<Expr>, env: &Env) -> Result<Expr, &'static str> {
    if let Expr::List(args) = &list[1] {
        let body = &list[2..list.len()];

        Ok(Expr::Lambda(args.to_vec(), body.to_vec(), env.clone()))
    } else {
        Err("not implemented")
    }
}

fn define(list: &Vec<Expr>, env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() != 3 { return Err("invalid define statement"); }

    let atom = &list[1];
    let atom = from_var(atom.clone());
    if atom.is_none() { return Err("first argument to define must be an atom"); }
    let atom = atom.unwrap();

    let val = &list[2];
    let val = eval(val.clone(), env);
    if val.is_err() { return val; }
    let val = val.unwrap();
    if is_nil(&val) { return Err("cannot be used as an expression") }

    env.insert(atom, val.clone());
    Ok(Expr::Nil)
}

fn ifexpr(list: &Vec<Expr>, env: &mut Env) -> Result<Expr, &'static str> {
    Err("not implemented")
}

fn apply(proc: Expr, args: Vec<Expr>, env: &mut Env) -> Result<Expr, &'static str> {
    match proc {
        Expr::Lambda(parms, body, mut proc_env) => {
            for (p, a) in parms.iter().zip(args) {
                proc_env.insert(from_var(p.clone()).unwrap(), a);
            }

            let mut result = Ok(Expr::List(vec![]));
            for expr in body {
                result = eval(expr, &mut proc_env);
            }

            return result;
        },
        Expr::Builtin(builtin) => {
            builtin(&args, env)
        }
        _ => Err("unable to apply"),
    }
}

fn from_var(var: Expr) -> Option<String> {
   if let Expr::Var(atom) = var {
       Some(atom)
   } else {
       None
   }
}

fn is_nil(var: &Expr) -> bool {
    if let Expr::Nil = var {
        return true;
    } else {
        return false;
    }
}
