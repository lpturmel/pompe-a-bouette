use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
        }
    }
    /// Lookup an identifier and return the corresponding token type
    pub fn lookup_ident(ident: &str) -> Self {
        match ident {
            "fn" => Self::new(TokenType::Fn, ident),
            "let" => Self::new(TokenType::Let, ident),
            "mut" => Self::new(TokenType::Mut, ident),
            "true" => Self::new(TokenType::True, ident),
            "false" => Self::new(TokenType::False, ident),
            "if" => Self::new(TokenType::If, ident),
            "else" => Self::new(TokenType::Else, ident),
            "return" => Self::new(TokenType::Return, ident),
            _ => Self::new(TokenType::Ident, ident),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Misc
    Illegal,
    EOF,
    Ident,
    Int,
    Float,
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    EQ,
    NotEQ,
    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Fn,
    Let,
    Mut,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            // Misc
            Self::Illegal => "Illegal",
            Self::EOF => "EOF",
            Self::Ident => "Ident",
            Self::Int => "Int",
            Self::Float => "Float",
            // Operators
            Self::Assign => "=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Bang => "!",
            Self::Asterisk => "*",
            Self::Slash => "/",
            Self::LT => "<",
            Self::GT => ">",
            Self::EQ => "==",
            Self::NotEQ => "!=",
            // Delimiters
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::LParen => "(",
            Self::RParen => ")",
            Self::LBrace => "{",
            Self::RBrace => "}",
            // Keywords
            Self::Fn => "fn",
            Self::Let => "let",
            Self::Mut => "mut",
            Self::True => "true",
            Self::False => "false",
            Self::If => "if",
            Self::Else => "else",
            Self::Return => "return",
        };
        write!(f, "{}", s)
    }
}
