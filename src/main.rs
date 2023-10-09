use clap::Parser;
use lexer::Lexer;
use thiserror::Error as ThisError;

pub mod lexer;
pub mod token;

/// Pompe a Bouette toy programming language
#[derive(Parser, Debug)]
#[clap(author = "Louis-Philippe Turmel", version, about, long_about = None)]
pub struct Cli {
    input_file: String,
}

#[derive(Debug, ThisError)]
enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(cli.input_file)?;

    let mut lexer = Lexer::new(&input);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token.token_type == token::TokenType::EOF {
            break;
        }
    }
    Ok(())
}
