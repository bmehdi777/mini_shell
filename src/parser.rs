use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
pub struct Parser {
    pub lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
        }
    }

    pub fn parse_command(&mut self) -> Vec<Vec<String>> {
        let mut commands: Vec<Vec<String>> = vec![];
        let mut tok: Token = self.lexer.next_token();
        while tok.tok_type != TokenType::EOF {
            println!("{:?}", tok);
            match tok.tok_type {
                TokenType::COMMAND => {
                    commands.push(vec![]);
                    commands.last_mut()
                                .expect("An error occured while parsing : can't take last Vec<Commands> if it doesn't exist.")
                                .push(tok.value);
                }
                TokenType::OPTIONS => {
                    commands.last_mut()
                                .expect("An error occured while parsing : can't take last Vec<Commands> if it doesn't exist.")
                                .push(tok.value);
                }
                _ => {}
            }

            tok = self.lexer.next_token();
        }
        println!("{:?}", tok);

        return commands;
    }
}
