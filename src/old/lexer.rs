use crate::location::{Location, Span};
use crate::token::{Token, TokenKind};
use crate::logger::{Log, ERR, FATAL};

#[derive(Debug)]
pub struct Lexer {
    filename:  &'static str,
    input:     String,

    column:    usize,
    line:      usize,

    cur_index: usize,
}

impl Lexer {
    pub fn new(input: String, filename: &'static str) -> Lexer {
        Lexer {
            filename,
            location: Location {
                line: 1,
                column: 1,
            },
            input,
            current_index: 0,
        }
    }

    fn cur(&self) -> Option<char> { self.input.chars().nth(self.current_index) }

    fn peek(&self) -> Option<char> { self.input.chars().nth(self.current_index + 1) }

    fn advance(&mut self) {
        match self.cur() {
            Some('\n') => {
                self.location.line += 1;
                self.location.column = 1;
                self.current_index += 1;
            },
            Some(_) => {
                self.location.column += 1;
                self.current_index += 1;
            },
            None => (),
        }
    }

    fn loc(&self) -> Location { self.location }

    fn span(&self, start: Location, end: Location) -> Span { Span::new(self.filename, start, end) }

    fn push(&mut self, tokens: &mut Vec<Token>, token: Token) { tokens.push(token); }

    fn push_simple(&mut self, tokens: &mut Vec<Token>, kind: TokenKind, len: usize) {
        let start = self.loc();
        let text = self.input[self.current_index..self.current_index + len].to_string();
        for _ in 0..len {
            self.advance();
        }
        self.push(tokens, Token::new(kind, self.span(start, self.loc()), text));
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.cur() {
            let start = self.loc();
            match c {
                ' ' => {
                    if let Some(last) = tokens.last_mut() {
                        last.set_flag_bit(6, true);
                    };
                    self.advance();
                }
                '\t' | '\r' => self.advance(),
                '\n' => {
                    if let Some(last_token) = tokens.last() {
                        match last_token.kind {
                            TokenKind::Newline => {
                                self.advance();
                            },
                            _ => {
                                self.push_simple(&mut tokens, TokenKind::Newline, 1);
                            },
                        }
                    } else {
                        self.push_simple(&mut tokens, TokenKind::Newline, 1);
                    }
                },
                '0' if self.peek().map_or(false, |c| "dbox".contains(c)) => {
                    let base = match self.peek() {
                        Some('d') => Base::Decimal,
                        Some('b') => Base::Binary,
                        Some('o') => Base::Octal,
                        Some('x') => Base::Hexadecimal,
                        _ => unreachable!(),
                    };
                    self.advance();
                    self.advance();

                    let mut num = String::new();
                    self.lex_number(&mut num, base);
                    self.push(&mut tokens, Token::new(Base::into_token(base), self.span(start, self.loc()), num))
                }
                '0'..='9' => {
                    let mut num = String::new();
                    self.lex_number(&mut num, Base::Decimal);

                    if let Some('.') = self.cur() {
                        num.push('.');
                        self.advance();
                        self.lex_number(&mut num, Base::Decimal);
                        self.push(&mut tokens, Token::new(TokenKind::FloatLiteral, self.span(start, self.loc()), num));
                    } else {
                        self.push(&mut tokens, Token::new(TokenKind::DecLiteral, self.span(start, self.loc()), num));
                    }
                }
                '"' => {
                    let token = self.lex_string_literal();
                    self.push(&mut tokens, token)
                },
                '`' => {
                    self.advance();
                    let mut text = String::new();
                    match self.cur() {
                        Some('\\') => {
                            self.advance();
                            match self.cur() {
                                Some('n') => text.push('\n'),
                                Some('t') => text.push('\t'),
                                Some('\\') => text.push('\\'),
                                Some('`') => text.push('`'),
                                _ => {}
                            }
                            self.advance();
                        }
                        Some(c) => {
                            text.push(c);
                            self.advance();
                        }
                        None => {
                            Log::new(ERR, self.span(self.loc(), self.loc()), "Unterminated Char Literal", "Expected '`'").push();
                        }
                    };
                    if let Some('`') = self.cur() {
                        self.push(&mut tokens, Token::new(TokenKind::CharLiteral, self.span(start, self.loc()), text));
                        self.advance();
                    } else {
                        Log::new(ERR, self.span(self.loc(), self.loc()), "Unterminated Char Literal", "Expected '`'").push();
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    match c {
                        'r' => {
                            self.advance();
                            let tmp = self.loc();
                            if let Some('0'..='9') = self.cur() {
                                let mut text = String::new();
                                self.lex_number(&mut text, Base::RDecimal);
                                let tmp = self.loc();
                                let size: u8 = match self.cur() {
                                        Some(' ' | '\t' | '\r') | None => 0,
                                        Some('l') => 1,
                                        Some('h') => 2,
                                        Some('w') => 3,
                                        Some('d') => 4,
                                        Some('q') => 5,
                                        _ => {
                                            Log::new(ERR, self.span(tmp, self.loc()), format!("Unexpected character: '{}'", self.cur().unwrap()), "Expected register size").push();
                                            0
                                        }
                                };
                                if size != 0 {
                                    self.advance();
                                }
                                let mut token = Token::new(TokenKind::Register, self.span(start, self.loc()), text);
                                token.set_register_size(size);
                                self.push(&mut tokens, token);
                            } else {
                                Log::new(ERR, self.span(tmp, self.loc()), format!("Unexpected character: '{}'", self.cur().unwrap()), "Expected register number").push();
                            }
                        }
                        _ => {
                            let mut ident = String::new();
                            while let Some(c) = self.cur() {
                                match c {
                                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                                        ident.push(c);
                                        self.advance();
                                    },
                                    _ => break,
                                }
                            }
                            match ident.as_ref() {
                                "_" => self.push(&mut tokens, Token::new(TokenKind::Underscore, self.span(start, self.loc()), ident)),
                                _ => self.push(&mut tokens, Token::from_string(self.span(start, self.loc()), ident)),
                            }
                        }
                    }
                }

                '&' => self.push_simple(&mut tokens, TokenKind::Ampersand, 1),
                '@' => self.push_simple(&mut tokens, TokenKind::At, 1),
                '\\' => self.push_simple(&mut tokens, TokenKind::Backslash, 1),
                '!' => match self.peek() {
                    Some('=') => self.push_simple(&mut tokens, TokenKind::NotEquals, 2),
                    _ => self.push_simple(&mut tokens, TokenKind::Bang, 1),
                },
                '^' => self.push_simple(&mut tokens, TokenKind::Caret, 1),
                ':' => self.push_simple(&mut tokens, TokenKind::Colon, 1),
                ',' => self.push_simple(&mut tokens, TokenKind::Comma, 1),
                '$' => self.push_simple(&mut tokens, TokenKind::Dollar, 1),
                '.' => self.push_simple(&mut tokens, TokenKind::Dot, 1),
                '=' => match self.peek() {
                    Some('>') => self.push_simple(&mut tokens, TokenKind::FatArrow, 2),
                    _ => self.push_simple(&mut tokens, TokenKind::Equals, 1),
                },
                '>' => match self.peek() {
                    Some('=') => self.push_simple(&mut tokens, TokenKind::GreaterThanEquals, 2),
                    _ => self.push_simple(&mut tokens, TokenKind::GreaterThan, 1),
                },
                '{' => self.push_simple(&mut tokens, TokenKind::LeftBrace, 1),
                '[' => self.push_simple(&mut tokens, TokenKind::LeftBracket, 1),
                '(' => self.push_simple(&mut tokens, TokenKind::LeftParen, 1),
                '<' => match self.peek() {
                    Some('-') => self.push_simple(&mut tokens, TokenKind::TinyArrowLeft, 2),
                    Some('=') => self.push_simple(&mut tokens, TokenKind::LessThanEquals, 2),
                    _ => self.push_simple(&mut tokens, TokenKind::LessThan, 1),
                },
                '-' => match self.peek() {
                    Some('>') => self.push_simple(&mut tokens, TokenKind::TinyArrowRight, 2),
                    Some('-') => self.push_simple(&mut tokens, TokenKind::MinusMinus, 2),
                    _ => self.push_simple(&mut tokens, TokenKind::Minus, 1),
                },
                '%' => self.push_simple(&mut tokens, TokenKind::Percent, 1),
                '|' => self.push_simple(&mut tokens, TokenKind::Pipe, 1),
                '+' => match self.peek() {
                    Some('+') => self.push_simple(&mut tokens, TokenKind::PlusPlus, 2),
                    _ => self.push_simple(&mut tokens, TokenKind::Plus, 1),
                },
                '#' => self.push_simple(&mut tokens, TokenKind::Pound, 1),
                '?' => self.push_simple(&mut tokens, TokenKind::Question, 1),
                '}' => self.push_simple(&mut tokens, TokenKind::RightBrace, 1),
                ']' => self.push_simple(&mut tokens, TokenKind::RightBracket, 1),
                ')' => self.push_simple(&mut tokens, TokenKind::RightParen, 1),
                ';' => self.push_simple(&mut tokens, TokenKind::Semicolon, 1),
                '/' => match self.peek() {
                    Some('/') => {
                        while let Some(c) = self.cur() {
                            match c {
                                '\n' => break,
                                _ => self.advance(),
                            }
                        }
                    },
                    Some('*') => {
                        self.advance();
                        while let Some(c) = self.cur() {
                            match c {
                                '*' => {
                                    self.advance();
                                    if let Some('/') = self.cur() {
                                        self.advance();
                                        break;
                                    }
                                },
                                _ => self.advance(),
                            }
                        }
                    },
                    _ => self.push_simple(&mut tokens, TokenKind::Slash, 1),
                },
                '*' => self.push_simple(&mut tokens, TokenKind::Star, 1),
                '~' => self.push_simple(&mut tokens, TokenKind::Tilde, 1),

                _ => {
                    let level = match c.to_string().into_bytes().len() {
                        1 => ERR,
                        _ => FATAL,
                    };
                    Log::new(level, self.span(start, self.loc()), format!("Unexpected character: '{}'", c), "").push();
                    self.advance();
                },
            }
        }
        self.push_simple(&mut tokens, TokenKind::EOF, 0);
        tokens
    }

