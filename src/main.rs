use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use std::cell::RefCell;
use std::rc::Rc;

extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use slippy::env::Env;
use slippy::eval::eval;
use slippy::lexer::Lexer;
use slippy::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("usage: slippy <file>");
        process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt();
    }
}

fn run_file(file: &String) {
    let env = Env::new();

    let path = Path::new(file);
    let mut file = File::open(&path)
        .expect("Failed to open file");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read file");

    run(&source, env);
}

fn run_prompt() {
    let mut rl = Editor::<()>::new();
    let env = Env::new();
    loop {
        let readline = rl.readline("slippy> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                run(&line, env.clone());
            },
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(_) => break,
        }
    }
}

fn run(source: &String, env: Rc<RefCell<Env>>) {
    let mut lexer = Lexer::new(&source);
    lexer.scan();
    let mut parser = Parser::new(lexer.tokens);
    let result = parser.parse();
    if result.is_ok() {
        let exprs = result.unwrap();
        for expr in exprs.iter() {
            let eval_result = eval(expr, env.clone());
            if eval_result.is_ok() {
                println!("{}", eval_result.unwrap());
            } else {
                println!("{}", eval_result.unwrap_err());
            }
        }
    } else {
        println!("{}", result.unwrap_err());
        println!("pos: {}", parser.current);
    }
}
