#[derive(Debug)]
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
            _ => Self::new(TokenType::Ident(ident.to_string()), ident.to_string()),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Misc
    Illegal,
    EOF,
    Ident(String),
    Int(i64),
    Float(f64),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
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
