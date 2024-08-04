use std::cmp::PartialEq;
use std::num::IntErrorKind;
use std::slice::Iter;
use std::str;

use crate::ast::{ASTKind, LabelAttribute, Program, Type, AST};
use crate::report::{Report, ReportKind, ReportLabel, ReportSender, Result, Unbox};
use crate::token::{Token, TokenKind};

pub struct Parser<'t, 'contents> {
    filename: &'static str,
    tokens:   &'t [Token<'contents>],
    current:  &'t Token<'contents>,
    index:    usize,
    sender:   ReportSender,
}

impl<'t, 'contents> Parser<'t, 'contents> {
    pub fn new(
        filename: &'static str, tokens: &'t [Token<'contents>], sender: ReportSender,
    ) -> Self {
        let mut index = 0;
        while tokens.get(index).is_some_and(|t| t.kind == TokenKind::NewLine) {
            index += 1;
        }

        Self {
            filename,
            sender,
            index: index.saturating_sub(1),
            tokens: &tokens[index..],
            current: &tokens[index],
        }
    }

    fn report(&self, report: Box<Report>) {
        self.sender.send(report);
    }

    fn advance(&mut self) {
        self.index += 1;
        self.current =
            &self.tokens.get(self.index).expect("Failed to advance token: out of bounds");
    }

    fn peek(&self, offset: usize) -> &Token {
        &self.tokens.get(self.index + offset).expect("Failed to peek token: out of bounds")
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
            TokenKind::NewLine => {
                loop {
                    if self.current.kind == TokenKind::NewLine {
                        self.advance();
                        continue;
                    }
                    break;
                }
                Ok(())
            },
            TokenKind::EOF => {
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
        let AST { kind: ASTKind::Block(stmts), .. } = self.parse_block(true)
        else {
            unreachable!("Can't happen nerds!")
        };
        Program { stmts, filename: self.filename }
    }

    fn parse_block(&mut self, global: bool) -> AST {
        let mut stmts: Vec<AST> = Vec::new();
        let until = if global { TokenKind::EOF } else { TokenKind::RBrace };
        let start = self.current.span.clone();

        while self.current.kind != until {
            match self.parse_statement() {
                Ok(val) => {
                    stmts.push(val);
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

        let end = start.extend(stmts.last().map_or_else(|| &start, |ast| &ast.span));

        ASTKind::Block(stmts).into_ast(start.extend(&end))
    }

    fn parse_statement(&mut self) -> Result<AST> {
        match self.current.kind {
            // TokenKind::Colon      => self.parse_tag(),
            TokenKind::Star => self.parse_interrupt(),
            TokenKind::Identifier => self.parse_label(),
            TokenKind::Ret => self.parse_return(),
            // HACK: this is temporary, this should parse assignments
            TokenKind::Percent => {
                self.advance();
                let ret = Ok(AST {
                    kind: ASTKind::TypeAnnotation(self.parse_type()?, None),
                    span: self.current.span.clone(),
                });
                self.advance();
                return ret;
            },
            _ => self.parse_expression(),
        }
    }

    fn parse_return(&mut self) -> Result<AST> {
        if self.peek(1).kind == TokenKind::NewLine {
            self.advance();
            return Ok(ASTKind::Return(None).into_ast(self.current.span.clone()));
        }

        self.advance();
        let expr = self.parse_expression()?;
        Ok(ASTKind::Return(Some(expr.into())).into_ast(self.current.span.clone()))
    }

    fn parse_interrupt(&mut self) -> Result<AST> {
        self.advance();
        // syscall
        if self.current.kind == TokenKind::Identifier {
            let call_name = self.current.text.to_string();
            let mut args = Vec::new();

            self.advance();
            while self.current.kind != TokenKind::NewLine {
                args.push(self.parse_expression()?);

                if self.current.kind != TokenKind::Comma {
                    break;
                }
                self.advance();
            }

            return Ok(ASTKind::Syscall(call_name, args).into_ast(self.current.span.clone()));
        }

        match self.parse_expression()? {
            AST { kind: ASTKind::IntegerLiteral(val), .. } =>
                Ok(ASTKind::Interrupt(val).into_ast(self.current.span.clone())),
            _ => ReportKind::SyntaxError
                .new("Expected Integer Literal")
                .with_label(ReportLabel::new(self.current.span.clone()))
                .into(),
        }
    }

    fn parse_label_attribute(&self) -> Option<LabelAttribute> {
        match self.current.text {
            "entry" => Some(LabelAttribute::Entry),
            _ => None,
        }
    }

    fn parse_label(&mut self) -> Result<AST> {
        if self.current.kind != TokenKind::Identifier {
            return ReportKind::UnexpectedToken
                .new("Expected Identifier")
                .with_label(ReportLabel::new(self.current.span.clone()))
                .into();
        }

        let mut attributes = Vec::with_capacity(5); // Could be adjusted

        let label = self.current.text;

        self.advance();
        while !matches!(self.current.kind, TokenKind::Colon | TokenKind::LBrace) {
            match self.parse_label_attribute() {
                Some(attribute) => {
                    if attributes.contains(&attribute) {
                        return ReportKind::DuplicateAttribute
                            .new("Duplicate attribute encountered")
                            .with_label(ReportLabel::new(self.current.span.clone()))
                            .into();
                    };

                    self.advance();
                    attributes.push(attribute);
                },
                None => {
                    self.advance();
                    return ReportKind::SyntaxError
                        .new("Invalid Label Attribute")
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .into();
                },
            }
        }
        self.advance();

        Ok(ASTKind::LabelDefinition(Some(label.to_string()), attributes)
            .into_ast(self.current.span.clone()))
    }

    fn parse_expression(&mut self) -> Result<AST> {
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> Result<AST> {
        let Token { kind, span, text } = self.current;

        match &kind {
            TokenKind::Identifier => {
                self.advance();
                Ok(ASTKind::Identifier((*text).to_string()).into_ast(span.clone()))
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
                    Ok(val) => Ok(ASTKind::IntegerLiteral(val).into_ast(span.clone())),
                    Err(_) => ReportKind::SyntaxError
                        .new("Invalid Integer Literal")
                        .with_label(ReportLabel::new(span.clone()))
                        .into(),
                }
            },

            TokenKind::StringLiteral => {
                // FIXME: this prob isnt the best way to do this :/
                let text_bytes = text.as_bytes();
                let text_len = text_bytes.len();

                let mut text = String::with_capacity(text_len);
                for (i, window) in text_bytes.windows(2).enumerate() {
                    match window[0] as char {
                        '\\' => {
                            text.push(Self::parse_escape(str::from_utf8(window).unwrap(), span)?);
                        },
                        _ if i + 2 >= text_len => text.push_str(str::from_utf8(window).unwrap()),
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
            "\\A" => 1,
            "\\B" => 2,
            "\\C" => 3,
            "\\D" => 4,
            "\\E" => 5,
            "\\F" => 6,
            "\\G" | "\\a" => 7,
            "\\H" | "\\b" => 8,
            "\\I" | "\\t" => 9,
            "\\J" | "\\n" => 10,
            "\\K" | "\\v" => 11,
            "\\L" | "\\f" => 12,
            "\\M" | "\\r" => 13,
            "\\N" => 14,
            "\\O" => 15,
            "\\P" => 16,
            "\\Q" => 17,
            "\\R" => 18,
            "\\S" => 19,
            "\\T" => 20,
            "\\U" => 21,
            "\\V" => 22,
            "\\W" => 23,
            "\\X" => 24,
            "\\Y" => 25,
            "\\Z" => 26,
            "\\[" | "\\e" => 27,
            "\\/" => 28,
            "\\]" => 29,
            "\\^" => 30,
            "\\_" => 31,
            "\\?" => 32,
            "\\" => b'\\',
            "\\`" => b'`',
            s if s.len() > 1 =>
                return ReportKind::InvalidEscapeSequence
                    .new("")
                    .with_label(ReportLabel::new(span.clone()))
                    .into(),
            s => s.as_bytes()[0],
        }) as char)
    }

    fn parse_register_binding(&mut self, inner: Option<Type>) -> Result<Type> {
        // register binding
        self.advance();

        match self.current.kind {
            TokenKind::Identifier => Ok(()),
            TokenKind::DecimalIntLiteral => ReportKind::SyntaxError
                .new("Expected register starting with r")
                .with_note(format!("HINT: You forgot the r prefix. Do: r{}", self.current.text))
                .with_label(ReportLabel::new(self.current.span.clone()))
                .into(),
            _ => ReportKind::UnexpectedToken
                .new(format!("Expected register, got {}", self.current.text))
                .with_note("HINT: Registers follow the format r<ident>. e.g r8 r32")
                .with_label(ReportLabel::new(self.current.span.clone()))
                .into(),
        }?;

        if !self.current.text.starts_with('r') {
            return ReportKind::SyntaxError
                .new("Register identifier format is incorrect!")
                .with_label(ReportLabel::new(self.current.span.clone()))
                .with_note("HINT: Registers follow the format r<ident>. e.g r8 r32")
                .into();
        }

        match self.current.text.strip_prefix('r').unwrap().parse::<usize>() {
            Err(e) => match e.kind() {
                IntErrorKind::Empty => ReportKind::SyntaxError
                    .new("Expected register identifier after r prefix")
                    .with_label(ReportLabel::new(self.current.span.clone()))
                    .with_note("HINT: Registers follow the format r<ident>. e.g r8 r32"),
                IntErrorKind::InvalidDigit => {
                    let mut span = self.current.span.clone();
                    span.start_index += 1;
                    span.end_index += self.current.text.len();

                    ReportKind::SyntaxError
                        .new("Register number contains an invalid digit")
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .with_note("HINT: Registers follow the format r<ident>. e.g r8 r32")
                },
                _ => ReportKind::SyntaxError
                    .new("Register identifier intager overflows")
                    .with_label(ReportLabel::new(self.current.span.clone()))
                    .with_note("HINT: You dont have this many registers. Trust me"),
            }
            .into(),
            Ok(i) => Ok(Type::Register { inner: inner.map(|t| Box::new(t)), ident: i }),
        }
    }

    fn parse_type(&mut self) -> Result<Type> {
        match self.current.kind {
            TokenKind::Semicolon => self.parse_register_binding(None),
            TokenKind::DecimalIntLiteral => {
                // We know it lexed so this has to pass, so we can unwrap
                let size = self.current.text.parse::<usize>().unwrap();
                if size == 0 {
                    return ReportKind::SyntaxError
                        .new("Size cannot be zero")
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .into();
                };

                Ok(Type::Size(size))
            },
            TokenKind::Identifier => Ok(Type::Struct(self.current.text.to_string())),
            TokenKind::LBracket => {
                self.advance();
                let t = self.parse_type()?;
                let mut n = None;

                self.advance();
                if self.current.kind == TokenKind::Colon {
                    self.advance();
                    match self.current.kind {
                        TokenKind::DecimalIntLiteral => {
                            n = Some(self.current.text.parse::<usize>().unwrap());
                            if n == Some(0) {
                                return ReportKind::SyntaxError
                                    .new("Array size cannot be zero.")
                                    .with_note(format!("HINT: Did you mean [{t}:]"))
                                    .with_label(ReportLabel::new(self.current.span.clone()))
                                    .into();
                            }
                            self.advance();
                        },
                        TokenKind::RBracket => {},
                        _ => {
                            self.advance();
                            return ReportKind::UnexpectedToken
                                .new(format!("got {:?}", self.current.kind))
                                .with_label(ReportLabel::new(self.current.span.clone()))
                                .into();
                        },
                    }
                }
                // We should fail earlier but we wait to gather the element size
                // n before logging for clearer error logging
                if let Type::Register { inner, ident } = t {
                    let mut inner_str = String::new();
                    let mut n_str = String::new();
                    if inner.is_some() {
                        inner_str = format!("{}", inner.unwrap());
                    }

                    if n.is_some() && n.unwrap() != 0 {
                        n_str = format!("{}", n.unwrap());
                    }

                    self.advance();
                    return ReportKind::SyntaxError
                        .new("Heap types cannot contain register bindings")
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .with_note(format!("HINT: Did you want to bind the pointer to the register? [{inner_str}:{n_str}];r{ident}"))
                        .into();
                }

                if self.current.kind != TokenKind::RBracket {
                    self.advance();
                    return ReportKind::SyntaxError
                        .new(format!("Expected closing bracket. Got {:?}", self.current.text))
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .into();
                };

                Ok(Type::Heap { is_pointer: true, contents: vec![(t, n)] })
            },
            TokenKind::LBrace => {
                self.advance();
                if self.current.kind == TokenKind::RBrace {
                    let mut span = self.current.span.clone();
                    span.start_index -= 1;

                    return ReportKind::SyntaxError
                        .new("Empty heaps are disallowed")
                        .with_label(ReportLabel::new(span))
                        .with_note("HINT: Did you want to create a void pointer: []")
                        .into();
                }
                //NOTE: idk if 5 is the right number. To be determined
                let mut vec: Vec<(Type, Option<usize>)> = Vec::with_capacity(5);
                loop {
                    let start = self.current.span.clone();
                    let t = self.parse_type()?;
                    let mut n = None;

                    let end = self.current.span.clone();
                    let span = start.extend(&end);
                    self.advance();

                    if self.current.kind == TokenKind::Colon {
                        self.advance();
                        match self.current.kind {
                            TokenKind::DecimalIntLiteral => {
                                n = Some(self.current.text.parse::<usize>().unwrap());
                                if n == Some(0) {
                                    return ReportKind::SyntaxError
                                        .new("Array size cannot be zero.")
                                        .with_note(format!("HINT: Did you mean {t}:"))
                                        .with_label(ReportLabel::new(self.current.span.clone()))
                                        .into();
                                }
                                self.advance();
                            },
                            TokenKind::Comma | TokenKind::RBrace => {},
                            _ => {
                                self.advance();
                                return ReportKind::UnexpectedToken
                                    .new(format!(
                                        "Expected either `,` `}}` or a intager, got {:?}",
                                        self.current.kind
                                    ))
                                    .with_label(ReportLabel::new(self.current.span.clone()))
                                    .into();
                            },
                        }
                    }

                    if self.current.kind != TokenKind::Comma {
                        if self.current.kind == TokenKind::RBrace {
                            vec.push((t, n));
                            break;
                        }
                        return ReportKind::SyntaxError
                            .new("Expected comma to separate heap types")
                            .with_label(ReportLabel::new(self.current.span.clone()))
                            .into();
                    }
                    self.advance();
                    if let Type::Register { ident, .. } = t {
                        return ReportKind::SyntaxError
                            .new("Heap types cannot contain register bindings")
                            .with_label(ReportLabel::new(span))
                            .with_note(format!("HINT: Did you want to bind the pointer to the register? {};r{ident}", Type::Heap { is_pointer: false, contents: vec }))
                            .into();
                    }

                    vec.push((t, n));
                }

                if self.current.kind != TokenKind::RBrace {
                    return ReportKind::SyntaxError
                        .new(format!("Expected closing brace. Got {:?}", self.current.text))
                        .with_label(ReportLabel::new(self.current.span.clone()))
                        .into();
                };
                Ok(Type::Heap { is_pointer: false, contents: vec })
            },
            _ => ReportKind::UnexpectedToken
                .new(format!("Unexpected token: {}", self.current.text))
                .with_label(ReportLabel::new(self.current.span.clone()))
                .with_note("Acceptable tokens are: `[` `{` or an intager")
                .into(),
        }.and_then(|t| {
            if self.peek(1).kind == TokenKind::Semicolon {
                self.advance();
                self.parse_register_binding(Some(t))
            } else {
                Ok(t)
            }
        })
    }
}
