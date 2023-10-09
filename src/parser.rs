use crate::ast::{self, LetStatement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut p = Self {
            lexer,
            cur_token: Token::new(TokenType::EOF, "".to_string()),
            peek_token: Token::new(TokenType::EOF, "".to_string()),
            errors: Vec::new(),
        };
        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    fn peek_error(&mut self, token_type: &TokenType) {
        let msg = format!(
            "expected next token to be '{}', got '{}' instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(&token_type);
            false
        }
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.token_type == *token_type
    }

    fn parse(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        while self.cur_token.token_type != TokenType::EOF {
            let stmt = self.parse_stmt();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }
    fn parse_stmt(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_stmt(),
            _ => None,
        }
    }
    fn parse_let_stmt(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = ast::Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(LetStatement::new(
            token, name,
            // Box::new(ast::Identifier::default()),
        )))
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn parse_let_stmt() {
        let input = r#"
        let x = 5;
        let y = 10;
        let a = 838383;
        "#;

        let lexer = &mut crate::lexer::Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let p = parser.parse();

        if !parser.errors.is_empty() {
            println!("parser has {} errors", parser.errors.len());
            for error in parser.errors {
                println!("parser error: {}", error);
            }
            panic!();
        }

        println!("{:?}", p);

        assert_eq!(p.statements.len(), 3);
    }
}
