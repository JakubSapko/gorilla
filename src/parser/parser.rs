use std::default;

use crate::{
    ast::ast::{Expression, Identifier, LetStatement, Program, Statement},
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

    pub fn parse_program(&mut self) {
        let mut program = Program {
            ..Default::default()
        };
        while self.cur_token.Type != TokenType::EOF {
            let stmt = self.parse_statement();
        }
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.Type {
            TokenType::LET => return self.parse_let_statement(),
            _ => return None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
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
            Name: Box::leak(Box::new(id)),
            Value: val,
        };
        return Some(Box::new(stmt));
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
