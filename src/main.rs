use std::io;

use slippy::lexer::Lexer;
use slippy::parser::Parser;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line.");
        let mut lexer = Lexer::new(&line);
        lexer.scan();
        let mut parser = Parser::new(lexer.tokens);
        let result = parser.parse();
        if result.is_ok() {
            println!("{}", result.unwrap());
        } else {
            println!("{}", result.unwrap_err());
        }
    }
}
