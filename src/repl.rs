use std::io::{self, Write };
use crate::lexer::lexer::{Lexer};
use crate::lexer::token;
use crate::parser::Parser;
use crate::exec;

const PROMPT: &str = ">> ";
const PROMPT_MULTILINE: &str = "> ";

pub fn start_lexing() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).expect("An error occured.");

        let mut lexer: Lexer = Lexer::new(buffer.clone());
        let mut tok: token::Token = lexer.next_token();
        while tok.tok_type != token::TokenType::EOF  {
            if tok.tok_type == token::TokenType::MULTILINE {
                print!("{}", PROMPT_MULTILINE);
                io::stdout().flush().unwrap();

                io::stdin().read_line(&mut buffer).expect("An error occured.");
                lexer.set_input(buffer.clone());
            }
            println!("{:?}", tok);

            tok = lexer.next_token();
        }
        println!("{:?}", tok);
    }
}

pub fn start_parsing() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).expect("An error occured.");

        let mut parser: Parser = Parser::new(Lexer::new(buffer));
        let commands_parsed: Vec<Vec<String>> = parser.parse_command();

        println!("Parsed commands : {commands_parsed:?}");
    }
}

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).expect("An error occured.");

        let mut parser: Parser = Parser::new(Lexer::new(buffer));
        let mut commands_parsed: Vec<Vec<String>> = parser.parse_command();

        for command in commands_parsed.iter_mut() {
            let bin: String = command.remove(1);
            exec::exec_program(bin.as_str(), command );
        }

    }
}