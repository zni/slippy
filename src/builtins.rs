use crate::eval::eval;
use crate::env::Env;
use crate::ast::{Expr, Literal};

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
