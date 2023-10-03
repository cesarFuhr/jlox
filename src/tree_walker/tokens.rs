use std::fmt::{self, Display};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum TokenType {
    // Single char tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Colon,
    Slash,
    Star,
    Question,

    //  One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Kewords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Final token.
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: u64,
}
