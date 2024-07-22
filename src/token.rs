use crate::span::Span;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TokenKind {
    EOF,
    NewLine,
    Identifier,

    Ret,

    FloatLiteral,
    IntLiteral,

    Tilde,
    Bang,
    At,
    Pound,
    Dollar,
    Percent,
    Caret,
    Ampersand,
    Star,
    LParen,
    RParen,
    Minus,
    Underscore,
    Equals,
    Plus,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Pipe,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Slash,
    Question,
    ArrowLeft,
    ArrowRight,
    FatArrowRight,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    MinusMinus,
    NotEquals,
    PlusPlus,
    EqualsEquals,
}

#[derive(Debug)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub span: Span,
    pub text: &'source str,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, {:?}", self.kind, self.span)?;
        if !self.text.is_empty() {
            write!(f, ", {:?}", self.text)?;
        };
        write!(f, ")")
    }
}
