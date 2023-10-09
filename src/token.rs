use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
    /// Lookup an identifier and return the corresponding token type
    pub fn lookup_ident(ident: &str) -> Self {
        match ident {
            "fn" => Self::new(TokenType::Fn, ident.to_string()),
            "let" => Self::new(TokenType::Let, ident.to_string()),
            "mut" => Self::new(TokenType::Mut, ident.to_string()),
            "true" => Self::new(TokenType::True, ident.to_string()),
            "false" => Self::new(TokenType::False, ident.to_string()),
            "if" => Self::new(TokenType::If, ident.to_string()),
            "else" => Self::new(TokenType::Else, ident.to_string()),
            "return" => Self::new(TokenType::Return, ident.to_string()),
            _ => Self::new(TokenType::Ident, ident.to_string()),
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
