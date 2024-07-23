use crate::ast::{ASTKind, Program, AST};
use crate::report::{Report, ReportKind, ReportLabel, ReportSender, Result, Unbox};
use crate::token::{Token, TokenKind};
use std::cmp::PartialEq;
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
        filename: &'static str,
        tokens: &'t [Token<'contents>],
        sender: ReportSender,
    ) -> Self {
        Self {
            filename,
            tokens,
            current: &tokens[0],
            index: 0,
            sender,
        }
    }

    fn report(&mut self, report: Box<Report>) {
        self.sender.send(report)
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current = &self.tokens[self.index];
    }

    fn consume(&mut self, kind: TokenKind, msg: &'static str) -> Result<&Token> {
        match self.current {
            token if token.kind == kind => {
                self.advance();
                Ok(token)
            }
            Token {
                kind: actual, span, ..
            } => ReportKind::UnexpectedToken
                .new(format!("expected '{kind:?}' got '{actual:?}'"))
                .with_label(ReportLabel::new(span.clone()).with_text(msg))
                .into(),
        }
    }

    fn consume_newline(&mut self) -> Result<()> {
        match self.current {
            Token {
                kind: TokenKind::NewLine | TokenKind::EOF,
                ..
            } => Ok(()),
            Token { kind, span, .. } => ReportKind::UnexpectedToken
                .new(format!("expected NewLine got '{kind:?}'"))
                .with_label(ReportLabel::new(span.clone()))
                .into(),
        }
    }

    fn synchronize(&mut self, until: TokenKind) {
        loop {
            match &self.current.kind {
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
            Ok(AST {
                kind: ASTKind::Block(stmts),
                ..
            }) => stmts,
            Err(err) => {
                self.report(err);
                Vec::new()
            }
            _ => unreachable!("Can't happen nerds"),
        };
        Program {
            stmts,
            filename: self.filename,
        }
    }

    fn parse_block(&mut self, global: bool) -> Result<AST> {
        let mut stmts = Vec::<Box<AST>>::new();
        let until = if global {
            TokenKind::EOF
        } else {
            TokenKind::RBrace
        };
        let start = self.current.span.clone();

        while self.current.kind != until {
            match self.parse_expression() {
                Ok(val) => {
                    stmts.push(val.into());
                    self.consume_newline().map_err(|err| {
                        self.report(err);
                        self.synchronize(until);
                    });
                }
                Err(report) => {
                    self.report(report);
                    self.synchronize(until);
                }
            };
        }

        if !global {
            self.consume(until, "block not terminated");
        };

        let end = start
            .clone()
            .extend(stmts.last().map_or_else(|| &start, |ast| &ast.span));

        Ok(ASTKind::Block(stmts).into_ast(start.extend(&end)))
    }

    fn parse_expression(&mut self) -> Result<AST> {
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> Result<AST> {
        // FIXME: Infinite loop because self.advance cannot be called, need to change to iterator I think. Someone else can handle this.
        match &self.current {
            Token {
                kind: TokenKind::Identifier,
                span,
                text,
            } => {
                self.advance();
                Ok(ASTKind::Identifier(text.to_string()).into_ast(span.clone()))
            }
            Token {
                kind:
                    TokenKind::DecimalIntLiteral
                    | TokenKind::BinaryIntLiteral
                    | TokenKind::OctalIntLiteral
                    | TokenKind::HexadecimalIntLiteral,
                span,
                text,
            } => {
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
            }
            Token {
                kind: TokenKind::EOF,
                span,
                ..
            } => {
                self.advance();
                ReportKind::UnexpectedEOF
                    .new("")
                    .with_label(ReportLabel::new(span.clone()))
                    .into()
            }
            Token { kind, span, .. } => {
                self.advance();
                ReportKind::UnexpectedToken
                    .new(format!("got {kind:?}"))
                    .with_label(ReportLabel::new(span.clone()))
                    .into()
            }
        }
    }
}
