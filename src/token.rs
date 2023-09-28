use crate::location::Span;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum TokenKind {
    Ampersand,
    At,
    Backslash,
    Backtick,
    Bang,
    Caret,
    CharLiteral,
    Colon,
    Comma,
    Dollar,
    Dot,
    DoubleQuote,
    EOF,
    Equals,
    FatArrow,
    FloatLiteral,
    GreaterThan,
    GreaterThanEquals,
    Identifier,
    IntegerLiteral,
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
    SingleQuote,
    Slash,
    Star,
    StringLiteral,
    Tilde,
    TinyArrowLeft,
    TinyArrowRight,
    Underscore,
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, text: String) -> Token {
        Token {
            kind,
            span,
            text,
        }
    }

    pub fn new_simple(kind: TokenKind, span: Span) -> Token {
        Token::new(kind, span, String::new())
    }

    pub fn new_eof(span: Span) -> Token {
        Token::new_simple(TokenKind::EOF, span)
    }

    pub fn from_string(span: Span, text: String) -> Token {
        Token {
            kind: match text.as_ref() {
                "ret" => TokenKind::Ret,
                "jmp" => TokenKind::Jmp,
                _ => TokenKind::Identifier,
            },
            span,
            text,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.kind)?;
        if !self.text.is_empty() {
            write!(f, "({:?})", self.text)?;
        }
        Ok(())
    }
}
