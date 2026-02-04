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
    This,

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
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::LeftBrack => "LeftBrack",
            TokenType::RightBrack => "RightBrack",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Mod => "Mod",
            TokenType::And => "And",
            TokenType::Or => "Or",
            TokenType::Pound => "Pound",
            TokenType::NotSign => "NotSign",
            TokenType::This => "This",

            // One or two character tokens
            TokenType::Star => "Star",
            TokenType::StarStar => "StarStar",
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",
            TokenType::At => "At",
            TokenType::Hash => "Hash",
            TokenType::UpArrow => "UpArrow",
            TokenType::DoubleUpArrow => "DoubleUpArrow",
            TokenType::Dollar => "Dollar",
            TokenType::DollarQuestionMark => "DollarQuestionMark",
            TokenType::QuestionMark => "QuestionMark",
            TokenType::Tilde => "Tilde",
            TokenType::TildeQuestionMark => "TildeQuestionMark",
            TokenType::Colon => "Colon",
            TokenType::DoubleColon => "DoubleColon",
            TokenType::Minus => "Minus",
            TokenType::MinusRight => "MinusRight",

            // Literals
            TokenType::Ident => "Ident",
            TokenType::String => "String",
            TokenType::Int => "Int",
            TokenType::Float => "Float",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Char => "Char",

            // Keywords
            TokenType::Cls => "Cls",
            TokenType::Ret => "Ret",
            TokenType::Comp => "Comp",
            TokenType::Stc => "Stc",
            TokenType::Ovr => "Ovr",
            TokenType::Err => "Err",
            TokenType::Del => "Del",
            TokenType::Use => "Use",
            TokenType::For => "For",
            TokenType::Fn => "Fn",

            TokenType::Nil => "Nil",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", s)
    }
}
