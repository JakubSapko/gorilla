use crate::{
    ast::ast::Program,
    token::token::{Lexer, Token},
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

    pub fn parse_program(&self) -> Option<Program> {
        return None::<Program>;
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
