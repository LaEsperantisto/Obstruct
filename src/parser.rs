use crate::error;
use crate::{expr::Expr, token::Token, token_type::TokenType as TT};
// Assuming you have main.rs with `error` and `report` functions

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
        let tree = self.expression();
        if self.current != self.tokens.len() - 1 {
            self.consume(TT::EOF, "");
        }
        tree
    }

    fn expression(&mut self) -> Expr {
        self.bools()
    }

    fn bools(&mut self) -> Expr {
        let mut expr = self.compare();

        while self.match_any(&[TT::AND, TT::OR]) {
            let operator = self.previous();
            let right = self.compare();
            expr = match operator.token_type {
                TT::AND => Expr::And(Box::new(expr), Box::new(right)),
                TT::OR => Expr::Or(Box::new(expr), Box::new(right)),
                _ => expr, // impossible
            };
        }

        expr
    }

    fn compare(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_any(&[
            TT::EQUAL_EQUAL,
            TT::GREATER_EQUAL,
            TT::LESS_EQUAL,
            TT::GREATER,
            TT::LESS,
            TT::BANG_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = match operator.token_type {
                TT::EQUAL_EQUAL => Expr::EqualEqual(Box::new(expr), Box::new(right)),
                TT::BANG_EQUAL => Expr::BangEqual(Box::new(expr), Box::new(right)),
                TT::GREATER_EQUAL => Expr::GreaterEqual(Box::new(expr), Box::new(right)),
                TT::LESS_EQUAL => Expr::LessEqual(Box::new(expr), Box::new(right)),
                TT::LESS => Expr::Less(Box::new(expr), Box::new(right)),
                TT::GREATER => Expr::Greater(Box::new(expr), Box::new(right)),
                _ => expr,
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_any(&[TT::PLUS, TT::MINUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = if operator.token_type == TT::PLUS {
                Expr::Add(Box::new(expr), Box::new(right))
            } else {
                Expr::Sub(Box::new(expr), Box::new(right))
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_any(&[TT::STAR, TT::SLASH, TT::MOD]) {
            let operator = self.previous();
            let right = self.unary();
            expr = match operator.token_type {
                TT::STAR => Expr::Mult(Box::new(expr), Box::new(right)),
                TT::MOD => Expr::Mod(Box::new(expr), Box::new(right)),
                TT::SLASH => Expr::Divide(Box::new(expr), Box::new(right)),
                _ => expr,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_any(&[TT::MINUS]) {
            let right = self.power();
            return Expr::Sub(Box::new(Expr::Num(0.0)), Box::new(right));
        } else if self.match_any(&[TT::PLUS]) {
            let right = self.power();
            return Expr::Add(Box::new(Expr::Num(0.0)), Box::new(right));
        } else if self.match_any(&[TT::BANG]) {
            let right = self.power();
            return Expr::Not(Box::new(right));
        }
        self.power()
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.primary();

        while self.match_any(&[TT::STAR_STAR]) {
            let _operator = self.previous();
            let right = self.primary();
            expr = Expr::Power(Box::new(expr), Box::new(right));
        }

        expr
    }

    fn primary(&mut self) -> Expr {
        if self.match_any(&[TT::INT, TT::FLOAT]) {
            let val: f64 = self.previous().lexeme.parse().unwrap_or(0.0);
            return Expr::Num(val);
        }

        if self.match_any(&[TT::MINUS, TT::PLUS]) {
            return self.expression();
        }

        if self.match_any(&[TT::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TT::RIGHT_PAREN, "Expected ')' after expression.");
            return expr;
        }

        error(
            self.peek().line,
            &format!("Unexpected token: {:?}", self.peek().token_type),
        );
        Expr::Num(0.0)
    }

    // Utilities
    fn match_any(&mut self, types: &[TT]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TT, message: &str) -> Token {
        if self.check(token_type.clone()) {
            return self.advance();
        }
        error(
            self.peek().line,
            &format!("{} Found: {:?}", message, self.peek().token_type),
        );
        Token::nil()
    }

    fn check(&self, token_type: TT) -> bool {
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
        self.peek().token_type == TT::EOF
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TT::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TT::CLS
                | TT::HASH_AT
                | TT::HASH
                | TT::FOR
                | TT::QUESTION_MARK
                | TT::POUND
                | TT::DOLLAR
                | TT::RET => return,
                _ => self.advance(),
            };
        }
    }
}
