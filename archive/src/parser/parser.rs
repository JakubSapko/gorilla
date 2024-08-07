use crate::{
    ast::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement},
    token::token::{Lexer, Token, TokenType},
};

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser<'_> {
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(), // The type is now Vec<Box<dyn StatementNode>>
        };
        while self.cur_token.Type != TokenType::EOF {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                program.statements.push(s);
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
            TokenType::RETURN => {
                if let Some(stmt) = self.parse_return_statement() {
                    return Some(Statement::Return(stmt));
                }
                return None;
            }
            _ => return None,
        }
    }

    pub fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let tok = &self.cur_token.clone();
        self.next_token();
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        let stmt = ReturnStatement {
            token: tok.clone(),
            return_value: Expression::Identifier(Identifier {
                token: Token {
                    Type: TokenType::RETURN,
                    Literal: "".to_string(),
                },
                value: "".to_string(),
            }),
        };
        Some(stmt)
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let tok = &self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT {
            name: tok.to_string(),
        }) {
            return None;
        }
        let id = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.Literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        let val: Expression = Expression::Identifier(Identifier {
            token: Token {
                Type: TokenType::ILLEGAL,
                Literal: "".to_string(),
            },
            value: "".to_string(),
        });
        let stmt = LetStatement {
            token: tok.clone(),
            name: id,
            value: val,
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
            self.peek_error(t);
            return false;
        }
    }

    pub fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.Type
        );
        self.errors.push(msg);
    }
}

pub fn new_parser(lexer: &mut Lexer) -> Parser {
    let mut p = Parser {
        l: lexer,
        cur_token: Default::default(),
        peek_token: Default::default(),
        errors: Vec::new(),
    };
    p.next_token();
    p.next_token();
    return p;
}
