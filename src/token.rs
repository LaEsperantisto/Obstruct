use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: String,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }

    pub fn nil() -> Self {
        Self {
            token_type: TokenType::Nil,
            lexeme: String::new(),
            literal: String::new(),
            line: 0,
            column: 0,
        }
    }
    pub fn to_string(&self) -> String {
        (self.token_type.to_string() + " " + self.lexeme.as_str() + " " + self.literal.as_str())
            .to_string()
    }
}
