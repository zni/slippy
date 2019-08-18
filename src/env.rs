use std::collections::HashMap;
use crate::builtins;
use crate::ast::Expr;

#[derive(Clone, Debug, Default)]
pub struct Env {
    env: Vec<HashMap<String, Expr>>
}

impl Env {
    pub fn new() -> Env {
        let mut env = Vec::new();
        let mut global = HashMap::new();
        global.insert(String::from("="), Expr::Builtin(builtins::equal));
        global.insert(String::from("<"), Expr::Builtin(builtins::lt));
        global.insert(String::from("<="), Expr::Builtin(builtins::lte));
        global.insert(String::from(">"), Expr::Builtin(builtins::gt));
        global.insert(String::from(">="), Expr::Builtin(builtins::gte));
        global.insert(String::from("+"), Expr::Builtin(builtins::add));
        global.insert(String::from("-"), Expr::Builtin(builtins::sub));
        global.insert(String::from("*"), Expr::Builtin(builtins::mul));
        global.insert(String::from("list"), Expr::Builtin(builtins::list));
        global.insert(String::from("car"), Expr::Builtin(builtins::car));
        global.insert(String::from("cdr"), Expr::Builtin(builtins::cdr));
        global.insert(String::from("cons"), Expr::Builtin(builtins::cons));
        global.insert(String::from("append"), Expr::Builtin(builtins::append));
        global.insert(String::from("length"), Expr::Builtin(builtins::length));
        global.insert(String::from("reverse"), Expr::Builtin(builtins::reverse));
        global.insert(String::from("equal?"), Expr::Builtin(builtins::equalp));
        global.insert(String::from("list?"), Expr::Builtin(builtins::listp));
        global.insert(String::from("null?"), Expr::Builtin(builtins::nullp));
        global.insert(String::from("number?"), Expr::Builtin(builtins::numberp));
        global.insert(String::from("procedure?"), Expr::Builtin(builtins::procedurep));
        global.insert(String::from("symbol?"), Expr::Builtin(builtins::symbolp));
        global.insert(String::from("pair?"), Expr::Builtin(builtins::pairp));
        global.insert(String::from("apply"), Expr::Builtin(builtins::apply));
        global.insert(String::from("load"), Expr::Builtin(builtins::load));
        global.insert(String::from("read"), Expr::Builtin(builtins::read));
        global.insert(String::from("display"), Expr::Builtin(builtins::display));
        env.push(global);
        Env { env }
    }

    pub fn get(&self, key: &String) -> Option<Expr> {
        for env in self.env.iter().rev() {
            let result = env.get(key);
            match result {
                Some(result) => return Some(result.clone()),
                None => continue,
            };
        }

        None
    }

    pub fn set(&mut self, key: String, value: Expr) -> Result<(), &'static str> {
        for env in self.env.iter_mut().rev() {
            let result = env.get(&key);
            if result.is_none() { continue }

            env.insert(key, value);
            return Ok(());
        }

        Err("key not found")
    }

    pub fn insert(&mut self, key: String, value: Expr) {
        let len = self.env.len();
        self.env[len - 1].insert(key, value);
    }

    pub fn extend_env(&mut self) {
        self.env.push(HashMap::new());
    }

    pub fn pop_env(&mut self) -> Option<HashMap<String, Expr>> {
        self.env.pop()
    }
}


