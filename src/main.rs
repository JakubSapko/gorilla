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
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

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
    ch: char,             // current char unde examination
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
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn determine_token_type(&mut self, lit: &String) -> TokenType {
        let tok_type = match lit.as_str() {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        };
        return tok_type;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '-' => new_token(TokenType::MINUS, char::from(self.ch).to_string()),
            '!' => new_token(TokenType::BANG, char::from(self.ch).to_string()),
            '/' => new_token(TokenType::SLASH, char::from(self.ch).to_string()),
            '*' => new_token(TokenType::ASTERISK, char::from(self.ch).to_string()),
            '<' => new_token(TokenType::LT, char::from(self.ch).to_string()),
            '>' => new_token(TokenType::GT, char::from(self.ch).to_string()),
            '=' => new_token(TokenType::ASSIGN, char::from(self.ch).to_string()),
            ';' => new_token(TokenType::SEMICOLON, char::from(self.ch).to_string()),
            '(' => new_token(TokenType::LPAREN, char::from(self.ch).to_string()),
            ')' => new_token(TokenType::RPAREN, char::from(self.ch).to_string()),
            ',' => new_token(TokenType::COMMA, char::from(self.ch).to_string()),
            '+' => new_token(TokenType::PLUS, char::from(self.ch).to_string()),
            '{' => new_token(TokenType::LBRACE, char::from(self.ch).to_string()),
            '}' => new_token(TokenType::RBRACE, char::from(self.ch).to_string()),
            '\0' => new_token(TokenType::EOF, char::from(self.ch).to_string()),
            _ => {
                if is_letter(self.ch) {
                    let lit = self.read_identifier();
                    let tok_type = self.determine_token_type(&lit);
                    return new_token(tok_type, lit);
                } else if is_digit(self.ch) {
                    let tok_type = TokenType::INT;
                    let lit = self.read_number();
                    return new_token(tok_type, lit);
                } else {
                    return new_token(TokenType::ILLEGAL, self.ch.to_string());
                }
            }
        };
        self.read_char();
        return tok;
    }
}

fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}

fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

fn new_token(token_type: TokenType, lit: String) -> Token {
    println!("{:?}, {:?}", token_type, lit);
    return Token {
        Type: token_type,
        Literal: lit,
    };
}

fn main() {
    println!("Hello, world!");
}
