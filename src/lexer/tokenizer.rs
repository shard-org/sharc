use std::{path::Path, rc::Rc};

use super::tokens::token::Token;

pub struct Tokenizer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: i32,
    file: Rc<Path>,
}

impl Tokenizer {
    pub fn new(source: &str, file: Rc<Path>) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1,
            file,
        }
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let character = self.source.get(self.curr).unwrap();
        self.curr += 1;
        *character
    }
}
