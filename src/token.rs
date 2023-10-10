use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, start: usize, end: usize, line: usize) -> Self {
        Self {
            token_type,
            start,
            end,
            line,
        }
    }
    /// Lookup an identifier and return the corresponding token type
    pub fn lookup_ident(ident: &str, start: usize, end: usize, line: usize) -> Self {
        let token_type = match ident {
            "fn" => TokenType::Fn,
            "let" => TokenType::Let,
            "mut" => TokenType::Mut,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident,
        };
        Self::new(token_type, start, end, line)
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
