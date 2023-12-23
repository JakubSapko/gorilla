use crate::token::token::Token;

pub trait Node {
    fn TokenLiteral(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub Statements: Vec<Box<dyn Statement>>,
}

pub struct LetStatement<'a> {
    pub Token: Token, //token.LET
    pub Name: &'a Identifier,
    pub Value: Box<dyn Expression>,
}

impl Node for LetStatement<'_> {
    fn TokenLiteral(&self) -> String {
        return self.Token.Literal.to_string();
    }
}

impl Statement for LetStatement<'_> {
    fn statement_node(&self) {}
}

#[derive(Debug)]
pub struct Identifier {
    pub Token: Token,
    pub Value: String,
}

impl Node for Identifier {
    fn TokenLiteral(&self) -> String {
        return self.Token.Literal.to_string();
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Program {
    fn TokenLiteral(&self) -> String {
        if self.Statements.len() > 0 {
            return self.Statements[0].TokenLiteral();
        } else {
            return "".to_string();
        }
    }
}
