use clap::Parser;
use lexer::Lexer;
use thiserror::Error as ThisError;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

/// Pompe a Bouette toy programming language
#[derive(Parser, Debug)]
#[clap(author = "Louis-Philippe Turmel", version, about, long_about = None)]
pub struct Cli {
    input_file: Option<String>,
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

    if let Some(input_file) = cli.input_file {
        let input = std::fs::read_to_string(input_file)?;

        let lexer = Lexer::new(&input);
        let mut parser = parser::Parser::new(lexer);
        let now = std::time::Instant::now();
        let _ = parser.parse();
        println!(
            "parsing {} tokens took {:?}",
            parser.token_count,
            now.elapsed()
        );
    } else {
        let mut repl = repl::Repl::new();
        repl.start();
    }
    Ok(())
}
