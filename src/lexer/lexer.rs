use crate::lexer::token::{Token, TokenType};
use crate::lexer::utils;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    token_count: usize,
    character: u8,
}

impl Lexer {
    pub fn new(input: String, ) -> Lexer {
        let mut lex: Lexer = Lexer { input, position: 0, read_position: 0, character: 0, token_count: 0 };
        lex.read_character();
        lex
    }
    pub fn set_input(&mut self, input: String) -> () {
        self.input = input;
    }

    pub fn read_character(&mut self) -> () {
        self.character = self.peek_char();
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
    
        self.skip_whitespace();

        match self.character {
            b'<' => {
                if self.peek_char() == b'<' {
                    let ch1: String = (self.character as char).to_string();
                    self.read_character();
                    let ch2: String = (self.character as char).to_string();
                    tok = Token::new(ch1 + &ch2, TokenType::APPEND_LEFT);
                } else {
                    tok = Token::new(String::from("<"), TokenType::OVERWRITE_LEFT);
                }
                self.token_count = 0;
            },
            b'>' => {
                if self.peek_char() == b'>' {
                    let ch1: String = (self.character as char).to_string();
                    self.read_character();
                    let ch2: String = (self.character as char).to_string();
                    tok = Token::new(ch1 + &ch2, TokenType::APPEND_LEFT);
                } else {
                    tok = Token::new(String::from(">"), TokenType::OVERWRITE_LEFT);
                }
                self.token_count = 0;
            },
            b'\\' => { tok = Token::new(String::from("\\"), TokenType::MULTILINE); },
            b'&' => {
                if self.peek_char() == b'&' {
                    let ch1: String = (self.character as char).to_string();
                    self.read_character();
                    let ch2: String = (self.character as char).to_string();
                    tok = Token::new(ch1 + &ch2, TokenType::THEN);
                } else {
                    tok = Token::new(String::from("&"), TokenType::OVERWRITE_LEFT);
                }
                self.token_count = 0;
            },
            0 => { tok = Token::new(String::from(""), TokenType::EOF); }
            _ => {
                if utils::is_authorize(self.character) {
                    let value: String = self.read_literals();
                    tok = if self.token_count == 0 {
                        Token::new(value, TokenType::COMMAND)
                    } else {
                        Token::new(value, TokenType::OPTIONS)
                    };
                    self.token_count += 1;
                    return tok;
                }
                return Token::new(String::from(""), TokenType::EOF);
            }
        }
        self.read_character();
        tok
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.bytes().nth(self.read_position).unwrap()
        }
    }
    fn read_literals(&mut self) -> String {
        let position: usize = self.position;
        while utils::is_authorize(self.character) {
            self.read_character();
        }
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) -> () {
        while self.character == b' ' || self.character == b'\t' || self.character == b'\n' || self.character == b'\r' {
            self.read_character();
        }
    }


}
