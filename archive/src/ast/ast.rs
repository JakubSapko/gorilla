use std::fmt::{Debug, Display, Formatter};

use crate::token::token::{Token, TokenType};

pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

pub enum Expression {
    Identifier(Identifier),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expression")
    }
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

pub struct LetStatement {
    pub token: Token, //token.LET
    pub name: Identifier,
    pub value: Expression,
}

pub struct ReturnStatement {
    pub token: Token, //token.RETURN
    pub return_value: Expression,
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(LetStatement {
                token: id,
                value: expr,
                ..
            }) => {
                if let TokenType::IDENT { name } = &id.Type {
                    return write!(f, "let {} = {};", name, expr);
                }
                panic!("unreachable")
            }
            Statement::Return(ReturnStatement {
                return_value: argument,
                ..
            }) => {
                write!(f, "return {};", argument)
            }
            Statement::Expression(expr) => write!(f, "{}", expr),
        }
    }
}
impl Debug for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ExpressionStatement {{ Token: {:?}, Expression: }}",
            self.Token
        )
    }
}

impl Debug for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReturnStatement {{ Token: {:?}, ReturnValue: }}",
            self.Token
        )
    }
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
#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Program {
    fn TokenLiteral(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].to_string();
        } else {
            return "".to_string();
        }
    }

    pub fn String(&self) -> String {
        let mut out = String::new();
        let mut i = 0;
        println!("{:?}", self.statements);
        for s in &self.statements {
            out.push_str(&s.to_string());
            println!("{}, {}", i, out);
            i += 1;
        }
        return out;
    }
}

impl Identifier {
    fn token_literal(&self) -> String {
        return self.token.Literal.to_string();
    }

    fn string(&self) -> String {
        return self.value.to_string();
    }
}
