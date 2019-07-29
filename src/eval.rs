use std::collections::HashMap;
use crate::ast::{Expr, Literal};

#[derive(Debug)]
pub enum EvalVal {
    Nil,
    Number(i32),
    Float(f64),
    Bool(bool),
    ProcVal(Expr),
    ListVal(Expr),
}

pub struct Eval {
    pub env: HashMap<String, Expr>,
}

impl Eval {
    pub fn new() -> Eval {
        Eval {
            env: HashMap::new()
        }
    }

    pub fn eval(&mut self, program: Expr) -> Result<Expr, &'static str> {
        match program {
            Expr::List(list) => {
                self.eval_list(list)
            },
            Expr::Var(t) => {
                let var = self.env.get(&t);
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

    fn eval_list(&mut self, form: Vec<Expr>) -> Result<Expr, &'static str>{
        if form.is_empty() {
            return Ok(Expr::List(Vec::new()));
        }

        let head = &form[0];
        match head {
            Expr::Var(t) => {
                match t.as_str() {
                    "define" => {
                        return self.define(form);
                    },
                    "+" => {
                        return self.add(form);
                    },
                    "lambda" => {
                        return self.lambda(form);
                    }
                    _ => {
                        let var = self.env.get(&t.clone());
                        match var {
                            Some(val) => self.apply(val.clone(), &form[1..form.len()]),
                            None => {
                                return Err("undefined");
                            }
                        }
                    }
                }
            },
            Expr::Literal(l) => {
                return Err("literal is not applicable");
            },
            _ => Err("not implemented"),
        }
    }

    fn define(&mut self, form: Vec<Expr>) -> Result<Expr, &'static str> {
        if form.len() != 3 {
            return Err("invalid define form");
        }

        let var = &form[1];
        if let Expr::Var(t) = var {
            let val = self.eval(form[2].clone());
            if val.is_err() {
                return Err(val.unwrap_err());
            }
            self.env.insert(t.clone(), val.unwrap());
            Ok(Expr::Var(t.to_string()))
        } else {
            Err("define second argument must be a variable")
        }
    }

    fn add(&mut self, form: Vec<Expr>) -> Result<Expr, &'static str> {
        let mut result = 0.0;
        for n in form.iter().skip(1) {
            let val = self.eval(n.clone());
            if val.is_err() { return val }

            match val.unwrap() {
                Expr::Literal(Literal::Number(d)) => result += d as f64,
                Expr::Literal(Literal::Float(d)) => result += d,
                _ => {
                    return Err("incorrect type for addition");
                }
            }
        }

        Ok(Expr::Literal(Literal::Float(result)))
    }

    fn lambda(&mut self, form: Vec<Expr>) -> Result<Expr, &'static str> {
        if form.len() != 3 {
            return Err("invalid lambda form");
        }

        let body = form[2].clone();
        match &form[1] {
            Expr::List(args) => {
                Ok(Expr::Lambda(args.clone(), Box::new(body)))
            },
            Expr::Var(v) => {
                Ok(Expr::Lambda(vec![Expr::Var(v.to_string())], Box::new(body)))
            },
            _ => {
                Err("invalid lambda arguments")
            }
        }

    }

    fn apply(&mut self, form: Expr, args: &[Expr]) -> Result<Expr, &'static str> {
        let mut env = HashMap::new();

        match form {
            Expr::Lambda(cargs, closure) => {
                for (c, a)in cargs.iter().zip(args) {
                    let result = self.eval(a.clone());
                    if result.is_err() { return result }
                    if let Expr::Var(v) = c {
                        env.insert(v.clone(), result.unwrap());
                    }
                }

                let prev_env = self.env.clone();
                self.env = env;
                let result = self.eval(*closure);
                self.env = prev_env;
                return result;
            },
            _ => Err("expected a lambda expression"),
        }
    }
}
