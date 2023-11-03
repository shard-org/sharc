use crate::location::Span;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum TokenKind {
    Ampersand,
    At,
    Backslash,
    Bang,
    BinLiteral,
    Caret,
    CharLiteral,
    Colon,
    Comma,
    DecLiteral,
    Dollar,
    Dot,
    EOF,
    Equals,
    FatArrow,
    FloatLiteral,
    GreaterThan,
    GreaterThanEquals,
    HexLiteral,
    Identifier,
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
    OctLiteral,
    Percent,
    Pipe,
    Plus,
    PlusPlus,
    Pound,
    Question,
    Register,
    Ret,
    RightBrace,
    RightBracket,
    RightParen,
    Semicolon,
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
    pub flag: u8,
    /*
       1-3: Register size
       7: whitespace after
       8: newline before
    */
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, text: String) -> Token {
        Token { kind, span, text, flag: 0 }
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
            flag: 0,
        }
    }

    pub fn register_size(&self) -> u8 {
        self.flag & 0b0000_0111
    }

    pub fn whitespace_after(&self) -> bool {
        self.flag & 0b0100_0000 != 0
    }

    pub fn set_register_size(&mut self, size: u8) {
        self.flag &= 0b1111_1000;
        self.flag |= size & 0b0000_0111;
    }

    pub fn set_flag_bit(&mut self, bit: u8, val: bool) {
        if val {
            self.flag |= 1 << bit;
        } else {
            self.flag &= !(1 << bit);
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.kind)?;
        if !self.text.is_empty() {
            let register_size = self.register_size();
            let whitespace_after = self.whitespace_after();
            write!(f, "({:?}", self.text)?;
            if register_size != 0 {
                write!(f, ", size={}", register_size)?;
            }
            write!(f, ", ws={}", whitespace_after)?;
            write!(f, ")")?;
        }
        Ok(())
    }
}
