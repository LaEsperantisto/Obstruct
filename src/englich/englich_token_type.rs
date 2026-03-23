use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    Comma,
    Dot,
    Colon,

    // One or two character tokens

    // Literals
    Ident,
    String,
    Int,
    Float,
    Char,

    // Keywords
    Let,
    Be,
    Set,
    To,
    True,
    False,
    Return,
    Block,
    Plus,
    Minus,
    Declare,
    Function,
    Show,
    End,
    Enter,
    Call,

    Nil, // this gives an error - not supposed to be fetched - interpreter badly programmed
    EOF, // End Of File
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // Single-character tokens
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Colon => "Colon",

            // Literals
            TokenType::Ident => "Ident",
            TokenType::String => "String",
            TokenType::Int => "Int",
            TokenType::Float => "Float",
            TokenType::Char => "Char",

            // Keywords
            TokenType::Let => "Let",
            TokenType::Be => "Be",
            TokenType::Set => "Set",
            TokenType::To => "To",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Return => "Return",
            TokenType::Block => "Block",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Declare => "Declare",
            TokenType::Function => "Function",
            TokenType::Show => "Show",
            TokenType::End => "End",
            TokenType::Enter => "Enter",
            TokenType::Call => "Call",

            TokenType::Nil => "Nil",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", s)
    }
}
