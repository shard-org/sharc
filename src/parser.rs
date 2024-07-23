use crate::ast::{ASTKind, Program, AST};
use crate::error::{Error, ErrorKind, ErrorLabel, ErrorSender, Result, Unbox};
use crate::token::{Token, TokenKind};
use std::cmp::PartialEq;
use std::slice::Iter;

pub struct Parser<'t, 'contents> {
    filename: &'static str,
    tokens: &'t [Token<'contents>],
    current: &'t Token<'contents>,
    index: usize,
    sender: ErrorSender,
}

impl<'t, 'contents> Parser<'t, 'contents> {
    pub fn new(
        filename: &'static str,
        tokens: &'t [Token<'contents>],
        sender: ErrorSender,
    ) -> Self {
        Self {
            filename,
            tokens,
            current: &tokens[0],
            index: 0,
            sender,
        }
    }

    fn error(&mut self, error: Box<Error>) {
        self.sender.send(error)
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
            } => ErrorKind::UnexpectedToken
                .new(format!("expected '{kind:?}' got '{actual:?}'"))
                .with_label(ErrorLabel::new(span.clone()).with_text(msg))
                .into(),
        }
    }

    fn consume_newline(&mut self) -> Result<()> {
        match self.current {
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
                        self.error(err);
                        self.synchronize(until);
                    });
                }
                Err(error) => {
                    self.error(error);
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
            } => {
                self.advance();
                ErrorKind::UnexpectedEOF
                    .new("")
                    .with_label(ErrorLabel::new(span.clone()))
                    .into()
            }
            Token { kind, span, .. } => {
                self.advance();
                ErrorKind::UnexpectedToken
                    .new(format!("got {kind:?}"))
                    .with_label(ErrorLabel::new(span.clone()))
                    .into()
            }
        }
    }
}
