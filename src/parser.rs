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
            .or({self.rewind(); self.lambda()})
    }

    fn lambda(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::LParen]) {
            let lambda = self.expect(TokenType::Lambda, "expecting lambda");
            if lambda.is_err() { return Err(lambda.unwrap_err()); }

            let formals = self.formals();
            if formals.is_err() { return Err(formals.unwrap_err()); }

            let body = self.list().or(self.simple_datum());
            if body.is_err() { return Err(body.unwrap_err()); }

            return Ok(Expr::Lambda(formals.unwrap(), Box::new(body.unwrap())));
        }

        return Err("expecting lambda expression");
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

        return Ok(vars);
    }

    fn variable(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::Identifier]) {
            Ok(Expr::Var(self.previous()))
        } else {
            Err("expecting variable")
        }
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
                let datum = self.simple_datum();
                if datum.is_err() { return Err(datum.unwrap_err()); }
                lexprs.push(datum.unwrap());

                // Dotted Pair
                if self.match_token(vec![TokenType::Dot]) {
                    let rexpr = self.simple_datum();
                    let rparen = self.expect(TokenType::RParen, "expecting right paren");
                    if rexpr.is_err() { return Err(rexpr.unwrap_err()); }
                    if rparen.is_err() { return Err(rparen.unwrap_err()); }

                    return Ok(Expr::DottedPair(lexprs, Box::new(rexpr.unwrap())));

                // List
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

    fn rewind(&mut self) {
        self.current -= 1;
    }
}
