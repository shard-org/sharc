use std::fmt::Display;

use crate::position::Span;

#[derive(Debug)]
pub enum RegSize {
    Arch,  // architecure dependent
    ByteLow,
    ByteHigh,
    Word,
    DWord,
    QWord,
}

impl From<Option<char>> for RegSize {
    fn from(value: Option<char>) -> Self {
        match value {
            Some('l') => Self::ByteLow,
            Some('h') => Self::ByteHigh,
            Some('w') => Self::Word,
            Some('d') => Self::DWord,
            Some('q') => Self::QWord,
            None => Self::Arch
        }
    }
}

impl Display for RegSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ByteLow  => "l",
            Self::ByteHigh => "h",
            Self::Word     => "w",
            Self::DWord    => "d",
            Self::QWord    => "q",
            Self::Arch     => ""
        })
    }
}

#[derive(Debug)]
pub enum IntLiteralRadix {
    Bin,
    Dec,
    Hex
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum TokenKind<'a> {
    Register(u8, RegSize),
    Intager(&'a str, IntLiteralRadix),
    StringLiteral(&'a str),
    Identifier(&'a str),
    CharLiteral(&'a str),

    Ampersand,
    At,
    Backslash,
    Bang,
    Caret,
    Colon,
    Comma,
    Dollar,
    Dot,
    EOF,
    Equals,
    FatArrow,
    FloatLiteral,
    GreaterThan,
    GreaterThanEquals,
    Jmp,
    LeftBrace,
    LeftBracket,
    LeftParen,
    LessThan,
    LessThanEquals,
    Minus,
    MinusMinus,
    Newline,
    NotEquals,
    Percent,
    Pipe,
    Plus,
    PlusPlus,
    Pound,
    Question,
    Ret,
    RightBrace,
    RightBracket,
    RightParen,
    Semicolon,
    Slash,
    Star,
    Tilde,
    TinyArrowLeft,
    TinyArrowRight,
    Underscore,
}

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.kind)?;
        match self.kind {
            TokenKind::Register(n, size) => write!(f, "Register r{}{}", n, size)?,
            TokenKind::Intager(n, rdx) => write!(f, "Intager {} (base {:?})", n, rdx)?,
            TokenKind::StringLiteral(s) => write!(f, "String \"{s}\"")?,
            TokenKind::Identifier(s) => write!(f, "Identifier {s}")?,
            TokenKind::CharLiteral(c) => write!(f, "Char \'{c}\'")?,
            _ => ()
        }
        Ok(())
    }
}
