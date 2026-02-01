use crate::error;
use crate::token::Token;
use crate::token_type::TokenType;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: isize,
    keywords: HashMap<String, TokenType>,
    prev_c: char,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert("cls".into(), TokenType::CLS);
        keywords.insert("ret".into(), TokenType::RET);
        keywords.insert("comp".into(), TokenType::COMP);
        keywords.insert("stc".into(), TokenType::STC);
        keywords.insert("ovr".into(), TokenType::OVR);
        keywords.insert("exit".into(), TokenType::EXIT);
        keywords.insert("err".into(), TokenType::ERR);
        keywords.insert("del".into(), TokenType::DEL);
        keywords.insert("use".into(), TokenType::USE);
        keywords.insert("for".into(), TokenType::FOR);
        keywords.insert("fn".into(), TokenType::FN);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
            prev_c: '\0',
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".into(), "".into(), self.line));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            '[' => self.add_token(TokenType::LEFT_BRACK),
            ']' => self.add_token(TokenType::RIGHT_BRACK),
            '&' => self.add_token(TokenType::AND),
            '|' => self.add_token(TokenType::OR),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '%' => self.add_token(TokenType::MOD),
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenType::MINUS_RIGHT);
                } else {
                    self.add_token(TokenType::MINUS);
                }
            }
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            ':' => {
                if self.match_char(':') {
                    self.add_token(TokenType::DOUBLE_COLON);
                } else {
                    self.add_token(TokenType::COLON);
                }
            }
            '*' => {
                if self.match_char('*') {
                    self.add_token(TokenType::STAR_STAR);
                } else {
                    self.add_token(TokenType::STAR);
                }
            }
            '~' => {
                if self.match_char('?') {
                    self.add_token(TokenType::TILDE_QUESTION_MARK);
                } else {
                    self.add_token(TokenType::TILDE);
                }
            }
            '?' => self.add_token(TokenType::QUESTION_MARK),
            '$' => {
                if self.match_char('?') {
                    self.add_token(TokenType::DOLLAR_QUESTION_MARK);
                } else {
                    self.add_token(TokenType::DOLLAR);
                }
            }
            '^' => {
                if self.match_char('^') {
                    self.add_token(TokenType::DOUBLE_UP_ARROW);
                } else {
                    self.add_token(TokenType::UP_ARROW);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '#' => {
                self.add_token(TokenType::HASH);
            }
            '@' => self.add_token(TokenType::AT),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('=') {
                    while self.peek() != '=' && !self.is_at_end() && self.peek_next() != '\\' {
                        self.advance();
                    }
                    self.advance();
                    self.advance();
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '`' => self.backtick(),
            '"' => self.string(),
            '\'' => self.character(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    error(self.line + 1, "Unexpected character.");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, "".into());
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: String) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current..].chars().next().unwrap() != expected {
            return false;
        }
        self.current += expected.len_utf8();
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
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1..].chars().next().unwrap()
        }
    }

    fn is_alpha(&mut self, c: char) -> bool {
        let output = (c.is_ascii_alphabetic() || c == '_') && !(c == '_' && self.prev_c == '_');
        self.prev_c = c;
        output
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
    }

    fn string(&mut self) {
        let mut value = String::new();

        while !self.is_at_end() {
            let c = self.advance();

            match c {
                '"' => {
                    // End of string
                    self.add_token_literal(TokenType::STRING, value);
                    return;
                }
                '\\' => {
                    if self.is_at_end() {
                        error(self.line, "Unterminated escape sequence in string.");
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
                            error(self.line, &format!("Invalid escape sequence: \\{}", esc));
                            return;
                        }
                    }
                }
                '\n' => {
                    self.line += 1;
                    value.push('\n');
                }
                _ => value.push(c),
            }
        }

        error(self.line, "Unterminated string literal.");
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
            self.add_token_literal(TokenType::FLOAT, value);
        } else {
            let value = self.source[self.start..self.current].to_string();
            self.add_token_literal(TokenType::INT, value);
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        if let Some(token_type) = self.keywords.get(&text) {
            self.add_token(*token_type);
        } else {
            self.add_token(TokenType::IDENTIFIER);
        }
    }

    fn character(&mut self) {
        if self.is_at_end() {
            error(self.line, "Unterminated character literal.");
            return;
        }

        let c = self.advance();
        let value = if c == '\\' {
            if self.is_at_end() {
                error(self.line, "Unterminated escape sequence.");
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
                    error(self.line, &format!("Invalid escape sequence: \\{}", esc));
                    return;
                }
            }
        } else {
            c.to_string()
        };

        if self.peek() != '\'' {
            error(
                self.line,
                "Character literal too long or missing closing quote.",
            );
            return;
        }

        self.advance(); // Consume closing quote
        self.add_token_literal(TokenType::CHAR, value);
    }

    fn backtick(&mut self) {
        if self.is_at_end() {
            error(self.line, "Expected character after backtick (`)");
            return;
        }

        let c = self.advance();
        match c {
            'f' => self.add_token(TokenType::FALSE),
            't' => self.add_token(TokenType::TRUE),
            _ => {
                error(self.line, "Invalid character after backtick (`)");
                return;
            }
        }

        if self.is_alpha(self.peek()) {
            error(
                self.line,
                "Only a single character should be after a backtick (`)",
            );
        }
    }
}
