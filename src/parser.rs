use crate::{error, expr::Expr, token::Token, token_type::TokenType};
use std::hash::Hash;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    // Entry point
    pub fn parse(&mut self) -> Expr {
        let expr = self.statement();
        if !self.is_at_end() {
            error(
                -1,
                &format!(
                    "Unexpected token at start of statement: {}",
                    self.advance().token_type
                ),
            );
        }
        expr
    }

    fn statement(&mut self) -> Expr {
        let statements = &[TokenType::DOLLAR, TokenType::HASH, TokenType::IDENTIFIER];
        if !self.match_any(statements) {
            let token = self.peek();
            error(
                token.line,
                &format!(
                    "Unexpected token at start of statement: {:?}",
                    token.token_type
                ),
            );
        }

        let mut exprs = Vec::new();

        loop {
            match self.previous().token_type {
                TokenType::DOLLAR => exprs.push(Box::new(self.print())),
                TokenType::HASH => exprs.push(Box::new(self.hash())),
                TokenType::IDENTIFIER => exprs.push(Box::new(self.identifier())),
                _ => panic!(
                    "TOKEN '{}' NOT RECOGNISED! INTERPRETER HORRIBLY PROGRAMMED!",
                    self.previous().token_type
                ),
            }

            self.consume(TokenType::SEMICOLON, "Expected \";\" after statement.");

            if !self.match_any(statements) {
                break;
            }
        }

        Expr::StmtBlock(exprs)
    }

    /// Parse statements
    fn print(&mut self) -> Expr {
        if self.peek().token_type == TokenType::SEMICOLON {
            Expr::Print(Box::new(Expr::Str("\n".to_string())))
        } else {
            let expr = self.bools();
            Expr::Print(Box::new(expr))
        }
    }

    fn hash(&mut self) -> Expr {
        let is_mutable = self.match_any(&[TokenType::AT]);

        self.consume(TokenType::IDENTIFIER, "Expected variable name after '#'.");
        let name = self.previous().lexeme.clone();

        self.consume(TokenType::EQUAL, "Expected '=' after declaring variable.");

        Expr::Declare(name, Box::new(self.expression()), is_mutable)
    }

    fn identifier(&mut self) -> Expr {
        let name = self.previous().lexeme.clone();

        match self.peek().token_type {
            TokenType::EQUAL => {
                self.advance(); // ✅ consume '='
                let value = self.expression();
                Expr::Assign(name, Box::new(value))
            }

            _ => {
                error(
                    self.peek().line,
                    &format!(
                        "Unexpected token '{}' after identifier",
                        self.peek().token_type
                    ),
                );
                Expr::Num(0.0)
            }
        }
    }

    /// Parse expressions starting with booleans
    fn expression(&mut self) -> Expr {
        self.bools()
    }

    fn bools(&mut self) -> Expr {
        let mut expr = self.compare();

        while self.match_any(&[TokenType::AND, TokenType::OR]) {
            let operator = self.previous();
            let right = self.compare();
            expr = match operator.token_type {
                TokenType::AND => Expr::And(Box::new(expr), Box::new(right)),
                TokenType::OR => Expr::Or(Box::new(expr), Box::new(right)),
                _ => expr,
            };
        }

        expr
    }

    fn compare(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_any(&[
            TokenType::EQUAL_EQUAL,
            TokenType::BANG_EQUAL,
            TokenType::GREATER_EQUAL,
            TokenType::LESS_EQUAL,
            TokenType::GREATER,
            TokenType::LESS,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = match operator.token_type {
                TokenType::EQUAL_EQUAL => Expr::EqualEqual(Box::new(expr), Box::new(right)),
                TokenType::BANG_EQUAL => Expr::BangEqual(Box::new(expr), Box::new(right)),
                TokenType::GREATER_EQUAL => Expr::GreaterEqual(Box::new(expr), Box::new(right)),
                TokenType::LESS_EQUAL => Expr::LessEqual(Box::new(expr), Box::new(right)),
                TokenType::LESS => Expr::Less(Box::new(expr), Box::new(right)),
                TokenType::GREATER => Expr::Greater(Box::new(expr), Box::new(right)),
                _ => expr,
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_any(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = if operator.token_type == TokenType::PLUS {
                Expr::Add(Box::new(expr), Box::new(right))
            } else {
                Expr::Sub(Box::new(expr), Box::new(right))
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_any(&[TokenType::STAR, TokenType::SLASH, TokenType::MOD]) {
            let operator = self.previous();
            let right = self.unary();
            expr = match operator.token_type {
                TokenType::STAR => Expr::Mult(Box::new(expr), Box::new(right)),
                TokenType::MOD => Expr::Mod(Box::new(expr), Box::new(right)),
                TokenType::SLASH => Expr::Divide(Box::new(expr), Box::new(right)),
                _ => expr,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_any(&[TokenType::MINUS]) {
            let right = self.power();
            Expr::Sub(Box::new(Expr::Num(0.0)), Box::new(right))
        } else if self.match_any(&[TokenType::PLUS]) {
            let right = self.power();
            Expr::Add(Box::new(Expr::Num(0.0)), Box::new(right))
        } else if self.match_any(&[TokenType::BANG]) {
            let right = self.power();
            Expr::Not(Box::new(right))
        } else {
            self.power()
        }
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.primary();

        while self.match_any(&[TokenType::STAR_STAR]) {
            let _operator = self.previous();
            let right = self.primary();
            expr = Expr::Power(Box::new(expr), Box::new(right));
        }

        expr
    }

    fn primary(&mut self) -> Expr {
        if self.match_any(&[TokenType::IDENTIFIER]) {
            let name = self.previous().lexeme.clone();
            return Expr::Variable(name);
        }

        if self.match_any(&[TokenType::INT, TokenType::FLOAT]) {
            let val: f64 = self.previous().literal.parse().unwrap_or(0.0);
            return Expr::Num(val);
        }

        if self.match_any(&[TokenType::TRUE]) {
            return Expr::Bool(true);
        }

        if self.match_any(&[TokenType::FALSE]) {
            return Expr::Bool(false);
        }

        if self.match_any(&[TokenType::STRING]) {
            let val = self.previous().literal.clone();
            return Expr::Str(val);
        }

        if self.match_any(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression.");
            return expr;
        }

        let token = self.peek();
        error(
            token.line,
            &format!("Unexpected token: {:?}", token.token_type),
        );
        Expr::Num(0.0)
    }

    // ----- Utilities -----
    fn match_any(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type.clone()) {
            return self.advance();
        }
        error(
            self.peek().line,
            &format!("{} Found: {:?}", message, self.peek().token_type),
        );
        Token::nil()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .cloned()
            .unwrap_or_else(|| Token::nil())
    }

    fn previous(&self) -> Token {
        if self.current == 0 {
            Token::nil()
        } else {
            self.tokens
                .get(self.current - 1)
                .cloned()
                .unwrap_or_else(|| Token::nil())
        }
    }
}
