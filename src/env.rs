use std::collections::HashMap;
use crate::builtins;
use crate::ast::Expr;

#[derive(Clone, Debug)]
pub struct Env {
    env: Vec<HashMap<String, Expr>>
}

impl Env {
    pub fn new() -> Env {
        let mut env = Vec::new();
        let mut global = HashMap::new();
        global.insert(String::from("+"), Expr::Builtin(builtins::add));
        env.push(global);
        Env { env }
    }

    pub fn get(&self, key: String) -> Option<&Expr> {
        for env in self.env.iter().rev() {
            let result = env.get(&key);
            if result.is_some() { return result.clone(); }
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


