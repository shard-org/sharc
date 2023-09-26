use crate::literals::literals::Literal;

use super::tokentype::TokenType;
use std::{path::Path, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    token: TokenType,
    lexeme: String,
    line: i32,
    literal: Option<Literal>, // Todo
    file: Rc<Path>,
    at: usize,
}

impl Token {
    pub fn new(
        token: TokenType,
        lexeme: String,
        line: i32,
        file: Rc<Path>,
        at: usize,
        literal: Option<Literal>,
    ) -> Self {
        Self {
            token,
            lexeme,
            literal,
            line,
            at,
            file,
        }
    }
}
