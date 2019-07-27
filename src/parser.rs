use crate::ast::{Token, TokenType, Literal, Expr};

pub struct Parser {
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        self.list()
    }

    pub fn list(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::LParen]) {
            let mut lexprs = Vec::new();

            // Empty list
            if self.check(TokenType::RParen) {
                self.advance();
                return Ok(Expr::List(lexprs));
            }

            loop {
                let datum = self.simple_datum();
                if datum.is_err() { return Err(datum.unwrap_err()); }
                lexprs.push(datum.unwrap());

                if self.match_token(vec![TokenType::Dot]) {
                    let rexpr = self.simple_datum();
                    let rparen = self.expect(TokenType::RParen, "expecting right paren");
                    if rexpr.is_err() { return Err(rexpr.unwrap_err()); }
                    if rparen.is_err() { return Err(rparen.unwrap_err()); }

                    return Ok(Expr::DottedPair(lexprs, Box::new(rexpr.unwrap())));

                } else if self.match_token(vec![TokenType::RParen]) {
                    return Ok(Expr::List(lexprs));
                }
            }
        } else {
            Err("expecting a list")
        }
    }

    fn simple_datum(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::Number, TokenType::Float]) {
            Ok(Expr::Literal(self.previous()))
        } else if self.match_token(vec![TokenType::Identifier]) {
            Ok(Expr::Var(self.previous()))
        } else {
            Err("expecting number, float, or identifier")
        }
    }

    fn match_token(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn expect(&mut self,
              token: TokenType,
              err: &'static str) -> Result<Token, &'static str> {
        if self.check(token) {
            let token: Token = self.tokens[self.current].clone();
            self.advance();
            return Ok(token);
        }
        Err(err)
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        token.ttype == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        let token = self.peek();
        token.ttype == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
