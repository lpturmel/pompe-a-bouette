use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or('\0')
    }

    /// Read a number and return the appropriate token
    ///
    /// This function will determine if the number is an integer or a float
    fn read_number(&mut self) -> Token {
        let position = self.position;
        let mut is_float = false;

        while self.ch.is_ascii_digit() || self.ch == '.' {
            if self.ch == '.' {
                if is_float {
                    break;
                }
                is_float = true;
            }
            self.read_char();
        }

        let literal = &self.input[position..self.position];

        let token_type = if is_float {
            TokenType::Float(literal.parse().unwrap())
        } else {
            TokenType::Int(literal.parse().unwrap())
        };

        Token::new(token_type, literal.to_string())
    }

    /// Consume the input and return the next token
    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::EQ, "==".to_string())
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEQ, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, self.ch.to_string())
                }
            }
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            '(' => Token::new(TokenType::LParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RParen, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '+' => Token::new(TokenType::Plus, self.ch.to_string()),
            '-' => Token::new(TokenType::Minus, self.ch.to_string()),
            '/' => Token::new(TokenType::Slash, self.ch.to_string()),
            '*' => Token::new(TokenType::Asterisk, self.ch.to_string()),
            '<' => Token::new(TokenType::LT, self.ch.to_string()),
            '>' => Token::new(TokenType::GT, self.ch.to_string()),
            '{' => Token::new(TokenType::LBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RBrace, self.ch.to_string()),
            '\0' => Token::new(TokenType::EOF, "".to_string()),
            _ => {
                if self.is_letter() {
                    return self.read_identifier();
                } else if self.is_number() {
                    return self.read_number();
                } else {
                    return Token::new(TokenType::Illegal, "".to_string());
                }
            }
        };
        self.read_char();
        tok
    }
    /// Checks if the current character is a number
    fn is_number(&self) -> bool {
        self.ch.is_ascii_digit() || self.ch == '.'
    }

    /// Consume all whitespace characters
    fn consume_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
    /// Checks if the current character is an alphabetic character or an underscore
    fn is_letter(&self) -> bool {
        self.ch.is_alphabetic() || self.ch == '_'
    }
    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        let literal = &self.input[position..self.position];
        Token::lookup_ident(literal)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tokens = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);
        for (expected_type, expected_literal) in tokens {
            let tok = l.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
    #[test]
    fn test_assignment() {
        let input = "let five = 5;";
        let tokens = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident("five".to_string()), "five"),
            (TokenType::Assign, "="),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);
        for (expected_type, expected_literal) in tokens {
            let tok = l.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
    #[test]
    fn test_from_file() {
        let input = std::fs::read_to_string("input/nb.pab").unwrap();
        let tokens = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident("ten".to_string()), "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Mut, "mut"),
            (TokenType::Ident("twenty".to_string()), "twenty"),
            (TokenType::Assign, "="),
            (TokenType::Int(20), "20"),
            (TokenType::Semicolon, ";"),
            (TokenType::Ident("twenty".to_string()), "twenty"),
            (TokenType::Assign, "="),
            (TokenType::Int(30), "30"),
            (TokenType::Semicolon, ";"),
        ];
        let mut l = Lexer::new(&input);
        for (expected_type, expected_literal) in tokens {
            let tok = l.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
    #[test]
    fn test_float_assignment() {
        let input = "let five_dot_zero = 5.0;";
        let tokens = vec![
            (TokenType::Let, "let"),
            (
                TokenType::Ident("five_dot_zero".to_string()),
                "five_dot_zero",
            ),
            (TokenType::Assign, "="),
            (TokenType::Float(5.0), "5.0"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);
        for (expected_type, expected_literal) in tokens {
            let tok = l.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
    #[test]
    fn test_long_assignment() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10;


if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;
        "#;
        let tokens = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident("five".to_string()), "five"),
            (TokenType::Assign, "="),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident("ten".to_string()), "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident("add".to_string()), "add"),
            (TokenType::Assign, "="),
            (TokenType::Fn, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident("x".to_string()), "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident("y".to_string()), "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident("x".to_string()), "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident("y".to_string()), "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident("result".to_string()), "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident("add".to_string()), "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident("five".to_string()), "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident("ten".to_string()), "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int(5), "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int(5), "5"),
            (TokenType::LT, "<"),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int(5), "5"),
            (TokenType::LT, "<"),
            (TokenType::Int(10), "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int(10), "10"),
            (TokenType::EQ, "=="),
            (TokenType::Int(10), "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int(10), "10"),
            (TokenType::NotEQ, "!="),
            (TokenType::Int(9), "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];
        let mut l = Lexer::new(input);
        for (expected_type, expected_literal) in tokens {
            let tok = l.next_token();
            assert_eq!(tok.token_type, expected_type);
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
