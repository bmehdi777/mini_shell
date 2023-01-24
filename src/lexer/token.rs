#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    COMMAND,
    OPTIONS,
    
    MULTILINE, // \

    THEN, // && 
    OVERWRITE_RIGHT, // >
    OVERWRITE_LEFT, // <
    APPEND_RIGHT, // >>
    APPEND_LEFT, // <<

    ILLEGAL,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub tok_type: TokenType,
}

impl Token {
    pub fn new(value: String, tok_type: TokenType) -> Token {
        Token { value, tok_type }
    }
    
}
