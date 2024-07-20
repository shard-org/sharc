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
    FatArrow,         // =>
    FatDoubleArrow,   // =>>
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
    Ident(String),
    NL,

    // keywords
    Jmp,
    Ret,
    End,
    Inline,
    Entry,

    // literals
    CharLit(char),
    StrLit(String),

    IntLit(usize),
    FloatLit(f64),
    SIntLit(isize),
}
struct Lexer {

}

impl Lexer {
    pub fn 
}
