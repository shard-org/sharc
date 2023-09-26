use crate::literals::literals::Literal;

use super::tokentype::TokenType;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    token: TokenType,
    lexeme: String,
    line: u32,
    literal: Option<Literal>, // Todo
    at: usize,
}

impl Token {
    pub fn new(
        token: TokenType,
        lexeme: String,
        line: u32,
        at: usize,
        literal: Option<Literal>,
    ) -> Self {
        Self {
            token,
            lexeme,
            literal,
            line,
            at,
        }
    }

    pub fn eof(line: u32) -> Self {
        Self {
            token: TokenType::EOF,
            lexeme: "".to_string(),
            line,
            literal: None,
            at: line as usize,
        }
    }
}
