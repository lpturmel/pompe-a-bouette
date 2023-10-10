use crate::ast::{self, LetStatement, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    pub token_count: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Self {
            lexer,
            cur_token: Token::new(TokenType::EOF, 0, 0),
            peek_token: Token::new(TokenType::EOF, 0, 0),
            errors: Vec::new(),
            token_count: 0,
        };
        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn peek_error(&mut self, token_type: &TokenType) {
        let msg = format!(
            "expected next token to be '{}', got '{}' instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
    fn next_token(&mut self) {
        std::mem::swap(&mut self.cur_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
        self.token_count += 1;
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

    pub fn parse(&mut self) -> ast::Program {
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
    fn parse_stmt(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self
                .parse_let_stmt()
                .map(|stmt| Statement::Let(Box::new(stmt))),
            _ => None,
        }
    }
    fn parse_let_stmt(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        let is_mut = self.peek_token_is(&TokenType::Mut);

        if is_mut {
            self.next_token();
        }

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = ast::Identifier::new(
            self.cur_token.clone(),
            self.cur_token.start,
            self.cur_token.end,
        );

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(LetStatement::new(
            token, name, is_mut,
            // Box::new(ast::Identifier::default()),
        ))
    }
}

#[cfg(test)]
pub mod test {

    #[test]
    fn parse_let_stmt() {
        let input = r#" let x = 5; let y = 10; let a = 838383; "#;
        let lexer = crate::lexer::Lexer::new(input);
        let mut parser = super::Parser::new(lexer);
        let p = parser.parse();
        if !parser.errors.is_empty() {
            println!("parser has {} errors", parser.errors.len());
            for error in parser.errors {
                println!("parser error: {}", error);
            }
            panic!();
        }
        assert_eq!(p.statements.len(), 3);
    }
    #[test]
    fn parse_mut_let_stmt() {
        let input = r#"
        let x = 3;
        let mut y = 5;
        y = 10;
        "#;
        let lexer = crate::lexer::Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let p = parser.parse();

        if !parser.errors.is_empty() {
            println!("parser has {} errors", parser.errors.len());
            for error in parser.errors {
                println!("parser error: {}", error);
            }
            panic!();
        }

        let mut stmts = p.statements.iter();
        let first_stmt = stmts.next().unwrap();
        let sec_stmt = stmts.next().unwrap();

        assert!(!first_stmt.is_mut());
        assert!(sec_stmt.is_mut());
    }
    #[test]
    fn parse_from_file() {
        let input = std::fs::read_to_string("input/large.pab").unwrap();

        let lexer = crate::lexer::Lexer::new(&input);
        let mut parser = super::Parser::new(lexer);

        let p = parser.parse();

        if !parser.errors.is_empty() {
            println!("parser has {} errors", parser.errors.len());
            for error in parser.errors {
                println!("parser error: {}", error);
            }
            panic!();
        }

        assert_eq!(p.statements.len(), 518400);
    }
}
