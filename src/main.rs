extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use slippy::lexer::Lexer;
use slippy::parser::Parser;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("slippy > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let mut lexer = Lexer::new(&line);
                lexer.scan();
                let mut parser = Parser::new(lexer.tokens);
                let result = parser.parse();
                if result.is_ok() {
                    println!("{}", result.unwrap());
                } else {
                    println!("{}", result.unwrap_err());
                    println!("pos: {}", parser.current);
                }
            },
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(_) => break,
        }
    }
}
