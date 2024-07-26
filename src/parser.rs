use crate::ast::{ASTKind, Program, AST, Type, Tag, LabelAttribute};
use crate::report::{Report, ReportKind, ReportLabel, ReportSender, Result, Unbox};
use crate::token::{Token, TokenKind};
use std::cmp::PartialEq;
use std::str;
use std::slice::Iter;

pub struct Parser<'t, 'contents> {
    filename: &'static str,
    tokens: &'t [Token<'contents>],
    current: &'t Token<'contents>,
    index: usize,
    sender: ReportSender,
}

impl<'t, 'contents> Parser<'t, 'contents> {
    pub fn new(
        filename: &'static str, tokens: &'t [Token<'contents>], sender: ReportSender,
    ) -> Self {
        Self { filename, tokens, current: &tokens[0], index: 0, sender }
    }

    fn report(&mut self, report: Box<Report>) {
        self.sender.send(report)
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current = &self.tokens[self.index];
    }

    fn nth(&self, index: usize) -> &Token {
        &self.tokens[self.index + index]
    }

    fn consume(&mut self, kind: TokenKind, msg: &'static str) -> Result<&Token> {
        let Token { kind: actual, span, .. } = self.current;
        match actual {
            k if k == &kind => {
                self.advance();
                Ok(self.current)
            },
            actual => ReportKind::UnexpectedToken
                .new(format!("expected '{kind:?}' got '{actual:?}'"))
                .with_label(ReportLabel::new(span.clone()).with_text(msg))
                .into(),
        }
    }

    fn consume_newline(&mut self) -> Result<()> {
        let Token { kind, span, .. } = self.current;
        match kind {
            TokenKind::NewLine | TokenKind::EOF => {
                self.advance();
                Ok(())
            },
            _ => ReportKind::UnexpectedToken
                .new(format!("expected NewLine got '{kind:?}'"))
                .with_label(ReportLabel::new(span.clone()))
                .into(),
        }
    }

    fn synchronize(&mut self, until: TokenKind) {
        loop {
            let token = &self.current.kind;

            if token != &TokenKind::EOF {
                self.advance();
            }

            match token {
                kind if kind == &until => break,
                TokenKind::NewLine => break,
                TokenKind::EOF => return,
                _ => continue,
            }
        }
        return;
    }

    pub fn parse(&mut self) -> Program {
        let stmts = match self.parse_block(true) {
            Ok(AST { kind: ASTKind::Block(stmts), .. }) => stmts,
            Err(err) => {
                self.report(err);
                Vec::new()
            },
            _ => unreachable!("Can't happen nerds"),
        };
        Program { stmts, filename: self.filename }
    }

    fn parse_block(&mut self, global: bool) -> Result<AST> {
        let mut stmts = Vec::<Box<AST>>::new();
        let until = if global { TokenKind::EOF } else { TokenKind::RBrace };
        let start = self.current.span.clone();

        while self.current.kind != until {
            match self.parse_statement() {
                Ok(val) => {
                    stmts.push(val.into());
                    self.consume_newline().map_err(|err| {
                        self.report(err);
                        self.synchronize(until);
                    });
                },
                Err(report) => {
                    self.report(report);
                    self.synchronize(until);
                },
            };
        }

        if !global {
            self.consume(until, "block not terminated");
        };

        let end = start.clone()
            .extend(stmts.last()
            .map_or_else(|| &start, |ast| &ast.span));

        Ok(ASTKind::Block(stmts).into_ast(start.extend(&end)))
    }

    fn parse_statement(&mut self) -> Result<AST> {
        match self.current.kind {
            TokenKind::Colon => self.parse_tag(),
            TokenKind::Star  => self.parse_interrupt(),
            // TokenKind::Identifier => self.parse_label(),
            TokenKind::Ret   => self.parse_return(),
            _ => self.parse_expression(),
        }
    }

    fn parse_return(&mut self) -> Result<AST> {
        if self.nth(1).kind == TokenKind::NewLine {
            self.advance();
            return Ok(ASTKind::Return(None).into_ast(self.current.span.clone()));
        }

        self.advance();
        let expr = self.parse_expression()?;
        Ok(ASTKind::Return(Some(expr.into())).into_ast(self.current.span.clone()))
    }

    fn parse_interrupt(&mut self) -> Result<AST> {
        // syscall
        if self.nth(1).kind == TokenKind::Identifier {
            self.advance();
            todo!("parse syscall")
        }

        self.advance();
        match self.parse_expression()? {
            AST { kind: ASTKind::IntegerLiteral(val), .. } => {
                Ok(ASTKind::Interrupt(val).into_ast(self.current.span.clone()))
            },
            _ => ReportKind::SyntaxError
                .new("Expected Integer Literal")
                .with_label(ReportLabel::new(self.current.span.clone()))
                .into(),
        }
    }


    fn parse_tag(&mut self) -> Result<AST> {
        self.advance();
        if self.current.kind != TokenKind::Identifier {
            return ReportKind::UnexpectedToken
                .new("Expected Identifier")
                .with_label(ReportLabel::new(self.current.span.clone()))
                .into();
        }

        match self.current.text {
            "name" => {
                self.advance();
                match self.current.kind {
                    TokenKind::StringLiteral => {
                        let Token { text, span, .. } = self.current;
                        self.advance();
                        Ok(ASTKind::Tag(Tag::Name(text.to_string())).into_ast(span.clone()))
                    },
                    _ => ReportKind::UnexpectedToken
                        .new("Expected String Literal")
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .into(),
                }
            },

            "arch" => {
                self.advance();

                let mut arch_args = Vec::with_capacity(2);
                while self.current.kind != TokenKind::NewLine{
                    match self.current.kind {
                        TokenKind::Identifier => arch_args.push(self.current.text.to_string()),
                        _ => {
                            let span = self.nth(1).span.clone();
                            self.advance();
                            return ReportKind::SyntaxError
                                .new("Expected Identifier")
                                .with_label(ReportLabel::new(span))
                                .into();
                        },
                    }
                    self.advance();
                }

                if arch_args.is_empty() {
                    return ReportKind::SyntaxError
                        .new("Expected at least one argument")
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .into();
                }

                Ok(ASTKind::Tag(Tag::Arch(arch_args)).into_ast(self.current.span.clone()))
            },

            text => ReportKind::InvalidTag
                .new(format!("{text:?}"))
                .with_label(ReportLabel::new(self.current.span.clone()))
                .into(),
        }
    }

    fn parse_expression(&mut self) -> Result<AST> {
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> Result<AST> {
        let Token{kind, span, text} = self.current;

        match &kind {
            TokenKind::Identifier => {
                self.advance();
                Ok(ASTKind::Identifier(text.to_string()).into_ast(span.clone()))
            },

            TokenKind::DecimalIntLiteral
            | TokenKind::BinaryIntLiteral
            | TokenKind::OctalIntLiteral
            | TokenKind::HexadecimalIntLiteral => {
                let base = match self.current.kind {
                    TokenKind::DecimalIntLiteral => 10,
                    TokenKind::BinaryIntLiteral => 2,
                    TokenKind::OctalIntLiteral => 8,
                    TokenKind::HexadecimalIntLiteral => 16,
                    _ => unreachable!(),
                };
                self.advance();
                match usize::from_str_radix(text, base) {
                    Ok(val) => Ok((ASTKind::IntegerLiteral(val).into_ast(span.clone()))),
                    Err(_) => ReportKind::SyntaxError
                        .new("Invalid {} Integer Literal")
                        .with_label(ReportLabel::new(span.clone()))
                        .into(),
                }
            },

            TokenKind::StringLiteral => { // FIXME: this prob isnt the best way to do this :/
                let text_bytes = text.as_bytes();
                let text_len = text_bytes.len();

                let mut text = String::with_capacity(text_len);
                for (i, window) in text_bytes.windows(2).enumerate() {
                    match window[0] as char {
                        '\\' => text.push(Self::parse_escape(str::from_utf8(window).unwrap(), span)?),
                        _ if i+1*2 >= text_len => text.push_str(str::from_utf8(window).unwrap()),
                        _ => text.push(window[0] as char),
                    }
                }

                if text_len == 1 {
                    text.push(text_bytes[0] as char);
                }

                self.advance();
                Ok(ASTKind::StringLiteral(text).into_ast(span.clone()))
            },

            TokenKind::CharLiteral => {
                self.advance();
                Ok(ASTKind::CharLiteral(Self::parse_escape(text, span)?).into_ast(span.clone()))
            },

            TokenKind::EOF => ReportKind::UnexpectedEOF.new("").into(),

            kind => {
                self.advance();
                ReportKind::UnexpectedToken
                    .new(format!("got {kind:?}"))
                    .with_label(ReportLabel::new(span.clone()))
                    .into()
            },
        }
    }


    fn parse_escape(text: &str, span: &crate::span::Span) -> Result<char> {
        Ok((match text {
            "\\0" | "\\@" => 0,
            "\\A"         => 1,
            "\\B"         => 2,
            "\\C"         => 3,
            "\\D"         => 4,
            "\\E"         => 5,
            "\\F"         => 6,
            "\\G" | "\\a" => 7,
            "\\H" | "\\b" => 8,
            "\\I" | "\\t" => 9,
            "\\J" | "\\n" => 10,
            "\\K" | "\\v" => 11,
            "\\L" | "\\f" => 12,
            "\\M" | "\\r" => 13,
            "\\N"         => 14,
            "\\O"         => 15,
            "\\P"         => 16,
            "\\Q"         => 17,
            "\\R"         => 18,
            "\\S"         => 19,
            "\\T"         => 20,
            "\\U"         => 21,
            "\\V"         => 22,
            "\\W"         => 23,
            "\\X"         => 24,
            "\\Y"         => 25,
            "\\Z"         => 26,
            "\\[" | "\\e" => 27,
            "\\/"         => 28,
            "\\]"         => 29,
            "\\^"         => 30,
            "\\_"         => 31,
            "\\?"         => 32,
            "\\"          => '\\' as u8,
            "\\`"         => '`'  as u8,
            s if s.len() > 1 => 
                return ReportKind::InvalidEscapeSequence
                    .new("")
                    .with_label(ReportLabel::new(span.clone()))
                    .into(),
            s => s.as_bytes()[0],
        }) as char)
    }
}
