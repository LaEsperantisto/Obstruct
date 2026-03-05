use crate::englich::englich_token::Token;
use crate::englich::englich_token_type::TokenType;
use crate::error;
use crate::span::Span;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenType>,
    prev_c: char,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert("let".into(), TokenType::Let);
        keywords.insert("set".into(), TokenType::Set);
        keywords.insert("be".into(), TokenType::Be);
        keywords.insert("to".into(), TokenType::To);
        keywords.insert("true".into(), TokenType::True);
        keywords.insert("false".into(), TokenType::False);
        keywords.insert("return".into(), TokenType::Return);
        keywords.insert("plus".into(), TokenType::Plus);
        keywords.insert("minus".into(), TokenType::Minus);
        keywords.insert("declare".into(), TokenType::Declare);
        keywords.insert("function".into(), TokenType::Function);
        keywords.insert("show".into(), TokenType::Show);
        keywords.insert("end".into(), TokenType::End);
        keywords.insert("enter".into(), TokenType::Enter);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            keywords,
            prev_c: '\0',
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".into(),
            "".into(),
            self.line,
            self.column,
        ));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ':' => self.add_token(TokenType::Colon),
            '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            ' ' | '\r' | '\t' | '\n' => {}
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if self.alpha(c) {
                    self.identifier();
                } else {
                    error(
                        Span {
                            line: self.line,
                            column: self.column,
                        },
                        "Unexpected character.",
                    );
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, "".into());
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current].to_string();

        let token_column = self.column - text.chars().count();

        self.tokens.push(Token::new(
            token_type,
            text,
            literal,
            self.line,
            token_column.max(1),
        ));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.advance();
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current..].chars().next().unwrap()
        }
    }

    fn peek_next(&self) -> char {
        let mut chars = self.source[self.current..].chars();
        chars.next();
        chars.next().unwrap_or('\0')
    }

    fn alpha(&mut self, c: char) -> bool {
        let output =
            (c.is_ascii_alphabetic() || c == '_' || c == '¬') && !(c == '_' && self.prev_c == '_');
        self.prev_c = c;
        output
    }

    fn alpha_numeric(&mut self, c: char) -> bool {
        self.alpha(c) || c.is_ascii_digit()
    }

    fn string(&mut self) {
        let mut value = String::new();

        while !self.is_at_end() {
            let c = self.advance();

            match c {
                '"' => {
                    // End of string
                    self.add_token_literal(TokenType::String, value);
                    return;
                }
                '\\' => {
                    if self.is_at_end() {
                        error(
                            Span {
                                line: self.line,
                                column: self.column,
                            },
                            "Unterminated escape sequence in string.",
                        );
                        return;
                    }

                    let esc = self.advance();
                    match esc {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        _ => {
                            error(
                                Span {
                                    line: self.line,
                                    column: self.column,
                                },
                                &format!("Invalid escape sequence: \\{}", esc),
                            );
                            return;
                        }
                    }
                }
                _ => value.push(c),
            }
        }

        error(
            Span {
                line: self.line,
                column: self.column,
            },
            "Unterminated string literal.",
        );
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
            let value = self.source[self.start..self.current].to_string();
            self.add_token_literal(TokenType::Float, value);
        } else {
            let value = self.source[self.start..self.current].to_string();
            self.add_token_literal(TokenType::Int, value);
        }
    }

    fn identifier(&mut self) {
        while self.alpha_numeric(self.peek()) {
            self.advance();
        }

        // Handle '::' type sequences like 'str::i32'
        while self.peek() == ':' && self.peek_next() == ':' {
            self.advance(); // consume first ':'
            self.advance(); // consume second ':'

            // continue scanning the next identifier part
            while self.alpha_numeric(self.peek()) {
                self.advance();
            }
        }

        let text = self.source[self.start..self.current]
            .to_string()
            .to_lowercase();
        if let Some(token_type) = self.keywords.get(&text) {
            self.add_token(*token_type);
        } else {
            self.add_token(TokenType::Ident);
        }
    }

    fn character(&mut self) {
        if self.is_at_end() {
            error(
                Span {
                    line: self.line,
                    column: self.column,
                },
                "Unterminated character literal.",
            );
            return;
        }

        let c = self.advance();
        let value = if c == '\\' {
            if self.is_at_end() {
                error(
                    Span {
                        line: self.line,
                        column: self.column,
                    },
                    "Unterminated escape sequence.",
                );
                return;
            }
            let esc = self.advance();
            match esc {
                'n' => "\n".to_string(),
                't' => "\t".to_string(),
                '\\' => "\\".to_string(),
                '\'' => "'".to_string(),
                'r' => "\r".to_string(),
                _ => {
                    error(
                        Span {
                            line: self.line,
                            column: self.column,
                        },
                        &format!("Invalid escape sequence: \\{}", esc),
                    );
                    return;
                }
            }
        } else {
            c.to_string()
        };

        if self.peek() != '\'' {
            error(
                Span {
                    line: self.line,
                    column: self.column,
                },
                "Character literal too long or missing closing quote.",
            );
            return;
        }

        self.advance(); // Consume closing quote
        self.add_token_literal(TokenType::Char, value);
    }

    fn block_comment(&mut self) {
        while !self.is_at_end() {
            if self.peek() == '*' {
                self.advance();
                if self.peek() == '/' {
                    self.advance();
                    return;
                }
            } else {
                self.advance();
            }
        }

        error(
            Span {
                line: self.line,
                column: self.column,
            },
            "Unterminated block comment.",
        );
    }
}
