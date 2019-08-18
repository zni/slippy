use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::builtins;
use crate::ast::Expr;

#[derive(Clone, Debug, Default)]
pub struct Env {
    node: Option<Rc<RefCell<Env>>>,
    env: HashMap<String, Expr>
}

impl Env {
    pub fn new() -> Rc<RefCell<Env>> {
        let mut global_env = HashMap::new();
        global_env.insert(String::from("="), Expr::Builtin(builtins::equal));
        global_env.insert(String::from("<"), Expr::Builtin(builtins::lt));
        global_env.insert(String::from("<="), Expr::Builtin(builtins::lte));
        global_env.insert(String::from(">"), Expr::Builtin(builtins::gt));
        global_env.insert(String::from(">="), Expr::Builtin(builtins::gte));
        global_env.insert(String::from("+"), Expr::Builtin(builtins::add));
        global_env.insert(String::from("-"), Expr::Builtin(builtins::sub));
        global_env.insert(String::from("*"), Expr::Builtin(builtins::mul));
        global_env.insert(String::from("list"), Expr::Builtin(builtins::list));
        global_env.insert(String::from("car"), Expr::Builtin(builtins::car));
        global_env.insert(String::from("cdr"), Expr::Builtin(builtins::cdr));
        global_env.insert(String::from("cons"), Expr::Builtin(builtins::cons));
        global_env.insert(String::from("append"), Expr::Builtin(builtins::append));
        global_env.insert(String::from("length"), Expr::Builtin(builtins::length));
        global_env.insert(String::from("reverse"), Expr::Builtin(builtins::reverse));
        global_env.insert(String::from("equal?"), Expr::Builtin(builtins::equalp));
        global_env.insert(String::from("list?"), Expr::Builtin(builtins::listp));
        global_env.insert(String::from("null?"), Expr::Builtin(builtins::nullp));
        global_env.insert(String::from("number?"), Expr::Builtin(builtins::numberp));
        global_env.insert(String::from("procedure?"), Expr::Builtin(builtins::procedurep));
        global_env.insert(String::from("symbol?"), Expr::Builtin(builtins::symbolp));
        global_env.insert(String::from("pair?"), Expr::Builtin(builtins::pairp));
        global_env.insert(String::from("apply"), Expr::Builtin(builtins::apply));
        global_env.insert(String::from("load"), Expr::Builtin(builtins::load));
        global_env.insert(String::from("read"), Expr::Builtin(builtins::read));
        global_env.insert(String::from("display"), Expr::Builtin(builtins::display));
        Rc::new(RefCell::new(Env { node: None, env: global_env }))
    }

    pub fn get(&self, key: &String) -> Option<Expr> {
        let result = self.env.get(key);
        match result {
            Some(result) => Some(result.clone()),
            None => {
                match &self.node {
                    Some(env) => env.borrow().get(key),
                    None => None
                }
            },
        }
    }

    pub fn set(&mut self, key: String, value: Expr) -> Result<(), &'static str> {
        let result = self.env.get(&key);
        match result {
            Some(_) => {
                self.env.insert(key, value);
                return Ok(())
            },
            None => {
                match &self.node {
                    Some(env) => env.borrow_mut().set(key, value),
                    None => return Err("key not found")
                }
            },
        }
    }

    pub fn insert(&mut self, key: String, value: Expr) {
        self.env.insert(key, value);
    }

    pub fn extend_env(&mut self, prev_env: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        Rc::new(RefCell::new(Env { node: Some(prev_env), env: HashMap::new() }))
    }
}


