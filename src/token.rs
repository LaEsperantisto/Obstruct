use crate::token_type::TokenType;
use crate::token_type::TokenType::NIL;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: isize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: isize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn to_string(&self) -> String {
        (self.token_type.to_string() + " " + self.lexeme.as_str() + " " + self.literal.as_str())
            .to_string()
    }

    pub fn nil() -> Self {
        Self {
            token_type: NIL,
            lexeme: String::new(),
            literal: String::new(),
            line: -1,
        }
    }
}
