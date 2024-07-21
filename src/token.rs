use crate::span::Span;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TokenKind {
    EOF,
    NewLine,
    Identifier,

    FloatLiteral,
    BinaryIntLiteral,
    OctalIntLiteral,
    DecimalIntLiteral,
    HexadecimalIntLiteral,

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
pub struct Token<'contents> {
    pub kind: TokenKind,
    pub span: Span,
    pub text: &'contents str,
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
