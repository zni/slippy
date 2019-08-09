use crate::ast::{Token, TokenType, Expr};

pub struct Parser {
    pub current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, &'static str> {
        let mut program = Vec::new();
        loop {
            if self.is_at_end() {
                break;
            }

            let result = self.quote();
            if result.is_err() { return Err(result.unwrap_err()) }
            program.push(result.unwrap());
        }
        Ok(program)
    }

    fn quote(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![TokenType::Quote]) {
            let datum = self.quote();
            if datum.is_ok() {
                Ok(Expr::List(vec![Expr::Var("quote".to_string()),
                                   datum.unwrap()]))
            } else {
                datum
            }
        } else {
            self.datum()
        }
    }

    fn datum(&mut self) -> Result<Expr, &'static str> {
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
                let datum = self.quote();
                if datum.is_err() { return Err(datum.unwrap_err()); }
                lexprs.push(datum.unwrap());

                // Dotted Pair
                if self.match_token(vec![TokenType::Dot]) {
                    let rexpr = self.quote();
                    let rparen = self.expect(TokenType::RParen, "expecting right paren");
                    if rexpr.is_err() { return Err(rexpr.unwrap_err()); }
                    if rparen.is_err() { return Err(rparen.unwrap_err()); }
                    let rexpr = rexpr.unwrap();

                    if rexpr.is_list() {
                        let rexpr = rexpr.to_vec().unwrap();
                        lexprs.append(&mut rexpr.clone());
                        return Ok(Expr::List(lexprs));
                    } else {
                        return Ok(Expr::DottedPair(lexprs, Box::new(rexpr)));
                    }

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
                                 TokenType::Bool,
                                 TokenType::String]) {
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
}
