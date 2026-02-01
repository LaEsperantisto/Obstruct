use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_BRACK,
    RIGHT_BRACK,
    COMMA,
    DOT,
    PLUS,
    SEMICOLON,
    SLASH,
    MOD,
    AND,
    OR,
    POUND,

    // One or two character tokens
    STAR,
    STAR_STAR,
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    AT,
    HASH_AT,
    HASH,
    UP_ARROW,
    DOUBLE_UP_ARROW,
    DOLLAR,
    DOLLAR_QUESTION_MARK,
    QUESTION_MARK,
    TILDE,
    TILDE_QUESTION_MARK,
    COLON,
    DOUBLE_COLON,
    MINUS,
    MINUS_RIGHT,

    // Literals
    IDENTIFIER,
    STRING,
    INT,
    FLOAT,
    TRUE,
    FALSE,
    CHAR,

    // Keywords
    CLS,
    RET,
    COMP,
    STC,
    OVR,
    EXIT,
    ERR,
    DEL,
    USE,
    FOR,

    NIL, // this gives an error - not supposed to be fetched - interpreter badly programmed
    EOF, // End Of File
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // Single-character tokens
            TokenType::LEFT_PAREN => "LEFT_PAREN",
            TokenType::RIGHT_PAREN => "RIGHT_PAREN",
            TokenType::LEFT_BRACE => "LEFT_BRACE",
            TokenType::RIGHT_BRACE => "RIGHT_BRACE",
            TokenType::LEFT_BRACK => "LEFT_BRACK",
            TokenType::RIGHT_BRACK => "RIGHT_BRACK",
            TokenType::COMMA => "COMMA",
            TokenType::DOT => "DOT",
            TokenType::PLUS => "PLUS",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::SLASH => "SLASH",
            TokenType::MOD => "MOD",
            TokenType::AND => "AND",
            TokenType::OR => "OR",
            TokenType::POUND => "POUND",

            // One or two character tokens
            TokenType::STAR => "STAR",
            TokenType::STAR_STAR => "STAR_STAR",
            TokenType::BANG => "BANG",
            TokenType::BANG_EQUAL => "BANG_EQUAL",
            TokenType::EQUAL => "EQUAL",
            TokenType::EQUAL_EQUAL => "EQUAL_EQUAL",
            TokenType::GREATER => "GREATER",
            TokenType::GREATER_EQUAL => "GREATER_EQUAL",
            TokenType::LESS => "LESS",
            TokenType::LESS_EQUAL => "LESS_EQUAL",
            TokenType::AT => "AT",
            TokenType::HASH_AT => "HASH_AT",
            TokenType::HASH => "HASH",
            TokenType::UP_ARROW => "UP_ARROW",
            TokenType::DOUBLE_UP_ARROW => "DOUBLE_UP_ARROW",
            TokenType::DOLLAR => "DOLLAR",
            TokenType::DOLLAR_QUESTION_MARK => "DOLLAR_QUESTION_MARK",
            TokenType::QUESTION_MARK => "QUESTION_MARK",
            TokenType::TILDE => "TILDE",
            TokenType::TILDE_QUESTION_MARK => "TILDE_QUESTION_MARK",
            TokenType::COLON => "COLON",
            TokenType::DOUBLE_COLON => "DOUBLE_COLON",
            TokenType::MINUS => "MINUS",
            TokenType::MINUS_RIGHT => "MINUS_RIGHT",

            // Literals
            TokenType::IDENTIFIER => "IDENTIFIER",
            TokenType::STRING => "STRING",
            TokenType::INT => "INT",
            TokenType::FLOAT => "FLOAT",
            TokenType::TRUE => "TRUE",
            TokenType::FALSE => "FALSE",
            TokenType::CHAR => "CHAR",

            // Keywords
            TokenType::CLS => "CLS",
            TokenType::RET => "RET",
            TokenType::COMP => "COMP",
            TokenType::STC => "STC",
            TokenType::OVR => "OVR",
            TokenType::EXIT => "EXIT",
            TokenType::ERR => "ERR",
            TokenType::DEL => "DEL",
            TokenType::USE => "USE",
            TokenType::FOR => "FOR",

            TokenType::NIL => "NIL",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", s)
    }
}
