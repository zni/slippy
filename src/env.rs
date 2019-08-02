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
        global.insert(String::from("+"), Expr::Builtin(builtins::add));
        global.insert(String::from("-"), Expr::Builtin(builtins::sub));
        global.insert(String::from("*"), Expr::Builtin(builtins::mul));
        global.insert(String::from("list"), Expr::Builtin(builtins::list));
        global.insert(String::from("car"), Expr::Builtin(builtins::car));
        global.insert(String::from("cdr"), Expr::Builtin(builtins::cdr));
        global.insert(String::from("cons"), Expr::Builtin(builtins::cons));
        env.push(global);
        Env { env }
    }

    pub fn get(&self, key: String) -> Option<&Expr> {
        for env in self.env.iter().rev() {
            let result = env.get(&key);
            if result.is_some() { return result; }
        }

        None
    }

    pub fn insert(&mut self, key: String, value: Expr) {
        self.env[0].insert(key, value);
    }

    pub fn extend_env(&mut self, env: HashMap<String, Expr>) {
        self.env.push(env);
    }

    pub fn pop_env(&mut self) -> Option<HashMap<String, Expr>> {
        self.env.pop()
    }
}


