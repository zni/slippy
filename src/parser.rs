use crate::ast::{Token, TokenType, Literal, Expr};

pub struct Parser {
    pub current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        self.datum()
    }
/*
    fn expression(&mut self) -> Result<Expr, &'static str> {
        let constant = self.constant();
        if constant.is_ok() {
            return constant;
        }

        let variable = self.variable();
        if variable.is_ok() {
            return variable;
        }

        if !self.match_token(vec![TokenType::LParen]) {
            return Err("expecting left paren");
        }

        // Quote
        if self.match_token(vec![TokenType::Quote]) {
            let datum = self.datum();

            if datum.is_ok() && self.check(TokenType::RParen) {
                self.advance();
                return Ok(Expr::Quote(Box::new(datum.unwrap())));
            }

        // Lambda
        } else if self.match_token(vec![TokenType::Lambda]) {
            let formals = self.formals();
            if formals.is_err() { return Err(formals.unwrap_err()); }

            let body = self.datum();
            if body.is_err() { return Err(body.unwrap_err()); }

            if self.check(TokenType::RParen) {
                self.advance();
                return Ok(Expr::Lambda(formals.unwrap(),
                                       Box::new(body.unwrap())));
            }

        // Application
        } else {
            let exp = self.expression();
            if exp.is_err() { return Err(exp.unwrap_err()); }

            let mut operands = Vec::new();
            while !self.check(TokenType::RParen) {
                let op = self.expression();
                if op.is_err() { return Err(op.unwrap_err()); }

                operands.push(op.unwrap());
            }
            self.advance();
            return Ok(Expr::App(Box::new(exp.unwrap()), operands));
        }

        if !self.match_token(vec![TokenType::RParen]) {
            return Err("expecting right paren");
        }

        Err("expecting expression")
    }

    fn formals(&mut self) -> Result<Vec<Expr>, &'static str> {
        let mut vars = Vec::new();
        let lparen = self.expect(TokenType::LParen, "expecting left paren");
        if lparen.is_err() { return Err(lparen.unwrap_err()); }
        loop {
            if self.match_token(vec![TokenType::RParen]) { break; }

            let var = self.variable();
            if var.is_err() { return Err(var.unwrap_err()) }
            vars.push(var.unwrap())
        }

        Ok(vars)
    }

    fn variable(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::Identifier]) {
            Ok(Expr::Var(self.previous()))
        } else {
            Err("expecting variable")
        }
    }
    */

    fn datum(&mut self) -> Result<Expr, &'static str> {

        // self.list().or({println!("fallback to datum"); self.simple_datum()})
        let simple_datum = self.simple_datum();
        if simple_datum.is_ok() {
            return simple_datum;
        }

        self.list()
    }

    fn list(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::LParen]) {
            let mut lexprs = Vec::new();

            // Empty list
            if self.check(TokenType::RParen) {
                self.advance();
                return Ok(Expr::List(lexprs));
            }

            loop {
                let datum = self.datum();
                if datum.is_err() { return Err(datum.unwrap_err()); }
                lexprs.push(datum.unwrap());

                // Dotted Pair
                if self.match_token(vec![TokenType::Dot]) {
                    let rexpr = self.datum();
                    let rparen = self.expect(TokenType::RParen, "expecting right paren");
                    if rexpr.is_err() { return Err(rexpr.unwrap_err()); }
                    if rparen.is_err() { return Err(rparen.unwrap_err()); }

                    return Ok(Expr::DottedPair(lexprs, Box::new(rexpr.unwrap())));

                // List
                } else if self.match_token(vec![TokenType::RParen]) {
                    return Ok(Expr::List(lexprs));
                }
            }
        }

        Err("expecting a list")
    }

    fn simple_datum(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::Number,
                                 TokenType::Float,
                                 TokenType::Bool]) {
            Ok(Expr::Literal(self.previous().literal.unwrap()))
        } else if self.match_token(vec![TokenType::Identifier]) {
            Ok(Expr::Var(self.previous().lexeme))
        } else {
            Err("expecting number, float, boolean, or identifier")
        }
    }
/*
    fn constant(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::Number,
                                 TokenType::Float,
                                 TokenType::Bool]) {
            Ok(Expr::Literal(self.previous()))
        } else {
            Err("expecting number, float, or boolean")
        }
    }
    */

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

    fn rewind(&mut self) {
        if self.current != 0 {
            self.current -= 1;
        }
    }
}
