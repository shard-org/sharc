use std::fmt::Display;

use iterlist::IterList;

use crate::report::{Report, ReportKind, ReportLabel, ReportSender, Result};
use crate::span::Span;
use crate::token::{Token, TokenKind};

pub struct Lexer<'source> {
    filename:    &'static str,
    contents:    &'source str,
    chars:       std::iter::Peekable<std::str::Chars<'source>>,
    current:     Option<char>,
    line_number: usize,
    index:       usize,
    sender:      ReportSender,
    pub tokens:  IterList<Token<'source>>,
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
            tokens: IterList::new(),
        }
    }

    fn report(&self, report: Box<Report>) {
        self.sender.send(report);
    }

    fn span(&self, line_number: usize, start_index: usize, end_index: usize) -> Span {
        Span { filename: self.filename, line_number, start_index, end_index }
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
        self.chars.peek().copied()
    }

    fn push_token(&mut self, kind: TokenKind, span: Span, text: &'source str) {
        self.tokens.push_next(Token { kind, span, text });
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
        'main: while let Some(current) = self.current {
            let (line_number, start_index) = (self.line_number, self.index);

            macro_rules! span_to {
                ($end:expr) => {
                    self.span(line_number, start_index, $end)
                };
            }

            // macro_rules! span_from {
            //     ($start:expr) => {
            //         self.span(line_number, $start, self.index)
            //     };
            // }

            let (token, len) = match current {
                '\n' => {
                    while Some('\n') == self.current {
                        self.advance();
                    }

                    if self
                        .tokens
                        .get_offset(-1)
                        .is_some_and(|token| token.kind != TokenKind::NewLine)
                    {
                        self.push_token(TokenKind::NewLine, span_to!(start_index), "");
                    }
                    continue;
                },
                char if char.is_whitespace() => {
                    self.advance();
                    continue;
                },
                '/' => match self.peek() {
                    Some('/') => {
                        while Some('\n') != self.current {
                            self.advance();
                        }
                        continue;
                    },
                    Some('*') => {
                        let mut depth = 0;
                        loop {
                            let current = self.current;
                            match current {
                                Some('/') if Some('*') == self.peek() => {
                                    self.advance();
                                    self.advance();
                                    depth += 1;
                                },
                                Some('*') if Some('/') == self.peek() => {
                                    self.advance();
                                    self.advance();
                                    depth -= 1;
                                },
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
                                    .new(format!("{depth} comments never terminated"))
                                    .with_label(ReportLabel::new(span_to!(self.index)))
                                    .into(),
                            );
                        }
                        continue;
                    },
                    _ => (TokenKind::Slash, 1),
                },
                'a'..='z' | 'A'..='Z' => {
                    while let Some(char) = self.current {
                        match char {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                self.advance();
                            },
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
                    continue;
                },
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
                    continue;
                },
                '0'..='9' => {
                    if let Err(report) = self.lex_integer(Base::Decimal) {
                        self.report(report);
                        continue;
                    }
                    if self.current == Some('.') {
                        self.advance();
                        if let Err(report) = self.lex_integer(Base::Decimal) {
                            self.report(report);
                            continue;
                        }
                        if self.current == Some('.') {
                            self.report(
                                ReportKind::SyntaxError
                                    .new("Invalid Float Literal")
                                    .with_label(ReportLabel::new(self.span(
                                        line_number,
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
                    continue;
                },
                '"' => {
                    self.advance();
                    while let Some(char) = self.current {
                        match char {
                            '"' => {
                                self.advance();
                                break;
                            },
                            '\\' => {
                                self.advance();
                                if self.current == Some('"') {
                                    self.advance();
                                }
                            },
                            '\n' => {
                                self.report(
                                    ReportKind::UnterminatedStringLiteral
                                        .new("")
                                        .with_label(ReportLabel::new(span_to!(self.index)))
                                        .into(),
                                );
                                continue 'main;
                            },
                            _ => self.advance(),
                        }
                    }
                    self.push_token(
                        TokenKind::StringLiteral,
                        span_to!(self.index),
                        self.slice_source(start_index + 1, self.index - 1),
                    );
                    continue;
                },
                '`' => {
                    self.advance();
                    while let Some(char) = self.current {
                        match char {
                            '`' => {
                                self.advance();
                                break;
                            },
                            '\\' => {
                                self.advance();
                                if self.current == Some('`') && self.peek() != Some('`') {
                                    self.advance();
                                    self.report(
                                        ReportKind::UnterminatedCharLiteral
                                            .new("")
                                            .with_label(ReportLabel::new(span_to!(self.index)))
                                            .with_note("help: Remove the escape character")
                                            .into(),
                                    );
                                    continue 'main;
                                }

                                self.advance();
                            },
                            '\n' => {
                                self.report(
                                    ReportKind::UnterminatedCharLiteral
                                        .new("")
                                        .with_label(ReportLabel::new(span_to!(self.index)))
                                        .into(),
                                );
                                continue 'main;
                            },
                            _ => self.advance(),
                        }
                    }
                    self.push_token(
                        TokenKind::CharLiteral,
                        span_to!(self.index),
                        self.slice_source(start_index + 1, self.index - 1),
                    );
                    continue;
                },
                // '.' => match self.peek() {
                //     Some('0'..='9') => {
                //         self.advance();
                //         if let Err(report) = self.lex_integer(Base::Decimal) {
                //             self.report(report);
                //             continue;
                //         }
                //         if let Some('.') = self.current {
                //             self.report(
                //                 ReportKind::SyntaxError
                //                     .new("Invalid Float Literal")
                //                     .with_label(ReportLabel::new(self.span(
                //                         _line_number,
                //                         self.index,
                //                         self.index + 1,
                //                     )))
                //                     .into(),
                //             );
                //             self.advance();
                //             continue;
                //         }
                //         self.push_token(
                //             TokenKind::FloatLiteral,
                //             span_to!(self.index),
                //             self.slice_source(start_index, self.index),
                //         );
                //         continue;
                //     },
                //     _ => (TokenKind::Dot, 1),
                // },

                // Characters
                '.' => (TokenKind::Dot, 1),
                '~' => match self.peek() {
                    Some('=') => (TokenKind::NotEquals, 2),
                    _ => (TokenKind::Tilde, 1),
                },
                '!' => (TokenKind::Bang, 1),
                '@' => (TokenKind::At, 1),
                '#' => (TokenKind::Pound, 1),
                '$' => (TokenKind::Dollar, 1),
                '%' => (TokenKind::Percent, 1),
                '^' => match self.peek() {
                    Some('^') => (TokenKind::CaretCaret, 2),
                    _ => (TokenKind::Caret, 1),
                },
                '&' => match self.peek() {
                    Some('&') => (TokenKind::AmpersandAmpersand, 2),
                    _ => (TokenKind::Ampersand, 1),
                },
                '*' => (TokenKind::Star, 1),
                '(' => (TokenKind::LParen, 1),
                ')' => (TokenKind::RParen, 1),
                '-' => match self.peek() {
                    Some('>') => (TokenKind::ArrowRight, 2),
                    Some('-') => (TokenKind::MinusMinus, 2),
                    _ => (TokenKind::Minus, 1),
                },
                '_' => (TokenKind::Underscore, 1),
                '+' => match self.peek() {
                    Some('+') => (TokenKind::PlusPlus, 2),
                    _ => (TokenKind::Plus, 1),
                },
                '[' => (TokenKind::LBracket, 1),
                ']' => (TokenKind::RBracket, 1),
                '{' => (TokenKind::LBrace, 1),
                '}' => (TokenKind::RBrace, 1),
                '|' => match self.peek() {
                    Some('|') => (TokenKind::PipePipe, 2),
                    _ => (TokenKind::Pipe, 1),
                },
                ';' => (TokenKind::Semicolon, 1),
                ':' => (TokenKind::Colon, 1),
                ',' => (TokenKind::Comma, 1),
                '=' => match self.peek() {
                    Some('=') => (TokenKind::EqualsEquals, 2),
                    Some('>') => (TokenKind::FatArrowRight, 2),
                    _ => (TokenKind::Equals, 1),
                },
                '<' => match self.peek() {
                    Some('=') => (TokenKind::LessThanEquals, 2),
                    Some('-') => (TokenKind::ArrowLeft, 2),
                    _ => (TokenKind::LessThan, 1),
                },
                '>' => match self.peek() {
                    Some('=') => (TokenKind::GreaterThanEquals, 2),
                    _ => (TokenKind::GreaterThan, 1),
                },
                '?' => (TokenKind::Question, 1),

                c => {
                    self.advance();
                    self.report(
                        ReportKind::UnexpectedCharacter
                            .new(format!("{c}"))
                            .with_label(ReportLabel::new(span_to!(self.index)))
                            .into(),
                    );
                    continue;
                },
            };
            self.push_simple_token(token, len);
        }

        self.push_token(TokenKind::EOF, self.span(self.line_number, self.index, self.index), "");
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
                },
                (_, '0'..='9' | 'a'..='z') => {
                    return ReportKind::SyntaxError
                        .new("Invalid Integer Literal")
                        .with_label(
                            ReportLabel::new(self.span(
                                self.line_number,
                                self.index,
                                self.index + 1,
                            ))
                            .with_text(format!("'{char}' not valid for {base} Integer Literal")),
                        )
                        .into();
                },
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
            Self::Binary => "Binary",
            Self::Octal => "Octal",
            Self::Decimal => "Decimal",
            Self::Hexadecimal => "Hexadecimal",
        }
        .to_string();
        write!(f, "{str}")
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
