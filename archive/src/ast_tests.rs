#[cfg(test)]
mod tests {
    use crate::ast::ast::{Identifier, LetStatement, Program, Statement};
    use crate::token::token::{Token, TokenType};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                token: Token {
                    Type: TokenType::LET,
                    Literal: "let".to_string(),
                },
                name: Identifier {
                    token: Token {
                        Type: TokenType::IDENT,
                        Literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                value: Box::new(Identifier {
                    token: Token {
                        Type: TokenType::IDENT,
                        Literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                }),
            })],
        };
        assert_eq!(program.String(), "let myVar = anotherVar;");
    }
}