    fn lex_number(&mut self, num: &mut String, base: Base) {
        while let Some(c) = self.cur() {
            match (base, c) {
                (Base::Decimal | Base::RDecimal, '0'..='9')
                | (Base::Binary, '0' | '1')
                | (Base::Octal, '0'..='7')
                | (Base::Hexadecimal, '0'..='9' | 'a'..='f') => {
                    num.push(c);
                    self.advance();
                },
                (_, '_') => self.advance(),
                (Base::RDecimal, _) => break,
                (_, '0'..='9' | 'a'..='f') => {
                    Log::new(ERR, self.span(self.loc(), self.loc()), format!("Unexpected character for base {}: '{}'", base, c), "").push();
                    break;
                },
                _ => break,
            }
        }
    }

    fn lex_string_literal(&mut self) -> Token {
        let start = self.loc();
        let mut text = String::new();
        let quote = self.cur().unwrap();
        self.advance();
        while let Some(c) = self.cur() {
            match c {
                '\n' => {
                    Log::new(ERR, self.span(start, self.loc()), "Unterminated String Literal", "Expected '\"'").push();
                    break;
                }
                '\\' => {
                    self.advance();
                    match self.cur() {
                        Some('n') => text.push('\n'),
                        Some('t') => text.push('\t'),
                        Some('\\') => text.push('\\'),
                        Some('\'') => text.push('\''),
                        Some(quote) => text.push(quote),
                        _ => {
                            Log::new(ERR, self.span(start, self.loc()), format!("Unexpected character: '{}'", self.cur().unwrap()), "Expected '\"'").push();
                        }
                    }
                    self.advance();
                }
                c if c == quote => {
                    self.advance();
                    break;
                }
                _ => {
                    text.push(c);
                    self.advance();
                }
            }
        }
        Token::new(TokenKind::StringLiteral, self.span(start, self.loc()), text)
    }
}


#[derive(Clone, Copy)]
pub enum Base {
    Binary,      // 0b
    Octal,       // 0o
    Decimal,     // No prefix | 0d
    RDecimal,    // Register Decimal  (ends on first non-decimal digit)
    Hexadecimal, // 0x
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Base::Binary => write!(f, "2"),
            Base::Octal => write!(f, "8"),
            Base::Decimal => write!(f, "10"),
            Base::RDecimal => write!(f, "10"),
            Base::Hexadecimal => write!(f, "16"),
        }
    }
}

impl Base {
    pub fn into_token(self) -> TokenKind {
        match self {
            Base::Binary => TokenKind::BinLiteral,
            Base::Octal => TokenKind::OctLiteral,
            Base::Decimal => TokenKind::DecLiteral,
            Base::RDecimal => TokenKind::DecLiteral,
            Base::Hexadecimal => TokenKind::HexLiteral,
        }
    }
}
