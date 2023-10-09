use crate::token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}

pub struct LetStatement {
    token: token::Token,
    name: Identifier,
    // value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: token::Token, name: Identifier) -> Self {
        Self { token, name }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct Identifier {
    token: token::Token,
    value: String,
}

impl Identifier {
    pub fn new(token: token::Token, value: String) -> Self {
        Self { token, value }
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
