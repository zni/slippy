use crate::ast::{Literal, Token, TokenType};

pub struct Lexer {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Lexer {
    pub fn new(block: &str) -> Lexer {
        let source: Vec<char> = block.chars().collect();
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EOF);
    }

    fn scan_token(&mut self) {
        let n = self.advance();
        match n {
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            '.' => {
                if Lexer::is_ident(self.peek()) {
                    self.identifier()
                } else {
                    self.add_token(TokenType::Dot)
                }
            },
            '\'' => self.add_token(TokenType::Quote),
            ' ' => (),
            '#' => {
                if self.match_char('t') {
                    self.add_literal_token(TokenType::Bool, Some(Literal::Bool(true)))
                } else if self.match_char('f') {
                    self.add_literal_token(TokenType::Bool, Some(Literal::Bool(false)))
                }
            },
            '\t' => (),
            '\r' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _   => {
                if Lexer::is_digit(n) {
                    self.number();
                } else if Lexer::is_ident(n) {
                    self.identifier();
                } else {
                    self.error("unexpected symbol");
                }
            },
        }
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_dot(c: char) -> bool {
        c == '.'
    }

    fn is_ident(c: char) -> bool {
        c.is_ascii_alphabetic() ||
            c == '!' ||
            c == '$' ||
            c == '%' ||
            c == '&' ||
            c == '*' ||
            c == '+' ||
            c == '-' ||
            c == '.' ||
            c == '/' ||
            c == ':' ||
            c == '<' ||
            c == '=' ||
            c == '>' ||
            c == '?' ||
            c == '@' ||
            c == '^' ||
            c == '_' ||
            c == '~'
    }

    fn float(&mut self) {
        while Lexer::is_digit(self.peek()) {
            self.advance();
        }

        let slice: Vec<char> = self.source[self.start..self.current].to_vec();
        let slice: String = slice.iter().collect();
        let digit: f64 = match slice.parse() {
            Ok(d) => d,
            Err(_) => {
                0.0
            }
        };
        self.add_literal_token(TokenType::Float, Some(Literal::Float(digit)));
    }

    fn number(&mut self) {
        while Lexer::is_digit(self.peek()) {
            self.advance();
        }
        if Lexer::is_dot(self.peek()) {
            self.advance();
            return self.float();
        }

        let slice: Vec<char> = self.source[self.start..self.current].to_vec();
        let slice: String = slice.iter().collect();
        let digit: i32 = match slice.parse() {
            Ok(d) => d,
            Err(_) => {
                0
            }
        };
        self.add_literal_token(TokenType::Number, Some(Literal::Number(digit)))
    }

    fn identifier(&mut self) {
        while Lexer::is_ident(self.peek()) || Lexer::is_digit(self.peek()) {
            self.advance();
        }

        self.add_token(TokenType::Identifier)
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.error("unterminated string.");
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("unterminated string.");
            return;
        }

        self.advance();
        let slice: Vec<char> = self.source[self.start + 1..self.current - 1].to_vec();
        let slice: String = slice.iter().collect();
        self.add_literal_token(TokenType::String, Some(Literal::String(slice)));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_literal_token(token, None);
    }

    fn add_literal_token(&mut self,
                         token: TokenType,
                         literal: Option<Literal>) {

        let slice: Vec<char> = self.source[self.start..self.current].to_vec();
        let lexeme: String = slice.iter().collect();
        self.tokens.push(Token::new(token, lexeme, self.line, literal));
    }

    fn error(&mut self, message: &str) {
        let slice: Vec<char> = self.source[self.start..self.current].to_vec();
        let lexeme: String = slice.iter().collect();
        println!("error at line {}, symbol '{}': {}", self.line, lexeme, message);
    }
}
