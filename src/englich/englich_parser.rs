use crate::span::Span;
use crate::type_env::{nil_type, Type};
use crate::{
    englich::englich_token::Token, englich::englich_token_type::TokenType, error, expr::Expr,
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    // ---------- ENTRY ----------
    pub fn parse(&mut self) -> Expr {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(Box::new(self.statement()));
            if !self.match_any(&[TokenType::Dot]) && !self.is_at_end() {
                let t = self.peek();
                error(
                    self.get_span(),
                    format!("Expected ',' after statement. Found '{}'", t.token_type).as_str(),
                );
                self.advance();
            }
        }

        Expr::StmtBlock(statements, self.get_span())
    }

    // ---------- STATEMENTS ----------
    fn statement(&mut self) -> Expr {
        if self.match_any(&[TokenType::Show]) {
            return self.print();
        }

        if self.match_any(&[TokenType::Return]) {
            return self.return_stmt();
        }

        if self.match_any(&[TokenType::Enter]) {
            self.advance();
            let var = self.item();
            return Expr::StmtBlock(
                vec![
                    Box::new(Expr::Print(
                        Box::new(Expr::Str(var.clone() + ": ")),
                        self.get_span(),
                    )),
                    Box::new(Expr::Declare(
                        var.clone(),
                        None,
                        Some(Box::new(Expr::Int(0))),
                        true,
                        self.get_span(),
                    )),
                    Box::new(Expr::Input(var)),
                ],
                self.get_span(),
            );
        }

        /*if self.match_any(&[TokenType::Pound]) {
            return self.while_loop();
        }*/

        if self.match_any(&[TokenType::Declare]) {
            return self.define_function();
        }

        if self.match_any(&[TokenType::Let]) {
            return self.declaration();
        }
        /*
        if self.match_any(&[TokenType::Del]) {
            return self.delete();
        }

        if self.match_any(&[TokenType::QuestionMark]) {
            return self.if_statement(false);
        }*/

        if self.match_any(&[TokenType::Set]) {
            self.advance();
            return self.assignment();
        }

        /*if self.match_any(&[TokenType::Use]) {
            return self.use_file();
        }

        if self.match_any(&[TokenType::For]) {
            return self.for_loop();
        }*/

        Expr::Stmt(Box::new(self.expression()))
    }

    // ---------- USE ------------

    /*fn use_file(&mut self) -> Expr {
        let kind = if self.match_any(&[TokenType::Std]) {
            UseKind::Std
        } else {
            UseKind::Normal
        };
        self.consume(TokenType::String, "Expected file name after 'use' keyword");
        Expr::Use {
            kind,
            path: self.previous().literal,
            span: self.get_span(),
        }
    }*/

    // ---------- BLOCK ----------
    fn statement_block(&mut self) -> Expr {
        let mut statements = Vec::new();

        while !self.check(TokenType::End) && !self.is_at_end() {
            let statement = self.statement();

            if !self.check(TokenType::Dot) && !self.check(TokenType::RightParen) {
                let t = self.peek();
                error(
                    self.get_span(),
                    format!("Expected '.' after statement. Found '{}'", t.token_type).as_str(),
                );
                self.advance();
            }
            if self.match_any(&[TokenType::Dot]) {
                statements.push(Box::new(Expr::Discard(Box::new(statement))));
            } else {
                statements.push(Box::new(statement));
            }
        }

        self.consume(TokenType::End, "Expected 'end' after block.");
        while !self.check(TokenType::Dot) {
            self.advance();
        }
        Expr::StmtBlockWithScope(statements, self.get_span())
    }

    // ---------- DELETE VAR -----------

    fn delete(&mut self) -> Expr {
        if self.match_any(&[TokenType::Ident]) {
            Expr::Delete(self.previous().lexeme)
        } else {
            error(self.get_span(), "Expected variable name after 'del'.");
            Expr::Nothing()
        }
    }

    // ---------- NEW CLASS -----------

    // ---------- WHILE LOOP -----------

    fn while_loop(&mut self) -> Expr {
        let cond = self.expression();
        // self.current += 1;
        let block = if self.match_any(&[TokenType::Colon]) {
            self.statement_block()
        } else {
            error(
                self.get_span(),
                ("Expected ':' after 'while' condition, found '".to_string()
                    + self.previous().lexeme.as_str()
                    + "'")
                    .as_str(),
            );
            Expr::Nothing()
        };

        Expr::While(Box::new(cond), Box::new(block))
    }

    // ----------- FOR LOOP -----------

    /*fn for_loop(&mut self) -> Expr {
        self.consume(TokenType::Ident, "Expected identifier after 'For' token");
        let loopee = self.previous().lexeme;
        self.consume(TokenType::Colon, "Expected ':' after loopee");
        let looper = self.expression();
        self.consume(TokenType::LeftBrace, "Expected '{' after looper");
        let block = self.statement_block();

        Expr::For(loopee, Box::new(looper), Box::new(block), self.get_span())
    }*/

    // ---------- DECLARATION ----------
    fn declaration(&mut self) -> Expr {
        // let is_mutable = self.match_any(&[TokenType::At]);
        let is_mutable = true;

        self.advance();
        let name = self.item();

        let var_type = if self.match_any(&[TokenType::Nil]) {
            Some(self.get_type())
        } else {
            None
        };
        let expr = if self.match_any(&[TokenType::Be]) {
            Some(Box::new(self.expression()))
        } else {
            None
        };

        if var_type.is_none() && expr.is_none() {
            let span = self.get_span();
            error(span, "Expected type or expression or both, got neither");
        }
        Expr::Declare(name, var_type, expr, is_mutable, self.get_span())
    }

    // ---------- ASSIGNMENT ----------
    fn assignment(&mut self) -> Expr {
        let name = self.item();
        self.consume(TokenType::To, "Expected 'to' after identifier.");
        let span = self.get_span();
        Expr::Assign(name, Box::new(self.expression()), span)
    }

    // ------- IF / ELSE IF / ELSE ----

    /*fn if_statement(&mut self, is_expr: bool) -> Expr {
        let if_cond = self.expression();
        // self.current += 1;
        let if_block = if self.match_any(&[TokenType::LeftBrace]) {
            self.statement_block()
        } else {
            error(
                self.get_span(),
                ("Expected '{' after 'if' condition, found '".to_string()
                    + self.previous().lexeme.as_str()
                    + "'")
                    .as_str(),
            );
            Expr::Nothing()
        };

        let mut else_block = None;

        if self.match_any(&[TokenType::TildeQuestionMark]) {
            else_block = Some(Box::new(self.if_statement(is_expr)));
        } else if self.match_any(&[TokenType::Tilde]) {
            self.consume(TokenType::LeftBrace, "Expected '{' after 'TILDE'.");
            else_block = Some(Box::new(self.statement_block()));
        }

        Expr::If(Box::new(if_cond), Box::new(if_block), else_block, is_expr)
    }*/
    // ---------- PRINT ---------------
    fn print(&mut self) -> Expr {
        Expr::Print(Box::new(self.expression()), self.get_span())
    }

    // ---------- FUNCTIONS ------------

    fn define_function(&mut self) -> Expr {
        let generic_params = vec![];
        self.consume(TokenType::Function, "Expected 'function' after 'declare'.");

        /*if self.match_any(&[TokenType::LessLess]) {
            loop {
                self.consume(TokenType::Ident, "Expected generic name");
                generic_params.push(self.previous().lexeme);

                if !self.match_any(&[TokenType::Comma]) {
                    break;
                }
            }
            self.consume(TokenType::GreaterGreater, "Expected '>'");
        }*/

        self.advance();
        let name = self.item();

        let start_span = self.get_span();

        /*let parameters: Vec<(String, Type)> = if self.match_any(&[TokenType::LeftParen]) {
            let mut parameters = vec![];
            /*while !self.is_at_end() && !self.check(TokenType::RightParen) {
                let name = self.advance().lexeme;
                self.consume(TokenType::Colon, "Expected ':' after parameter name.");
                let var_type = self.get_type();
                parameters.push((name, var_type));
                if !self.match_any(&[TokenType::Comma]) {
                    break;
                }
            }

            self.consume(
                TokenType::RightParen,
                "Expected ')' after function parameters.",
            );*/
            parameters
        } else {
            vec![]
        };*/

        let return_type = if !self.check(TokenType::Colon) {
            Some(self.get_type())
        } else {
            None
        };

        self.consume(TokenType::Colon, "Expected ':' after function declaration.");

        let body = Box::new(self.statement_block());

        Expr::DeclareFunction(name, body, return_type, vec![], generic_params, start_span)
    }

    fn define_lambda(&mut self) -> Expr {
        let mut generic_params = vec![];

        /*if self.match_any(&[TokenType::LessLess]) {
            loop {
                self.consume(TokenType::Ident, "Expected generic name");
                generic_params.push(self.previous().lexeme);

                if !self.match_any(&[TokenType::Comma]) {
                    break;
                }
            }
            self.consume(TokenType::GreaterGreater, "Expected '>'");
        }*/

        let parameters: Vec<(String, Type)> = if self.match_any(&[TokenType::LeftParen]) {
            let mut parameters = vec![];
            /*while !self.is_at_end() && !self.check(TokenType::RightParen) {
                let name = self.advance().lexeme;
                self.consume(TokenType::Colon, "Expected ':' after parameter name.");
                let var_type = self.get_type();
                parameters.push((name, var_type));
                if !self.match_any(&[TokenType::Comma]) {
                    break;
                }
            }

            self.consume(
                TokenType::RightParen,
                "Expected ')' after function parameters.",
            );*/
            parameters
        } else {
            vec![]
        };

        let return_type = if !self.check(TokenType::LeftParen) {
            self.get_type()
        } else {
            nil_type()
        };

        self.consume(
            TokenType::LeftParen,
            "Expected '{' after function declaration.",
        );

        let body = Box::new(self.statement_block());

        Expr::Function(body, return_type, parameters, generic_params)
    }

    fn return_stmt(&mut self) -> Expr {
        let value = self.expression();
        Expr::Return(Box::new(value), self.get_span())
    }

    fn call_function(&mut self) -> Expr {
        let name = self.previous().lexeme;

        let generics = vec![];

        /*if self.match_any(&[TokenType::LessLess]) {
            loop {
                generics.push(self.get_type());

                if !self.match_any(&[TokenType::Comma]) {
                    break;
                }
            }
            self.consume(TokenType::GreaterGreater, "Expected '>' after generics");
        }*/

        self.consume(TokenType::LeftParen, "Expected '(' after function name.");

        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(Box::new(self.expression()));
                if !self.match_any(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Missing ')' after function call.");

        Expr::CallFunc(name, generics, arguments, self.get_span())
    }

    // ---------- EXPRESSIONS ----------

    fn expression(&mut self) -> Expr {
        let mut expr = self.primary();

        while self.match_any(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().token_type;
            let right = self.primary();

            expr = match operator {
                TokenType::Plus => Expr::Add(Box::new(expr), Box::new(right), self.get_span()),
                TokenType::Minus => Expr::Sub(Box::new(expr), Box::new(right), self.get_span()),
                _ => unreachable!(),
            };
        }

        expr
    }
    fn primary(&mut self) -> Expr {
        if self.match_any(&[TokenType::Int]) {
            return Expr::Int(str::parse::<i32>(&self.previous().lexeme).unwrap());
        }
        if self.match_any(&[TokenType::Float]) {
            return Expr::Float(str::parse::<f64>(&self.previous().lexeme).unwrap());
        }
        if self.match_any(&[TokenType::String]) {
            return Expr::Str(self.previous().lexeme);
        }

        if self.check(TokenType::Ident) {
            self.advance();
            let item = self.item();
            return Expr::Variable(item, self.get_span());
        }

        if self.match_any(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Missing ')' after expression.");
            return expr;
        }

        panic!("Unexpected token {:?}", self.advance())
    }

    fn get_type(&mut self) -> Type {
        if self.match_any(&[TokenType::Ident]) {
            let name = self.previous().lexeme.clone();

            // Generic placeholder (capital letter convention)
            /*if name.chars().next().unwrap().is_uppercase() && !self.check(TokenType::LessLess) {
                return Type::generic(&name);
            }

            if self.match_any(&[TokenType::LessLess]) {
                let mut gens = vec![];

                loop {
                    gens.push(self.get_type());
                    if !self.match_any(&[TokenType::Comma]) {
                        break;
                    }
                }

                self.consume(TokenType::GreaterGreater, "Expected '>' after generics");
                return Type::with_generics(&name, gens);
            }*/

            return Type::simple(&name);
        } /*else if self.match_any(&[TokenType::LeftBrack]) {
        let mut gens = vec![];

        loop {
        gens.push(self.get_type());
        if !self.match_any(&[TokenType::Comma]) {
        break;
        }
        }

        self.consume(TokenType::RightBrack, "Expected ']' after arr");
        return Type::with_generics("arr", gens);
        }*/

        nil_type()
    }

    // ---------- UTIL ----------
    fn get_span(&self) -> Span {
        let token = self.previous();
        Span {
            line: token.line,
            column: token.column,
        }
    }

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

    fn peek_next_next(&self, token_type: TokenType) -> bool {
        self.tokens
            .get(self.current + 2)
            .map(|t| t.token_type == token_type)
            .unwrap_or(false)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            self.advance()
        } else {
            let t = self.peek();
            error(
                self.get_span(),
                format!("{} Found: {}", message, t.token_type).as_str(),
            );
            self.advance();
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

    fn item(&mut self) -> String {
        self.current -= 1;
        let mut item = String::new();
        while self.advance().token_type == TokenType::Ident {
            item.push(' ');
            item.push_str(self.previous().lexeme.as_str());
        }
        item.remove(0);
        self.current -= 1;
        item
    }
}
