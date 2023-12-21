#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TokenType {
    #[default]
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
    EQ,
    NOT_EQ,

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
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub Literal: String,
    pub Type: TokenType,
}

#[derive(Default)]
pub struct Lexer {
    pub input: String,
    pub position: usize, // current position in input (points to current char)
    pub read_position: usize, // current reading position in in input (after current char)
    pub ch: char,        // current char unde examination
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
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
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        };
        return tok_type;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '-' => new_token(TokenType::MINUS, char::from(self.ch).to_string()),
            '!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let lit = format!("{}{}", ch, self.ch);
                    new_token(TokenType::NOT_EQ, lit)
                } else {
                    new_token(TokenType::BANG, char::from(self.ch).to_string())
                }
            }
            '/' => new_token(TokenType::SLASH, char::from(self.ch).to_string()),
            '*' => new_token(TokenType::ASTERISK, char::from(self.ch).to_string()),
            '<' => new_token(TokenType::LT, char::from(self.ch).to_string()),
            '>' => new_token(TokenType::GT, char::from(self.ch).to_string()),
            '=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let lit = format!("{}{}", ch, self.ch);
                    new_token(TokenType::EQ, lit)
                } else {
                    new_token(TokenType::ASSIGN, char::from(self.ch).to_string())
                }
            }
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
    return Token {
        Type: token_type,
        Literal: lit,
    };
}
