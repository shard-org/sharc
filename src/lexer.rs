use std::fmt::Display;

use iterlist::IterList;

use crate::report::{LogHandler, Report, ReportKind};
use crate::span::Span;
use crate::token::{Token, TokenKind};

pub struct Lexer<'source> {
    contents: &'source str,
    index: usize,
    span: Span,

    handler: LogHandler,
    pub tokens: IterList<Token<'source>>,
}

impl<'source> Lexer<'source> {
    pub fn new(filename: &'static str, contents: &'source str, handler: LogHandler) -> Self {
        Self {
            contents,
            handler,
            index: 0,
            span: Span::new(filename, 1, 0, 0),
            tokens: IterList::new(),
        }
    }

    fn report(&self, report: Report) {
        let (priority, log) = report.into();
        self.handler.add_log(priority, log);
    }

    fn current(&self) -> Option<&'source str> {
        self.contents.get(self.index..=self.index)
    }

    fn peek(&self) -> Option<&'source str> {
        self.contents.get(self.index + 1..=self.index + 1)
    }

    fn slice_source(&self, index: usize, len: usize) -> &'source str {
        &self.contents[index..index + len]
    }

    fn push_token(&mut self, kind: TokenKind, span: Span, text: &'source str) {
        self.tokens.push_next(Token { kind, span, text });
    }

    fn push_token_simple(&mut self, kind: TokenKind, length: usize) {
        let (index, span) = (self.index, self.span);

        (0..length).for_each(|_| self.advance());
        self.push_token(kind, span.len(length), self.slice_source(index, length));
    }

    fn advance(&mut self) {
        self.index += 1;
        self.span.offset += 1;

        if Some("\n") == self.current() {
            self.span.line_number += 1;
            self.span.offset = 0;
        }
    }

    pub fn lex_tokens(&mut self) {
        'outer: while let Some(current) = self.current() {
            let (index, span) = (self.index, self.span);

            let (token, len) = match current {
                "\n" => {
                    while Some("\n") == self.current() {
                        self.advance();
                    }

                    if self.tokens.get_cursor().is_some_and(|t| t.kind != TokenKind::NewLine) {
                        self.push_token(TokenKind::NewLine, self.span, "");
                    }
                    continue;
                },

                c if c.chars().any(char::is_whitespace) => {
                    self.advance();
                    continue;
                },

                "/" => match self.peek() {
                    Some("/") => {
                        while Some("\n") != self.current() {
                            self.advance();
                        }
                        continue;
                    },
                    Some("*") => {
                        let mut depth = 0;
                        loop {
                            match self.current() {
                                Some("/") if Some("*") == self.peek() => {
                                    self.advance();
                                    self.advance();
                                    depth += 1;
                                },
                                Some("*") if Some("/") == self.peek() => {
                                    self.advance();
                                    self.advance();
                                    depth -= 1;
                                },
                                None => break,
                                _ => self.advance(),
                            }

                            if depth == 0 {
                                break;
                            }
                        }

                        if depth > 0 {
                            self.report(
                                ReportKind::UnterminatedMultilineComment
                                    .title(format!("{depth} comments never terminated"))
                                    .span(self.span),
                            );
                        }

                        continue;
                    },
                    _ => (TokenKind::Slash, 1),
                },

                c if c.chars().any(|c| c.is_ascii_alphabetic()) => {
                    while let Some(c) = self.current() {
                        if c.chars().any(|c| c.is_ascii_alphanumeric() || c == '_') {
                            self.advance();
                            continue;
                        }
                        break;
                    }

                    let ident = self.slice_source(index, self.index - index);
                    let kind = match ident {
                        "ret" => TokenKind::KeywordRet,
                        "struct" => TokenKind::KeywordStruct,
                        "enum" => TokenKind::KeywordEnum,
                        "destr" => TokenKind::KeywordDestr,
                        "type" => TokenKind::KeywordType,
                        "op" => TokenKind::KeywordOp,
                        "cast" => TokenKind::KeywordCast,
                        "extern" => TokenKind::KeywordExtern,
                        _ => TokenKind::Identifier,
                    };

                    self.push_token(kind, span.len(self.index - index), ident);
                    continue;
                },

                "\"" => {
                    self.advance();
                    let span = self.span;
                    while let Some(c) = self.current() {
                        match c {
                            "\"" => break,
                            "\\" => {
                                self.advance();
                                if self.current() == Some("\"") {
                                    self.advance();
                                }
                            },
                            "\n" => {
                                self.report(
                                    ReportKind::UnterminatedStringLiteral
                                        .untitled()
                                        .span(span.offset(span.offset - 2).len(self.index - index)),
                                );
                                continue 'outer;
                            },
                            _ => self.advance(),
                        }
                    }
                    let span = span.len(self.index - (index + 1));
                    self.push_token(
                        TokenKind::StringLiteral,
                        span,
                        self.slice_source(index + 1, self.index - (index + 1)),
                    );

                    self.advance();

                    continue;
                },

                "`" => {
                    self.advance();
                    let start = self.index;
                    while let Some(c) = self.current() {
                        match c {
                            "`" => {
                                if self.index == start {
                                    self.report(
                                        ReportKind::EmptyCharLiteral
                                            .untitled()
                                            .span(span.len(2).offset(span.offset - 1)),
                                    );
                                    self.advance();
                                    continue 'outer;
                                }

                                self.advance();
                                break;
                            },

                            "\\" => {
                                self.advance();
                                if self.current() == Some("`") && self.peek() != Some("`") {
                                    self.advance();
                                    self.report(
                                        ReportKind::UnterminatedCharLiteral
                                            .untitled()
                                            .span(span.len(self.index - index))
                                            .help("Remove the escape character"),
                                    );
                                    continue 'outer;
                                }

                                self.advance();
                            },

                            "\n" => {
                                self.report(
                                    ReportKind::UnterminatedCharLiteral
                                        .untitled()
                                        .span(span.len(self.index - index).offset(span.offset - 1)),
                                );
                                continue 'outer;
                            },

                            _ => self.advance(),
                        }
                    }

                    self.push_token(
                        TokenKind::CharLiteral,
                        span.len(self.index - index),
                        self.slice_source(index, self.index - index),
                    );

                    continue;
                },

                "0" if self.peek().filter(|c| "box".contains(c)).is_some() => {
                    let (kind, base) = match self.peek() {
                        Some("b") => (TokenKind::BinaryIntLiteral, 2),
                        Some("o") => (TokenKind::OctalIntLiteral, 8),
                        Some("x") => (TokenKind::HexadecimalIntLiteral, 16),
                        _ => unreachable!(),
                    };

                    self.advance();
                    self.advance();
                    if !self.lex_integer(base) {
                        continue;
                    }

                    self.push_token(
                        kind,
                        self.span.len(self.index - index),
                        self.slice_source(index, self.index - index),
                    );

                    continue;
                },

                c if c.chars().any(|c| c.is_ascii_digit()) => {
                    if !self.lex_integer(10) {
                        continue;
                    }

                    if self.current() == Some(".") {
                        self.advance();
                        if !self.lex_integer(10) {
                            continue;
                        }

                        if self.current() == Some(".") {
                            self.report(
                                ReportKind::SyntaxError
                                    .title("Invalid Float Literal")
                                    .span(self.span.len(1)),
                            );
                            self.advance();
                            continue;
                        }

                        self.push_token(
                            TokenKind::FloatLiteral,
                            span.len(self.index - index),
                            self.slice_source(index, self.index - index),
                        );

                        continue;
                    }

                    self.push_token(
                        TokenKind::DecimalIntLiteral,
                        span.len(self.index - index),
                        self.slice_source(index, self.index - index),
                    );
                    continue;
                },

                "." => (TokenKind::Dot, 1),
                "'" => (TokenKind::Apostrophe, 1),
                "~" => match self.peek() {
                    Some("=") => (TokenKind::NotEquals, 2),
                    _ => (TokenKind::Tilde, 1),
                },
                "!" => (TokenKind::Bang, 1),
                "@" => (TokenKind::At, 1),
                "#" => (TokenKind::Pound, 1),
                "$" => (TokenKind::Dollar, 1),
                "%" => (TokenKind::Percent, 1),
                "^" => match self.peek() {
                    Some("^") => (TokenKind::CaretCaret, 2),
                    _ => (TokenKind::Caret, 1),
                },
                "&" => match self.peek() {
                    Some("&") => (TokenKind::AmpersandAmpersand, 2),
                    _ => (TokenKind::Ampersand, 1),
                },
                "*" => (TokenKind::Star, 1),
                "(" => (TokenKind::LParen, 1),
                ")" => (TokenKind::RParen, 1),
                "-" => match self.peek() {
                    Some(">") => (TokenKind::ArrowRight, 2),
                    Some("-") => (TokenKind::MinusMinus, 2),
                    _ => (TokenKind::Minus, 1),
                },
                "_" => (TokenKind::Underscore, 1),
                "+" => match self.peek() {
                    Some("+") => (TokenKind::PlusPlus, 2),
                    _ => (TokenKind::Plus, 1),
                },
                "[" => (TokenKind::LBracket, 1),
                "]" => (TokenKind::RBracket, 1),
                "{" => (TokenKind::LBrace, 1),
                "}" => (TokenKind::RBrace, 1),
                "|" => match self.peek() {
                    Some("|") => (TokenKind::PipePipe, 2),
                    _ => (TokenKind::Pipe, 1),
                },
                ";" => (TokenKind::Semicolon, 1),
                ":" => (TokenKind::Colon, 1),
                "," => (TokenKind::Comma, 1),
                "=" => match self.peek() {
                    // Some("=") => (TokenKind::EqualsEquals, 2),
                    Some(">") => (TokenKind::FatArrowRight, 2),
                    _ => (TokenKind::Equals, 1),
                },
                "<" => match self.peek() {
                    Some("=") => (TokenKind::LessThanEquals, 2),
                    Some("-") => (TokenKind::ArrowLeft, 2),
                    Some("<") => (TokenKind::ShiftLeft, 2),
                    _ => (TokenKind::LessThan, 1),
                },
                ">" => match self.peek() {
                    Some("=") => (TokenKind::GreaterThanEquals, 2),
                    Some(">") => (TokenKind::ShiftRight, 2),
                    _ => (TokenKind::GreaterThan, 1),
                },
                "?" => (TokenKind::Question, 1),

                c => {
                    self.report(ReportKind::UnexpectedCharacter.title(c).span(self.span));
                    self.advance();
                    continue;
                },
            };
            self.push_token_simple(token, len);
        }

        self.push_token(TokenKind::EOF, self.span, "");
    }

    fn lex_integer(&mut self, base: usize) -> bool {
        const CHARS: [char; 16] =
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

        while let Some(c) = self.current() {
            match (base, c.to_ascii_lowercase().chars().next().unwrap()) {
                (2, c) if CHARS[..1].contains(&c) => self.advance(),
                (8, c) if CHARS[..7].contains(&c) => self.advance(),
                (10, c) if CHARS[..9].contains(&c) => self.advance(),
                (16, c) if CHARS.contains(&c) => self.advance(),
                (_, '_') => self.advance(),

                (_, c) if c.is_ascii_alphanumeric() => {
                    self.report(
                        ReportKind::SyntaxError
                            .title("Invalid Integer Literal")
                            .span(self.span.len(1).offset(self.span.offset - 1))
                            .label(format!("{c:?} not valid for base{base} Integer Literal")),
                    );
                    return false;
                },

                _ => break,
            }
        }
        true
    }
}
