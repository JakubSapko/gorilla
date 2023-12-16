mod tests;
enum TokenType {
    ILLEGAL,
    EOF,
    //identifiers + literals
    IDENT, //add, foobar, x, y, ...
    INT,

    //operators
    ASSIGN,
    PLUS,

    //delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //keywords
    FUNCTION,
    LET,
}

struct Token {
    Literal: String,
    Type: TokenType,
}

#[derive(Default)]
struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in in input (after current char)
    ch: u8,               // current char unde examination
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            ..Default::default()
        };
        l.read_char();
        return l;
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap() as u8;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            b'=' => Token {
                Type: TokenType::ASSIGN,
                Literal: self.ch.to_string(),
            },
            b';' => Token {
                Type: TokenType::SEMICOLON,
                Literal: self.ch.to_string(),
            },
            b'(' => Token {
                Type: TokenType::LPAREN,
                Literal: self.ch.to_string(),
            },
            b')' => Token {
                Type: TokenType::RPAREN,
                Literal: self.ch.to_string(),
            },
            b',' => Token {
                Type: TokenType::COMMA,
                Literal: self.ch.to_string(),
            },
            b'+' => Token {
                Type: TokenType::PLUS,
                Literal: self.ch.to_string(),
            },
            b'{' => Token {
                Type: TokenType::LBRACE,
                Literal: self.ch.to_string(),
            },
            b'}' => Token {
                Type: TokenType::RBRACE,
                Literal: self.ch.to_string(),
            },
            0 => Token {
                Type: TokenType::EOF,
                Literal: "".to_string(),
            },
            _ => panic!("Token not recognized"),
        };
        self.read_char();
        return tok;
    }
}

fn main() {
    println!("Hello, world!");
}
