use crate::{error, expr::Expr, token::Token, token_type::TokenType};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    variables: Vec<Vec<String>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            current: 0,
            variables: vec![],
        }
    }

    // ---------- ENTRY ----------
    pub fn parse(&mut self) -> Expr {
        self.variables.push(vec![]);
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(Box::new(self.statement()));
            if !self.match_any(&[TokenType::Semicolon]) && !self.is_at_end() {
                error(self.previous().line, "Expected ';' after statement.");
            }
        }

        Expr::StmtBlock(statements)
    }

    // ---------- STATEMENTS ----------
    fn statement(&mut self) -> Expr {
        if self.match_any(&[TokenType::Dollar]) {
            return self.print();
        }

        if self.match_any(&[TokenType::Fn]) {
            return self.define_function();
        }

        if self.match_any(&[TokenType::Hash]) {
            return self.declaration();
        }

        if self.match_any(&[TokenType::Del]) {
            return self.delete();
        }

        if self.check(TokenType::Ident) && self.peek_next(TokenType::Equal) {
            self.advance();
            return self.assignment();
        }

        self.expression()
    }

    // ---------- BLOCK ----------
    fn statement_block(&mut self) -> Expr {
        self.variables.push(vec![]);
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.statement();

            if self.check(TokenType::RightBrace) {
                statements.push(Box::new(stmt));
                break;
            }

            self.consume(TokenType::Semicolon, "Expected ';' after statement.");
            statements.push(Box::new(Expr::Discard(Box::new(stmt))));
        }
        self.consume(TokenType::RightBrace, "Expected '}' after block.");

        for var in self.variables.last().unwrap().iter() {
            statements.push(Box::new(Expr::Delete(var.clone())));
        }
        self.variables.pop();
        Expr::StmtBlock(statements)
    }

    // ---------- DELETE VAR -----------

    fn delete(&mut self) -> Expr {
        if self.match_any(&[TokenType::Ident]) {
            Expr::Delete(self.previous().lexeme)
        } else {
            error(self.previous().line, "Expected variable name after 'del'.");
            Expr::Nothing()
        }
    }

    // ---------- DECLARATION ----------
    fn declaration(&mut self) -> Expr {
        let is_mutable = self.match_any(&[TokenType::At]);

        self.consume(TokenType::Ident, "Expected variable name.");
        let name = self.previous().lexeme.clone();

        if self.match_any(&[TokenType::Colon]) {
            let var_type = if self.match_any(&[TokenType::Ident]) {
                self.previous().lexeme.clone()
            } else {
                error(
                    self.previous().line,
                    format!(
                        "Expected variable type after colon, but got '{}'.",
                        self.previous().token_type
                    )
                    .as_str(),
                );
                String::from("[]")
            };

            self.variables.last_mut().unwrap().push(name.clone());
            Expr::Declare(name, var_type, is_mutable)
        } else {
            self.consume(TokenType::Equal, "Expected '=' after variable name.");
            let value = self.expression();

            self.variables.last_mut().unwrap().push(name.clone());
            Expr::DeclareAndAssign(name, Box::new(value), is_mutable)
        }
    }

    // ---------- ASSIGNMENT ----------
    fn assignment(&mut self) -> Expr {
        let name = self.previous().lexeme.clone();
        self.consume(TokenType::Equal, "Expected '=' after identifier.");
        Expr::Assign(name, Box::new(self.expression()))
    }

    // ------- IF / ELSE IF / ELSE ----

    fn if_statement(&mut self) -> Expr {
        let if_cond = self.expression();
        // self.current += 1;
        let if_block = if self.match_any(&[TokenType::LeftBrace]) {
            self.statement_block()
        } else {
            error(
                self.previous().line,
                ("Expected '{' after 'if' condition, found '".to_string()
                    + self.previous().lexeme.as_str()
                    + "'")
                    .as_str(),
            );
            Expr::Nothing()
        };

        let mut else_block = None;

        if self.match_any(&[TokenType::TildeQuestionMark]) {
            else_block = Some(Box::new(self.if_statement()));
        } else if self.match_any(&[TokenType::Tilde]) {
            self.consume(TokenType::LeftBrace, "Expected '{' after 'TILDE'.");
            else_block = Some(Box::new(self.statement_block()));
        }

        Expr::If(Box::new(if_cond), Box::new(if_block), else_block)
    }
    // ---------- PRINT ---------------
    fn print(&mut self) -> Expr {
        if self.peek().token_type == TokenType::Dollar {
            self.advance();
            Expr::StmtBlock(vec![Box::new(Expr::Add(
                Box::new(self.print()),
                Box::new(Expr::Str("\n".to_string())),
            ))])
        } else if self.peek().token_type == TokenType::Semicolon {
            Expr::Str(String::new())
        } else {
            Expr::Print(Box::new(self.expression()))
        }
    }

    // ---------- FUNCTIONS ------------

    fn define_function(&mut self) -> Expr {
        let is_mutable = self.match_any(&[TokenType::At]);

        let (return_type, name) =
            if self.check(TokenType::Ident) && self.peek_next(TokenType::Ident) {
                (self.advance().lexeme, self.advance().lexeme)
            } else {
                ("[]".to_string(), self.advance().lexeme)
            };

        let _parameters: Vec<String> = if self.match_any(&[TokenType::LeftBrack]) {
            self.consume(
                TokenType::RightBrack,
                "Expected ']' after function parameters.",
            );
            vec![]
        } else {
            vec![]
        };
        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after function declaration.",
        );

        let body = Box::new(self.statement_block());

        Expr::Function(name, body, return_type, is_mutable)
    }

    fn call_function(&mut self) -> Expr {
        let name = self.previous().lexeme;

        if self.match_any(&[TokenType::LeftParen]) {
            self.consume(TokenType::RightParen, "Missing ')' after function call.");
        }

        Expr::CallFunc(name)
    }

    // ---------- EXPRESSIONS ----------
    fn expression(&mut self) -> Expr {
        self.bools()
    }

    fn bools(&mut self) -> Expr {
        let mut expr = self.compare();

        while self.match_any(&[TokenType::And, TokenType::Or]) {
            let op = self.previous().token_type;
            let right = self.compare();
            expr = match op {
                TokenType::And => Expr::And(Box::new(expr), Box::new(right)),
                TokenType::Or => Expr::Or(Box::new(expr), Box::new(right)),
                _ => unreachable!(),
            };
        }

        expr
    }

    fn compare(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_any(&[
            TokenType::EqualEqual,
            TokenType::BangEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous().token_type;
            let right = self.term();
            expr = match op {
                TokenType::EqualEqual => Expr::EqualEqual(Box::new(expr), Box::new(right)),
                TokenType::BangEqual => Expr::BangEqual(Box::new(expr), Box::new(right)),
                TokenType::Greater => Expr::Greater(Box::new(expr), Box::new(right)),
                TokenType::GreaterEqual => Expr::GreaterEqual(Box::new(expr), Box::new(right)),
                TokenType::Less => Expr::Less(Box::new(expr), Box::new(right)),
                TokenType::LessEqual => Expr::LessEqual(Box::new(expr), Box::new(right)),
                _ => unreachable!(),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_any(&[TokenType::Plus, TokenType::Minus]) {
            let op = self.previous().token_type;
            let right = self.factor();
            expr = match op {
                TokenType::Plus => Expr::Add(Box::new(expr), Box::new(right)),
                TokenType::Minus => Expr::Sub(Box::new(expr), Box::new(right)),
                _ => unreachable!(),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_any(&[TokenType::Star, TokenType::Slash, TokenType::Mod]) {
            let op = self.previous().token_type;
            let right = self.unary();
            expr = match op {
                TokenType::Star => Expr::Mult(Box::new(expr), Box::new(right)),
                TokenType::Slash => Expr::Divide(Box::new(expr), Box::new(right)),
                TokenType::Mod => Expr::Mod(Box::new(expr), Box::new(right)),
                _ => unreachable!(),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_any(&[TokenType::Minus]) {
            Expr::Sub(Box::new(Expr::Nothing()), Box::new(self.unary()))
        } else if self.match_any(&[TokenType::Minus, TokenType::Plus]) {
            Expr::Add(Box::new(Expr::Nothing()), Box::new(self.unary()))
        } else if self.match_any(&[TokenType::Bang]) {
            Expr::Not(Box::new(self.unary()))
        } else {
            self.power()
        }
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.primary();

        while self.match_any(&[TokenType::StarStar]) {
            expr = Expr::Power(Box::new(expr), Box::new(self.primary()));
        }

        expr
    }

    // ---------- PRIMARY ----------
    fn primary(&mut self) -> Expr {
        if self.match_any(&[TokenType::LeftBrace]) {
            return self.statement_block();
        }

        if self.check(TokenType::Ident) && self.peek_next(TokenType::LeftParen) {
            self.consume(TokenType::Ident, "THIS SHOULD BE UNREACHABLE.");
            return self.call_function();
        }

        if self.match_any(&[TokenType::Ident]) {
            return Expr::Variable(self.previous().lexeme.clone());
        }

        if self.match_any(&[TokenType::Int, TokenType::Float]) {
            return Expr::Num(self.previous().literal.parse().unwrap_or(0.0));
        }

        if self.match_any(&[TokenType::True]) {
            return Expr::Bool(true);
        }

        if self.match_any(&[TokenType::False]) {
            return Expr::Bool(false);
        }

        if self.match_any(&[TokenType::String]) {
            return Expr::Str(self.previous().literal.clone());
        }

        if self.match_any(&[TokenType::CHAR]) {
            return Expr::Char(self.previous().literal.clone());
        }

        if self.match_any(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')'.");
            return expr;
        }

        if self.match_any(&[TokenType::QuestionMark]) {
            return self.if_statement();
        }

        error(
            self.peek().line,
            format!("Unexpected token '{}'", self.peek().token_type).as_str(),
        );
        self.advance();
        Expr::Nothing()
    }

    // ---------- UTIL ----------
    fn match_any(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn peek_next(&self, token_type: TokenType) -> bool {
        self.tokens
            .get(self.current + 1)
            .map(|t| t.token_type == token_type)
            .unwrap_or(false)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            self.advance()
        } else {
            error(
                self.peek().line,
                format!("{} Found: {}", message, self.peek().token_type).as_str(),
            );
            Token::nil()
        }
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
            .unwrap_or_else(Token::nil)
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current.saturating_sub(1))
            .cloned()
            .unwrap_or_else(Token::nil)
    }
}

// Grammar:
/*
statement_block -> "{" ( statement )+ "}"
statement       -> print | declaration | expression | return | function

print           -> "$" ( "$" )? expression ";"
declaration     -> "#" ( "@" )? IDENTIFIER ( "=" expression )? ";"
return          -> "ret" expression ";"

if_statement    -> "?" expression statement_block ( "~?" expression statement_block )* ( "~" statement_block )?
function_call   -> IDENTIFIER "(" (expression)* ")"
function        -> "fn" (IDENTIFIER)? IDENTIFIER ( "[" (IDENTIFIER ":" IDENTIFIER)* "]" )? statement_block

expression      -> bool
bool            -> compare ( ( "&" | "|" ) compare )*
compare         -> term ( ( "==" | ">=" | "<=" | ">" | "<" | "!=" ) term )*
term            -> factor ( ( "+" | "-" ) factor )*
factor          -> unary ( ( "*" | "/" | "%" ) unary )*
unary           -> ( "-" | "+" ) unary | power
power           -> primary ( ( "**" ) primary )*
primary         -> NUMBER | STRING | BOOLEAN | IDENTIFIER | "(" expression ")" | statement_block | if_statement | function_call

*/
