use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBrack,
    RightBrack,
    Comma,
    Dot,
    Plus,
    Semicolon,
    Slash,
    Mod,
    And,
    Or,
    Pound,
    NotSign,

    // One or two character tokens
    Star,
    StarStar,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    At,
    Hash,
    UpArrow,
    DoubleUpArrow,
    Dollar,
    DollarQuestionMark,
    QuestionMark,
    Tilde,
    TildeQuestionMark,
    Colon,
    DoubleColon,
    Minus,
    MinusRight,

    // Literals
    Ident,
    String,
    Int,
    Float,
    True,
    False,
    Char,

    // Keywords
    Cls,
    Ret,
    Comp,
    Stc,
    Ovr,
    Exit,
    Err,
    Del,
    Use,
    For,
    Fn,

    Nil, // this gives an error - not supposed to be fetched - interpreter badly programmed
    EOF, // End Of File
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // Single-character tokens
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::LeftBrack => "LeftBrack",
            TokenType::RightBrack => "RightBrack",
            TokenType::Comma => "COMMA",
            TokenType::Dot => "DOT",
            TokenType::Plus => "PLUS",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Slash => "SLASH",
            TokenType::Mod => "MOD",
            TokenType::And => "AND",
            TokenType::Or => "OR",
            TokenType::Pound => "POUND",
            TokenType::NotSign => "NotSign",

            // One or two character tokens
            TokenType::Star => "STAR",
            TokenType::StarStar => "StarStar",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LessEqual",
            TokenType::At => "AT",
            TokenType::Hash => "HASH",
            TokenType::UpArrow => "UpArrow",
            TokenType::DoubleUpArrow => "DoubleUpArrow",
            TokenType::Dollar => "DOLLAR",
            TokenType::DollarQuestionMark => "DollarQuestionMark",
            TokenType::QuestionMark => "QuestionMark",
            TokenType::Tilde => "TILDE",
            TokenType::TildeQuestionMark => "TildeQuestionMark",
            TokenType::Colon => "COLON",
            TokenType::DoubleColon => "DoubleColon",
            TokenType::Minus => "MINUS",
            TokenType::MinusRight => "MinusRight",

            // Literals
            TokenType::Ident => "IDENTIFIER",
            TokenType::String => "STRING",
            TokenType::Int => "INT",
            TokenType::Float => "FLOAT",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::Char => "CHAR",

            // Keywords
            TokenType::Cls => "CLS",
            TokenType::Ret => "RET",
            TokenType::Comp => "COMP",
            TokenType::Stc => "STC",
            TokenType::Ovr => "OVR",
            TokenType::Exit => "EXIT",
            TokenType::Err => "ERR",
            TokenType::Del => "DEL",
            TokenType::Use => "USE",
            TokenType::For => "FOR",
            TokenType::Fn => "FN",

            TokenType::Nil => "NIL",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", s)
    }
}
