#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // single character
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Slash,
    BackSlash,
    Star,
    Percent,
    Pipe,
    Ampersand,
    Tilde,
    Greater,
    Lesser,
    Comma,
    Bang,
    Dollar,
    Assign,
    Dot,
    Colon,
    Semicolon,
    QuestionMark,
    At,
    Hash,
    Quote,
    BackTick,
    Caret,
    Underscore,
    NewLine,
    Space,

    // Double character
    Equals,
    BangEquals,
    GreaterEquals,
    LesserEquals,
    And,
    Or,
    Increment,
    Decrement,
    RightShift,
    LeftShift,
    LeftArrow,
    RightArrow,

    // Literals
    Identifier,
    String,
    Char,
    Int, // Todo
    Float, // Todo

    // Registers
    Register,
    RegisterNumber,
    LowByte,
    HighByte,
    Word,
    DoubleWord,
    QuadWord,

    // Keywords/Directive
    Ret,
    Inc,
    Ent,
    Txt,
    Arch,
    Def,
    Con,

    EOF
}
