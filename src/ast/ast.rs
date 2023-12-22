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

struct LetStatement<'a> {
    Token: Token, //token.LET
    Name: &'a Identifier,
    Value: Box<dyn Expression>,
}

impl Node for LetStatement<'_> {
    fn TokenLiteral(&self) -> String {
        return self.Token.Literal.to_string();
    }
}

impl Statement for LetStatement<'_> {
    fn statement_node(&self) {}
}

struct Identifier {
    Token: Token,
    Value: String,
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
