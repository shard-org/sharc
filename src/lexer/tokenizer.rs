use super::tokens::{token::*, tokentype::*};
use crate::{
    args_parser::ARGS,
    log_at,
    logger::ERR,
    logger::{at, logger, At},
    parser::{Arg, RegSize},
};
use std::{path::Path, process};

pub struct Tokenizer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    column: usize,
    at: Box<At>,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            at: Box::new(at(1, Path::new(unsafe { ARGS.infile }))),
            column: 0,
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.at.line));
        &self.tokens
    }

    pub fn print(&self) {
        for token in self.tokens.iter() {
            println!("{:?}", token);
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '+' => {
                let token = if self.is_match('+') {
                    TokenType::Increment
                } else {
                    TokenType::Plus
                };
                self.add_token(token)
            }
            '-' => {
                let token = if self.is_match('-') {
                    TokenType::Decrement
                } else if self.is_match('>') {
                    TokenType::RightArrow
                } else {
                    TokenType::Minus
                };
                self.add_token(token)
            }
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.is_match('*') {
                    self.block_comment();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '\\' => self.add_token(TokenType::BackSlash),
            '%' => self.add_token(TokenType::Percent),
            '|' => {
                let token = if self.is_match('|') {
                    TokenType::Or
                } else {
                    TokenType::Pipe
                };
                self.add_token(token)
            }
            '&' => {
                let token = if self.is_match('&') {
                    TokenType::And
                } else {
                    TokenType::Ampersand
                };
                self.add_token(token)
            }
            '~' => self.add_token(TokenType::Tilde),
            '>' => {
                let token = if self.is_match('=') {
                    TokenType::GreaterEquals
                } else if self.is_match('>') {
                    TokenType::RightShift
                } else {
                    TokenType::Greater
                };

                self.add_token(token)
            }
            '<' => {
                let token = if self.is_match('=') {
                    TokenType::LesserEquals
                } else if self.is_match('<') {
                    TokenType::LeftShift
                } else if self.is_match('-') {
                    TokenType::LeftArrow
                } else {
                    TokenType::Lesser
                };

                self.add_token(token)
            }
            ',' => self.add_token(TokenType::Comma),
            '!' => {
                let token = if self.is_match('=') {
                    TokenType::BangEquals
                } else {
                    TokenType::Bang
                };
                self.add_token(token)
            }
            '$' => self.add_token(TokenType::Dollar),
            '=' => {
                let token = if self.is_match('=') {
                    TokenType::Equals
                } else {
                    TokenType::Assign
                };
                self.add_token(token)
            }
            '.' => self.add_token(TokenType::Dot),
            ':' => self.add_token(TokenType::Colon),
            ';' => self.add_token(TokenType::Semicolon),
            '?' => self.add_token(TokenType::QuestionMark),
            '@' => self.add_token(TokenType::At),
            '#' => self.add_token(TokenType::Hash),
            '\'' => self.mutate(),
            '"' => self.string(),
            '`' => self.add_token(TokenType::BackTick),
            'r' => {
                if !Tokenizer::is_digit(self.peek()) {
                    self.identifier();
                } else {
                    self.register();
                }
            }
            '^' => self.add_token(TokenType::Caret),
            _ if c.is_ascii_alphabetic() => self.identifier(),
            '_' => self.add_token(TokenType::Underscore),
            '\n' => {
                self.add_token(TokenType::NewLine);
                self.at.line += 1;
                self.column = 0;
            }
            '\r' => (),
            '\t' => (),
            ' ' => self.add_token(TokenType::Space),
            _ => {
                log_at!(ERR, *self.at.clone(), "Unexpected character '{}'", c);
            }
        }
    }

    fn register(&mut self) {
        self.add_token(TokenType::Register);
        while Tokenizer::is_digit(self.peek()) {
            self.advance();
        }

        self.start += 1;
        let num: String = self.source[self.start..self.curr].iter().collect();
        let Ok(reg_num) = num.parse::<u8>() else {
            log_at!(ERR, *self.at.clone(), "Could not parse register number keep range within 1..=255");
            return;
        };
        self.add_literal(
            TokenType::RegisterNumber,
            Some(Arg::Reg(reg_num, RegSize::Unknown)),
        );
        self.start = self.curr;
        match self.peek() {
            'l' => {
                self.advance();
                self.add_token(TokenType::LowByte)
            }
            'h' => {
                self.advance();
                self.add_token(TokenType::HighByte)
            }
            'w' => {
                self.advance();
                self.add_token(TokenType::Word)
            }
            'd' => {
                self.advance();
                self.add_token(TokenType::DoubleWord)
            },
            'q' => {
                self.advance();
                self.add_token(TokenType::QuadWord)
            },
            ' ' => (),
            _ => {
                let c = self.advance();
                log_at!(ERR, *self.at.clone(), "Unexpected register width specifier. {}", c);
            },
        };
    }

    fn mutate(&mut self) {
        if self.peek_next() == '\'' {
            let c = self.advance();
            self.advance();
            self.add_literal(TokenType::Char, Some(Arg::Char(c)));
            return;
        }
        self.add_token(TokenType::Quote);
        self.start += 1;
        self.identifier();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.at.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            log_at!(ERR, *self.at.clone(), "String was not terminated");
            process::exit(1);
        }

        self.advance();

        // TODO: Handle escapes sequences
        let value: String = self.source[(self.start + 1)..(self.curr - 1)]
            .iter()
            .collect();
        self.add_literal(TokenType::String, Some(Arg::Str(value)));
    }

    // fn number(&mut self) {
    //     while Tokenizer::is_digit(self.peek()) {
    //         self.advance();
    //     }
    //
    //     if self.peek() == '.' && Tokenizer::is_digit(self.peek_next()) {
    //         self.advance();
    //
    //         while self.peek().is_ascii_digit() {
    //             self.advance();
    //         }
    //     }
    //
    //     let value: String = self.source[self.start..self.curr].iter().collect();
    //     self.add_literal(
    //         TokenType::Number,
    //     );
    // }

    fn identifier(&mut self) {
        while Tokenizer::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.curr].iter().collect();
        if let Some(token) = self.keywords(&text) {
            self.add_token(token);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn keywords(&self, text: &str) -> Option<TokenType> {
        match text {
            "inc" => Some(TokenType::Inc),
            "con" => Some(TokenType::Con),
            "ret" => Some(TokenType::Ret),
            "txt" => Some(TokenType::Txt),
            "def" => Some(TokenType::Def),
            "arch" => Some(TokenType::Arch),
            "ent" => Some(TokenType::Ent),
            _ => None,
        }
    }

    fn block_comment(&mut self) {
        loop {
            match self.peek() {
                '/' => {
                    self.advance();
                    if self.is_match('*') {
                        self.block_comment();
                    }
                }
                '*' => {
                    self.advance();
                    if self.is_match('/') {
                        return;
                    }
                }
                '\n' => {
                    self.at.line += 1;
                    self.advance();
                }
                '\0' => {
                    log_at!(ERR, *self.at.clone(), "Comment block was not terminated");
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(ch) = self.source.get(self.curr) {
            if *ch != expected {
                return false;
            }
        }

        self.curr += 1;
        self.column += 1;
        true
    }

    fn peek_next(&self) -> char {
        if self.curr + 1 >= self.source.len() {
            return '\0';
        }

        *self.source.get(self.curr + 1).unwrap_or_else(|| {
            log_at!(ERR, *self.at.clone(), "Could not peek_next");
            process::exit(1);
        })
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        *self.source.get(self.curr).unwrap_or_else(|| {
            log_at!(ERR, *self.at.clone(), "Could not peek");
            process::exit(1);
        })
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_literal(token, None);
    }

    fn add_literal(&mut self, token: TokenType, literal: Option<Arg>) {
        let text: String = self.source[self.start..self.curr].iter().collect();
        self.tokens
            .push(Token::new(token, text, self.at.line, self.column, literal))
    }

    fn is_at_end(&self) -> bool {
        self.curr >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let Some(character) = self.source.get(self.curr) else {
            log_at!(ERR, *self.at.clone(), "Unexpected end of file!");
            process::exit(1);
        };
        self.curr += 1;
        self.column += 1;
        *character
    }

    fn is_alpha(ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_alphanumeric(ch: char) -> bool {
        Tokenizer::is_alpha(ch) || Tokenizer::is_digit(ch)
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }
}
