use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{self, BufRead, BufReader, Stdin, Write};

const PROMPT: &str = ">> ";

#[derive(Debug)]
pub struct Repl {
    reader: BufReader<Stdin>,
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            reader: BufReader::new(std::io::stdin()),
        }
    }
}

impl Repl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        let welcome = format!(
            r#"
            Welcome to Pompe a Bouette {}
            This is the interactive REPL.
            "#,
            env!("CARGO_PKG_VERSION")
        );
        println!("{}", welcome);
        loop {
            print!("{}", PROMPT);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            self.reader.read_line(&mut input).unwrap();

            let mut lexer = Lexer::new(&input);
            loop {
                let token = lexer.next_token();
                if token.token_type == TokenType::EOF {
                    break;
                }
                println!("{:?}", token);
            }
        }
    }
}
