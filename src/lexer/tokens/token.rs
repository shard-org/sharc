use crate::parser::Arg;

use super::tokentype::TokenType;

#[derive(Debug)]
pub struct Token {
    token: TokenType,
    lexeme: String,
    line: usize,
    column: usize,
    literal: Option<Arg>,
}

impl Token {
    pub fn new(
        token: TokenType,
        lexeme: String,
        line: usize,
        column: usize,
        literal: Option<Arg>,
    ) -> Self {
        Self {
            token,
            lexeme,
            literal,
            line,
            column,
        }
    }

    pub fn eof(line: usize) -> Self {
        Self {
            token: TokenType::EOF,
            lexeme: "".to_string(),
            line,
            literal: None,
            column: 0,
        }
    }
}
