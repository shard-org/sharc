use crate::ast::{ASTKind, Program, AST};
use crate::error::{Error, ErrorKind, ErrorLabel, ErrorSender, Result, Unbox};
use crate::token::{Token, TokenKind};
use std::cmp::PartialEq;
use std::slice::Iter;

pub struct Parser<'contents> {
    filename: &'static str,
    tokens: Vec<Token<'contents>>,
    sender: ErrorSender,
    index: usize,
}

impl<'contents> Parser<'contents> {
    pub fn new(filename: &'static str, tokens: Vec<Token<'contents>>, sender: ErrorSender) -> Self {
        let mut iter = tokens.iter();
        Self {
            filename,
            tokens,
            sender,
            index: 0,
        }
    }

    fn error(&mut self, error: Box<Error>) {
        self.sender.send(error)
    }

    fn current(&self) -> &Token<'contents> {
        &self.tokens[self.index]
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn consume(&mut self, kind: TokenKind, msg: &'static str) -> Result<()> {
        match self.current() {
            token if token.kind == kind => Ok(()),
            Token {
                kind: actual, span, ..
            } => ErrorKind::UnexpectedToken
                .new(format!("expected '{kind:?}' got '{actual:?}'"))
                .with_label(ErrorLabel::new(span.clone()).with_text(msg))
                .into(),
        }
    }

    fn consume_newline(&mut self) -> Result<()> {
        match self.current() {
            Token {
                kind: TokenKind::NewLine | TokenKind::EOF,
                ..
            } => Ok(()),
            Token { kind, span, .. } => ErrorKind::UnexpectedToken
                .new(format!("expected NewLine got '{kind:?}'"))
                .with_label(ErrorLabel::new(span.clone()))
                .into(),
        }
    }

    fn synchronize(&mut self) {
        loop {
            match self.current().kind {
                TokenKind::NewLine => break,
                TokenKind::EOF => break,
                _ => continue,
            }
        }
        return;
    }

    pub fn parse(&mut self) -> Program {
        let stmts = match self.parse_block(TokenKind::EOF) {
            Ok(AST {
                kind: ASTKind::Block(stmts),
                ..
            }) => stmts,
            Err(err) => {
                self.error(err);
                Vec::new()
            }
            _ => unreachable!("Can't happen nerds"),
        };
        Program {
            stmts,
            filename: self.filename,
        }
    }

    fn parse_block(&mut self, until: TokenKind) -> Result<AST> {
        let mut stmts = Vec::<Box<AST>>::new();
        let start = self.current().span.clone();

        while self.current().kind != until || self.current().kind != TokenKind::EOF {
            match self.parse_expression() {
                Ok(val) => {
                    stmts.push(val.into());
                    self.consume_newline().map_err(|err| {
                        self.error(err);
                        self.synchronize();
                    });
                }
                Err(error) => {
                    self.error(error);
                    self.synchronize();
                }
            };
        }
        self.consume(until, "");
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
        match &self.current() {
            Token {
                kind: TokenKind::Identifier,
                span,
                text,
            } => Ok(ASTKind::Identifier(text.to_string()).into_ast(span.clone())),
            Token {
                kind:
                    TokenKind::DecimalIntLiteral
                    | TokenKind::BinaryIntLiteral
                    | TokenKind::OctalIntLiteral
                    | TokenKind::HexadecimalIntLiteral,
                span,
                text,
            } => {
                let base = match self.current().kind {
                    TokenKind::DecimalIntLiteral => 10,
                    TokenKind::BinaryIntLiteral => 2,
                    TokenKind::OctalIntLiteral => 8,
                    TokenKind::HexadecimalIntLiteral => 16,
                    _ => unreachable!(),
                };
                match usize::from_str_radix(text, base) {
                    Ok(val) => Ok((ASTKind::IntegerLiteral(val).into_ast(span.clone()))),
                    Err(_) => ErrorKind::SyntaxError
                        .new("Invalid {} Integer Literal")
                        .with_label(ErrorLabel::new(span.clone()))
                        .into(),
                }
            }
            Token {
                kind: TokenKind::EOF,
                span,
                ..
            } => ErrorKind::UnexpectedEOF
                .new("")
                .with_label(ErrorLabel::new(span.clone()))
                .into(),
            Token { kind, span, .. } => ErrorKind::UnexpectedToken
                .new(format!("got {kind:?}"))
                .with_label(ErrorLabel::new(span.clone()))
                .into(),
        }
    }
}
