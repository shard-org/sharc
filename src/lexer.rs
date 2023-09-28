use crate::location::{Location, Span};
use crate::token::{Token, TokenKind};


#[derive(Debug)]
pub struct Lexer {
    location: Location,
    input: String,
    current_index: usize,
}

impl Lexer {
    pub fn new(input: String, filename: &'static str) -> Lexer {
        Lexer {
            location: Location {
                line: 1,
                column: 1,
                filename,
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

    fn push(&mut self, tokens: &mut Vec<Token>, token: Token) { tokens.push(token); }

    fn push_simple(&mut self, toknes: &mut Vec<Token>, kind: TokenKind, len: usize) {
        let start = self.loc();
        let text = self.input[self.current_index..self.current_index + len].to_string();
        for _ in 0..len {
            self.advance();
        }
        self.push(toknes, Token::new(kind, Span(start, self.loc()), text));
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.cur() {
            let start = self.loc();
            match c {
                ' ' | '\t' | '\r' => self.advance(),
                '\n' => {
                    if let Some(last_token) = tokens.last() {
                        match last_token.kind {
                            TokenKind::Newline => self.advance(),
                            _ => self.push_simple(&mut tokens, TokenKind::Newline, 1),
                        }
                    }
                },
                '0'..='9' => {
                    let mut num = String::new();
                    self.lex_number(&mut num);

                    if let Some('.') = self.cur() {
                        num.push('.');
                        self.advance();
                        self.lex_number(&mut num);
                        self.push(&mut tokens, Token::new(TokenKind::FloatLiteral, Span(start, self.loc()), num));
                    } else {
                        self.push(&mut tokens, Token::new(TokenKind::IntegerLiteral, Span(start, self.loc()), num));
                    }
                }
                '\'' | '"' => {
                    let token = self.lex_quoted_literal();
                    self.push(&mut tokens, token)
                },
                'a'..='z' | 'A'..='Z' | '_' => {
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
                        "_" => self.push(&mut tokens, Token::new(TokenKind::Underscore, Span(start, self.loc()), ident)),
                        _ => self.push(&mut tokens, Token::from_string(Span(start, self.loc()), ident)),
                    }
                }

                '&' => self.push_simple(&mut tokens, TokenKind::Ampersand, 1),
                '@' => self.push_simple(&mut tokens, TokenKind::At, 1),
                '\\' => self.push_simple(&mut tokens, TokenKind::Backslash, 1),
                '`' => self.push_simple(&mut tokens, TokenKind::Backtick, 1),
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
                    _ => self.push_simple(&mut tokens, TokenKind::Slash, 1),
                },
                '*' => self.push_simple(&mut tokens, TokenKind::Star, 1),
                '~' => self.push_simple(&mut tokens, TokenKind::Tilde, 1),

                _ => {
                    // fixme: Error handling
                    panic!("Unexpected character: {}", c)
                },
            }
        }
        self.push_simple(&mut tokens, TokenKind::EOF, 0);
        tokens
    }

    fn lex_number(&mut self, num: &mut String) {
        while let Some(c) = self.cur() {
            match c {
                '0'..='9' => {
                    num.push(c);
                    self.advance();
                },
                _ => break,
            }
        }
    }

    fn lex_quoted_literal(&mut self) -> Token {
        // todo: String literals
        let start = self.loc();
        let mut text = String::new();
        let quote = self.cur().unwrap();
        self.advance();
        while let Some(c) = self.cur() {
            match c {
                '\n' => {
                    // fixme: Error handling
                    panic!("Unterminated quoted literal")
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
                            // fixme: Error handling
                            panic!("Unexpected character: {}", c)
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
        match quote {
            '\'' => {
                if text.len() != 1 {
                    // fixme: Error handling
                    panic!("Char literal must be one character long")
                }
                Token::new(TokenKind::CharLiteral, Span(start, self.loc()), text)
            },
            '"' => Token::new(TokenKind::StringLiteral, Span(start, self.loc()), text),
            _ => unreachable!(),
        }
    }
}
