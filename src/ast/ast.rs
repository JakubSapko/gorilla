use std::fmt::Debug;

use crate::token::token::Token;

pub trait Node {
    fn TokenLiteral(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
}

pub trait StatementNode {
    fn statement_node(&self);
}

impl StatementNode for Statement {
    fn statement_node(&self) {}
}

impl Node for Statement {
    fn TokenLiteral(&self) -> String {
        match self {
            Statement::Let(l) => l.TokenLiteral(),
        }
    }
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Default)]
pub struct Program {
    pub Statements: Vec<Statement>,
}

pub struct LetStatement {
    pub Token: Token, //token.LET
    pub Name: Identifier,
    pub Value: Box<dyn Expression>,
}

impl Debug for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LetStatement {{ Token: {:?}, Name: {:?}, Value: }}",
            self.Token, self.Name
        )
    }
}

impl StatementNode for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn TokenLiteral(&self) -> String {
        return self.Token.Literal.to_string();
    }
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
