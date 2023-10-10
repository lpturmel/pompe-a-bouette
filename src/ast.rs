use crate::token;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(Box<LetStatement>),
}

impl Statement {
    pub fn is_mut(&self) -> bool {
        match self {
            Statement::Let(let_statement) => let_statement.is_mut,
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct LetStatement {
    token: token::Token,
    name: Identifier,
    is_mut: bool,
    // value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: token::Token, name: Identifier, is_mut: bool) -> Self {
        Self {
            token,
            name,
            is_mut,
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    token: token::Token,
    start: usize,
    end: usize,
}

impl Identifier {
    pub fn new(token: token::Token, start: usize, end: usize) -> Self {
        Self { token, start, end }
    }
}
