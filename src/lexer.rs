use crate::report::{Report, ReportKind, ReportLabel, ReportSender, Result};
use crate::span::Span;
use crate::token::{Token, TokenKind};
use std::fmt::Display;

pub struct Lexer<'source> {
    filename: &'static str,
    contents: &'source str,
    chars: std::iter::Peekable<std::str::Chars<'source>>,
    current: Option<char>,
    line_number: usize,
    index: usize,
    sender: ReportSender,
    pub tokens: Vec<Token<'source>>,
}

impl<'source> Lexer<'source> {
    pub fn new(filename: &'static str, contents: &'source str, sender: ReportSender) -> Self {
        let mut chars = contents.chars().peekable();
        Self {
            filename,
            contents,
            current: chars.next(),
            chars,
            line_number: 1,
            index: 0,
            sender,
            tokens: Vec::new(),
        }
    }

    fn report(&mut self, report: Box<Report>) {
        self.sender.send(report)
    }

    fn span(&self, line_number: usize, start_index: usize, end_index: usize) -> Span {
        Span {
            filename: self.filename,
            line_number,
            start_index,
            end_index,
        }
    }

    fn slice_source(&self, start: usize, end: usize) -> &'source str {
        &self.contents[start..end]
    }

    fn advance(&mut self) {
        self.current = self.chars.next();
        if Some('\n') == self.current {
            self.line_number += 1;
        }
        self.index += 1;
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    fn push_token(&mut self, kind: TokenKind, span: Span, text: &'source str) {
        self.tokens.push(Token { kind, span, text })
    }

    fn push_simple_token(&mut self, kind: TokenKind, length: usize) {
        let (line_number, start_index) = (self.line_number, self.index);
        for _ in 0..length {
            self.advance();
        }
        self.push_token(
            kind,
            self.span(line_number, start_index, self.index),
            self.slice_source(start_index, self.index),
        );
    }
    pub fn lex_tokens(&mut self) {
        while let Some(current) = self.current {
            let (_line_number, start_index) = (self.line_number, self.index);

            macro_rules! span_to {
                ($end:expr) => {
                    self.span(_line_number, start_index, $end)
                };
            }

            // macro_rules! span_from {
            //     ($start:expr) => {
            //         self.span(line_number, $start, self.index)
            //     };
            // }

            match current {
                '\n' => {
                    while Some('\n') == self.current {
                        self.advance();
                    }
                    self.push_token(TokenKind::NewLine, span_to!(start_index), "")
                }
                char if char.is_whitespace() => self.advance(),
                '/' => match self.peek() {
                    Some('/') => {
                        while Some('\n') != self.current {
                            self.advance()
                        }
                    }
                    Some('*') => {
                        let mut depth = 0;
                        loop {
                            match self.current.clone() {
                                Some('/') if Some('*') == self.peek() => {
                                    self.advance();
                                    self.advance();
                                    depth += 1
                                }
                                Some('*') if Some('/') == self.peek() => {
                                    self.advance();
                                    self.advance();
                                    depth -= 1;
                                }
                                None => break,
                                _ => self.advance(),
                            }
                            if depth == 0 {
                                break;
                            };
                        }
                        if depth > 0 {
                            self.report(
                                ReportKind::UnterminatedMultilineComment
                                    .new(format!("{} comments never terminated", depth))
                                    .with_label(ReportLabel::new(span_to!(self.index)))
                                    .into(),
                            )
                        }
                    }
                    _ => self.push_simple_token(TokenKind::Slash, 1),
                },
                'a'..='z' | 'A'..='Z' => {
                    while let Some(char) = self.current {
                        match char {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                self.advance();
                            }
                            _ => break,
                        }
                    }
                    let ident = self.slice_source(start_index, self.index);
                    let span = span_to!(self.index);
                    let kind = match ident {
                        // Any keywords are handled here
                        "ret" => TokenKind::Ret,
                        _ => TokenKind::Identifier,
                    };
                    self.push_token(kind, span, ident);
                }
                // Literals
                '0' if self.peek().map_or(false, |c| "box".contains(c)) => {
                    let base = match (current, self.peek()) {
                        ('0', Some('b')) => Base::Binary,
                        ('0', Some('o')) => Base::Octal,
                        ('0', Some('x')) => Base::Hexadecimal,
                        _ => unreachable!(),
                    };
                    self.advance();
                    self.advance();
                    if let Err(report) = self.lex_integer(base) {
                        self.report(report);
                        continue;
                    }
                    self.push_token(
                        TokenKind::from(base),
                        span_to!(self.index),
                        self.slice_source(start_index + 2, self.index),
                    );
                }
                '0'..='9' => {
                    if let Err(report) = self.lex_integer(Base::Decimal) {
                        self.report(report);
                        continue;
                    }
                    if let Some('.') = self.current {
                        self.advance();
                        if let Err(report) = self.lex_integer(Base::Decimal) {
                            self.report(report);
                            continue;
                        }
                        if let Some('.') = self.current {
                            self.report(
                                ReportKind::SyntaxError
                                    .new("Invalid Float Literal")
                                    .with_label(ReportLabel::new(self.span(
                                        _line_number,
                                        self.index,
                                        self.index + 1,
                                    )))
                                    .into(),
                            );
                            self.advance();
                            continue;
                        }
                        self.push_token(
                            TokenKind::FloatLiteral,
                            span_to!(self.index),
                            self.slice_source(start_index, self.index),
                        );
                        continue;
                    }
                    self.push_token(
                        TokenKind::DecimalIntLiteral,
                        span_to!(self.index),
                        self.slice_source(start_index, self.index),
                    );
                }
                '.' => match self.peek() {
                    Some('0'..='9') => {
                        self.advance();
                        if let Err(report) = self.lex_integer(Base::Decimal) {
                            self.report(report);
                            continue;
                        }
                        if let Some('.') = self.current {
                            self.report(
                                ReportKind::SyntaxError
                                    .new("Invalid Float Literal")
                                    .with_label(ReportLabel::new(self.span(
                                        _line_number,
                                        self.index,
                                        self.index + 1,
                                    )))
                                    .into(),
                            );
                            self.advance();
                            continue;
                        }
                        self.push_token(
                            TokenKind::FloatLiteral,
                            span_to!(self.index),
                            self.slice_source(start_index, self.index),
                        );
                    }
                    _ => self.push_simple_token(TokenKind::Dot, 1),
                },

                // Characters
                '~' => self.push_simple_token(TokenKind::Tilde, 1),
                '!' => match self.peek() {
                    Some('=') => self.push_simple_token(TokenKind::NotEquals, 2),
                    _ => self.push_simple_token(TokenKind::Bang, 1),
                },
                '@' => self.push_simple_token(TokenKind::At, 1),
                '#' => self.push_simple_token(TokenKind::Pound, 1),
                '$' => self.push_simple_token(TokenKind::Dollar, 1),
                '%' => self.push_simple_token(TokenKind::Percent, 1),
                '^' => self.push_simple_token(TokenKind::Caret, 1),
                '&' => self.push_simple_token(TokenKind::Ampersand, 1),
                '*' => self.push_simple_token(TokenKind::Star, 1),
                '(' => self.push_simple_token(TokenKind::LParen, 1),
                ')' => self.push_simple_token(TokenKind::RParen, 1),
                '-' => match self.peek() {
                    Some('>') => self.push_simple_token(TokenKind::ArrowRight, 2),
                    Some('-') => self.push_simple_token(TokenKind::MinusMinus, 2),
                    _ => self.push_simple_token(TokenKind::Minus, 1),
                },
                '_' => self.push_simple_token(TokenKind::Underscore, 1),
                '+' => match self.peek() {
                    Some('+') => self.push_simple_token(TokenKind::PlusPlus, 2),
                    _ => self.push_simple_token(TokenKind::Plus, 1),
                },
                '[' => self.push_simple_token(TokenKind::LBracket, 1),
                ']' => self.push_simple_token(TokenKind::RBracket, 1),
                '{' => self.push_simple_token(TokenKind::LBrace, 1),
                '}' => self.push_simple_token(TokenKind::RBrace, 1),
                '|' => self.push_simple_token(TokenKind::Pipe, 1),
                ';' => self.push_simple_token(TokenKind::Semicolon, 1),
                ':' => self.push_simple_token(TokenKind::Colon, 1),
                ',' => self.push_simple_token(TokenKind::Comma, 1),
                '=' => match self.peek() {
                    Some('=') => self.push_simple_token(TokenKind::EqualsEquals, 2),
                    Some('>') => self.push_simple_token(TokenKind::FatArrowRight, 2),
                    _ => self.push_simple_token(TokenKind::Equals, 1),
                },
                '<' => match self.peek() {
                    Some('=') => self.push_simple_token(TokenKind::LessThanEquals, 2),
                    Some('-') => self.push_simple_token(TokenKind::ArrowLeft, 2),
                    _ => self.push_simple_token(TokenKind::LessThan, 1),
                },
                '>' => match self.peek() {
                    Some('=') => self.push_simple_token(TokenKind::GreaterThanEquals, 2),
                    _ => self.push_simple_token(TokenKind::GreaterThan, 1),
                },
                '?' => self.push_simple_token(TokenKind::Question, 1),

                c => {
                    self.advance();
                    self.report(
                        ReportKind::UnexpectedCharacter
                            .new(format!("{}", c))
                            .with_label(ReportLabel::new(span_to!(self.index)))
                            .into(),
                    );
                }
            };
        }
        self.push_token(
            TokenKind::EOF,
            self.span(self.line_number, self.index, self.index),
            "",
        );
    }

    fn lex_integer(&mut self, base: Base) -> Result<()> {
        // use slices instead
        while let Some(char) = self.current {
            match (base, char.to_ascii_lowercase()) {
                (Base::Binary, '0'..='1')
                | (Base::Octal, '0'..='7')
                | (Base::Decimal, '0'..='9')
                | (Base::Hexadecimal, '0'..='9' | 'a'..='f') => {
                    self.advance();
                }
                (_, '0'..='9' | 'a'..='z') => {
                    return ReportKind::SyntaxError
                        .new("Invalid Integer Literal")
                        .with_label(
                            ReportLabel::new(self.span(
                                self.line_number,
                                self.index,
                                self.index + 1,
                            ))
                            .with_text(format!(
                                "'{}' not valid for {} Integer Literal",
                                char, base
                            )),
                        )
                        .into();
                }
                (_, '_') => self.advance(),
                _ => break,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Base::Binary => "Binary",
            Base::Octal => "Octal",
            Base::Decimal => "Decimal",
            Base::Hexadecimal => "Hexadecimal",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

impl From<Base> for TokenKind {
    fn from(value: Base) -> Self {
        match value {
            Base::Binary => Self::BinaryIntLiteral,
            Base::Octal => Self::OctalIntLiteral,
            Base::Decimal => Self::DecimalIntLiteral,
            Base::Hexadecimal => Self::HexadecimalIntLiteral,
        }
    }
}
