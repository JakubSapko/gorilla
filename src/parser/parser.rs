use std::default;

use crate::{
    ast::ast::{Expression, Identifier, LetStatement, Program, Statement, StatementNode},
    token::token::{Lexer, Token, TokenType},
};

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            Statements: Vec::new(), // The type is now Vec<Box<dyn StatementNode>>
        };
        while self.cur_token.Type != TokenType::EOF {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                program.Statements.push(s);
            }
            self.next_token();
        }
        return program;
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.Type {
            TokenType::LET => {
                if let Some(stmt) = self.parse_let_statement() {
                    return Some(Statement::Let(stmt));
                }
                return None;
            }
            _ => return None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let tok = &self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        let id = Identifier {
            Token: self.cur_token.clone(),
            Value: self.cur_token.Literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        let val: Box<dyn Expression> = Box::new(Identifier {
            Token: Token {
                Type: TokenType::ILLEGAL,
                Literal: "".to_string(),
            },
            Value: "".to_string(),
        });
        let stmt = LetStatement {
            Token: tok.clone(),
            Name: id,
            Value: val,
        };
        Some(stmt)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        return self.cur_token.Type == t;
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        return self.peek_token.Type == t;
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

pub fn new_parser(lexer: &mut Lexer) -> Parser {
    let mut p = Parser {
        l: lexer,
        cur_token: Default::default(),
        peek_token: Default::default(),
    };
    p.next_token();
    p.next_token();
    return p;
}
