mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone)]
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
            b'=' => new_token(TokenType::ASSIGN, self.ch),
            b';' => new_token(TokenType::SEMICOLON, self.ch),
            b'(' => new_token(TokenType::LPAREN, self.ch),
            b')' => new_token(TokenType::RPAREN, self.ch),
            b',' => new_token(TokenType::COMMA, self.ch),
            b'+' => new_token(TokenType::PLUS, self.ch),
            b'{' => new_token(TokenType::LBRACE, self.ch),
            b'}' => new_token(TokenType::RBRACE, self.ch),
            b'\0' => new_token(TokenType::EOF, self.ch),
            _ => panic!("Token not recognized"),
        };
        self.read_char();
        return tok;
    }
}

fn new_token(token_type: TokenType, ch: u8) -> Token {
    return Token {
        Type: token_type,
        Literal: char::from(ch).to_string(),
    };
}

fn main() {
    println!("Hello, world!");
}
