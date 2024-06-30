use std::io;
use std::io::{BufRead, Write};

use crate::token::token::{Lexer, TokenType};

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    print!("{}", PROMPT);
    io::stdout().flush().unwrap();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut lexer = Lexer::new(line);
                let mut tok = lexer.next_token();
                while tok.Type != TokenType::EOF {
                    tok = lexer.next_token();
                    println!("{:?}", tok);
                }
                print!("{}", PROMPT);
                io::stdout().flush().unwrap();
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                break;
            }
        }
    }
}
