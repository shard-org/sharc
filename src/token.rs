use crate::span::Span;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    Identifier,
}

pub struct Token<'source> {
    kind: TokenKind,
    span: Span,
    text: &'source str,
}

impl<'source> Token<'source> {
    pub fn new(kind: TokenKind, span: Span, text: &'source str) -> Self {
        Self { kind, span, text }
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token{{{:?}, {}}}", self.kind, self.span)
    }
}
