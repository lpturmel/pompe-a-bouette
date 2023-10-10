use crate::token;
use std::fmt::Debug;

pub trait Node: Debug {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn is_mut(&self) -> bool;
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Default, Debug)]
pub struct Program<'a> {
    pub statements: Vec<Box<dyn Statement + 'a>>,
}

impl Program<'_> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Node for Program<'_> {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            ""
        }
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

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn is_mut(&self) -> bool {
        self.is_mut
    }
}

#[derive(Debug)]
pub struct Identifier {
    token: token::Token,
    value: String,
}

impl Identifier {
    pub fn new(token: token::Token, value: &str) -> Self {
        Self {
            token,
            value: value.to_string(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
