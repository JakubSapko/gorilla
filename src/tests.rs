#[cfg(test)]
mod tests {
    use crate::token::token::{Lexer, Token, TokenType};
    #[test]
    fn test_next_token() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ";
        let tests: Vec<Token> = vec![
            Token {
                Type: TokenType::LET,
                Literal: "let".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "five".to_string(),
            },
            Token {
                Type: TokenType::ASSIGN,
                Literal: "=".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "5".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::LET,
                Literal: "let".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "ten".to_string(),
            },
            Token {
                Type: TokenType::ASSIGN,
                Literal: "=".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "10".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::LET,
                Literal: "let".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "add".to_string(),
            },
            Token {
                Type: TokenType::ASSIGN,
                Literal: "=".to_string(),
            },
            Token {
                Type: TokenType::FUNCTION,
                Literal: "fn".to_string(),
            },
            Token {
                Type: TokenType::LPAREN,
                Literal: "(".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "x".to_string(),
            },
            Token {
                Type: TokenType::COMMA,
                Literal: ",".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "y".to_string(),
            },
            Token {
                Type: TokenType::RPAREN,
                Literal: ")".to_string(),
            },
            Token {
                Type: TokenType::LBRACE,
                Literal: "{".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "x".to_string(),
            },
            Token {
                Type: TokenType::PLUS,
                Literal: "+".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "y".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::RBRACE,
                Literal: "}".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::LET,
                Literal: "let".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "result".to_string(),
            },
            Token {
                Type: TokenType::ASSIGN,
                Literal: "=".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "add".to_string(),
            },
            Token {
                Type: TokenType::LPAREN,
                Literal: "(".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "five".to_string(),
            },
            Token {
                Type: TokenType::COMMA,
                Literal: ",".to_string(),
            },
            Token {
                Type: TokenType::IDENT,
                Literal: "ten".to_string(),
            },
            Token {
                Type: TokenType::RPAREN,
                Literal: ")".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::BANG,
                Literal: "!".to_string(),
            },
            Token {
                Type: TokenType::MINUS,
                Literal: "-".to_string(),
            },
            Token {
                Type: TokenType::SLASH,
                Literal: "/".to_string(),
            },
            Token {
                Type: TokenType::ASTERISK,
                Literal: "*".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "5".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "5".to_string(),
            },
            Token {
                Type: TokenType::LT,
                Literal: "<".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "10".to_string(),
            },
            Token {
                Type: TokenType::GT,
                Literal: ">".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "5".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::IF,
                Literal: "if".to_string(),
            },
            Token {
                Type: TokenType::LPAREN,
                Literal: "(".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "5".to_string(),
            },
            Token {
                Type: TokenType::LT,
                Literal: "<".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "10".to_string(),
            },
            Token {
                Type: TokenType::RPAREN,
                Literal: ")".to_string(),
            },
            Token {
                Type: TokenType::LBRACE,
                Literal: "{".to_string(),
            },
            Token {
                Type: TokenType::RETURN,
                Literal: "return".to_string(),
            },
            Token {
                Type: TokenType::TRUE,
                Literal: "true".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::RBRACE,
                Literal: "}".to_string(),
            },
            Token {
                Type: TokenType::ELSE,
                Literal: "else".to_string(),
            },
            Token {
                Type: TokenType::LBRACE,
                Literal: "{".to_string(),
            },
            Token {
                Type: TokenType::RETURN,
                Literal: "return".to_string(),
            },
            Token {
                Type: TokenType::FALSE,
                Literal: "false".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::RBRACE,
                Literal: "}".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "10".to_string(),
            },
            Token {
                Type: TokenType::EQ,
                Literal: "==".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "10".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "10".to_string(),
            },
            Token {
                Type: TokenType::NOT_EQ,
                Literal: "!=".to_string(),
            },
            Token {
                Type: TokenType::INT,
                Literal: "9".to_string(),
            },
            Token {
                Type: TokenType::SEMICOLON,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::EOF,
                Literal: "\0".to_string(),
            },
        ];
        let mut lexer = Lexer::new(input.to_string());
        for (index, tt) in tests.clone().into_iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.Type, tests[index].Type);
            assert_eq!(tok.Literal, r#tests[index].Literal);
        }
    }
}
