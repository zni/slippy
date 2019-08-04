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
                        "quote"  => quote(&list, env),
                        "set!"   => set(&list, env),
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
                    Err("not applicable")
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
                _ => Err("not implemented")
            }
        },

        Expr::Var(atom) => {
            let var = env.get(atom);
            match var {
                Some(val) => Ok(val.clone()),
                None => {
                    Err("undefined variable")
                },
            }
        },

        Expr::Literal(l) => Ok(Expr::Literal(l)),
        _ => Err("not implemented"),
    }
}

fn lambda(list: &[Expr], env: &Env) -> Result<Expr, &'static str> {
    if let Expr::List(args) = &list[1] {
        let body = &list[2..list.len()];

        Ok(Expr::Lambda(args.to_vec(), body.to_vec(), env.clone()))
    } else {
        Err("not implemented")
    }
}

fn define(list: &[Expr], env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() < 3 { return Err("invalid define statement"); }

    match &list[1] {
        Expr::List(vars) => {
            if vars.is_empty() { return Err("define vars cannot be empty") }

            let args = if vars.len() == 1 {
                Vec::new()
            } else {
                vars[1..vars.len()].to_vec()
            };

            env.insert(
                vars[0].to_string(),
                Expr::Lambda(args, list[2..list.len()].to_vec(), env.clone())
            );

            Ok(Expr::Unspecified)
        },
        Expr::Var(atom) => {
            let val = &list[2];
            let val = eval(val.clone(), env);
            if val.is_err() { return val; }
            let val = val.unwrap();
            if is_unspecified(&val) {
                return Err("unspecified value cannot be used as an expression")
            }

            env.insert(atom.to_string(), val.clone());

            Ok(Expr::Unspecified)
        },
        _ => Err("invalid define statement")
    }
}

fn ifexpr(list: &[Expr], env: &mut Env) -> Result<Expr, &'static str> {
    let test = eval(list[1].clone(), env);
    if test.is_err() { return test }
    let test = test.unwrap();

    if let Expr::Literal(Literal::Bool(false)) = test {
        if list.len() == 3 { return Ok(Expr::Unspecified) }

        let alternate = &list[3];
        return eval(alternate.clone(), env);
    } else {
        let consequent = &list[2];
        return eval(consequent.clone(), env);
    }
}

fn quote(list: &[Expr], _env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() != 2 { return Err("invalid quote syntax") }
    Ok(list[1].clone())
}

fn set(list: &[Expr], env: &mut Env) -> Result<Expr, &'static str> {
    if list.len() != 3 { return Err("invalid set syntax") }

    let var = &list[1];
    match var {
        Expr::Var(atom) => {
            let val = &list[2];
            let val = eval(val.clone(), env);
            if val.is_err() { return val; }
            let val = val.unwrap();

            let result = env.set(atom.to_string(), val);
            if result.is_ok() {
                Ok(Expr::Unspecified)
            } else {
                Err("variable is not bound")
            }
        },
        _ => Err("first parameter must be an atom"),
    }
}

fn apply(proc: Expr, args: Vec<Expr>, env: &mut Env) -> Result<Expr, &'static str> {
    match proc {
        Expr::Lambda(parms, body, _) => {
            if parms.len() != args.len() { return Err("applied to incorrect number of args") }

            env.extend_env();
            for (p, a) in parms.iter().zip(args) {
                env.insert(from_var(p).unwrap(), a);
            }

            let mut result = Ok(Expr::Unspecified);
            for expr in body {
                result = eval(expr, env);
            }
            env.pop_env();

            result
        },
        Expr::Builtin(builtin) => {
            builtin(&args, env)
        }
        _ => Err("unable to apply"),
    }
}

fn from_var(var: &Expr) -> Option<String> {
   if let Expr::Var(atom) = var {
       Some(atom.to_string())
   } else {
       None
   }
}

fn is_unspecified(var: &Expr) -> bool {
    if let Expr::Unspecified = var {
        return true;
    } else {
        return false;
    }
}
