use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{self, BufRead, BufReader, Stdin, Write};
use std::str::FromStr;

const PROMPT: &str = ">> ";

#[derive(Debug)]
enum Commands {
    Exit,
    Help,
}

impl Commands {
    fn handle(&self) {
        match self {
            Self::Exit => {
                println!("Bye! ⛽️");
                std::process::exit(0);
            }
            Self::Help => {
                println!("Commands:");
                println!("exit: Exit the REPL");
                println!("help: Print this help message");
            }
        }
    }
}

impl FromStr for Commands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".exit" => Ok(Self::Exit),
            ".help" => Ok(Self::Help),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

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
            Welcome to Pompe à Bouette {}
            This is the interactive REPL.
            Type .help for more information.
            "#,
            env!("CARGO_PKG_VERSION")
        );
        println!("{}", welcome);
        loop {
            print!("{}", PROMPT);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            self.reader.read_line(&mut input).unwrap();

            if let Ok(command) = input.trim().parse::<Commands>() {
                command.handle();
                continue;
            }

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
