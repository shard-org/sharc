use crate::span::Span;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TokenKind {
    // Symbols
    Ampersand,
    At,
    // Backtick,   // error or Chalit
    Backslash,
    Bang,
    Caret,
    Colon,
    Comma,
    Dollar,
    Dot,
    // DoubleQuote,   // error or StrLit
    Equals,

    FatArrow,       // =>
    FatDoubleArrow, // =>>

    GreaterThan,
    GreaterThanEquals,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LessThan,
    LessThanEquals,
    Minus,
    MinusMinus,
    NotEquals,
    Percent,
    Pipe,
    Plus,
    PlusPlus,
    Pound,
    Question,
    RightBrace,
    RightBracket,
    RightParen,
    Semicolon,
    SingleQuote,
    Slash,
    Star,
    Tilde,
    SmallArrowLeft,
    SmallArrowRight,
    Underscore,

    // Other
    Ident,
    NL,

    // keywords
    Jmp,
    Ret,

    // literals
    CharLit,
    StrLit,

    IntLit,
    FloatLit,
    SIntLit,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }

    pub fn some(self) -> Option<Self> {
        Some(self)
    }
}
