use crate::logger::Log;
use crate::token::{Token, TokenKind};

#[derive(Debug, Default, Clone)]
pub struct Span {
    pub file: &'static str,
    // both line and col are counted from 1
    pub line: usize,
    pub col: usize,
    pub length: Option<usize>,
}

impl Span {
    pub fn new(file: &'static str, line: usize, col: usize) -> Self {
        Self { file, line, col, length: None }
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }

    pub fn line<F: FnOnce(usize) -> usize>(mut self, f: F) -> Self {
        self.line = f(self.line);
        self
    }

    pub fn col<F: FnOnce(usize) -> usize>(mut self, f: F) -> Self {
        self.col = f(self.col);
        self
    }

    pub fn into_log(self) -> Log {
        Log::new().span(self)
    }

    pub fn into_token(self, kind: TokenKind) -> Token {
        Token { kind, span: self }
    }
}
