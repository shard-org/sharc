use std::borrow::Cow;
use std::cmp::PartialEq;
use std::num::IntErrorKind;
use std::slice::Iter;
use std::str;

use crate::ast::{ASTKind, LabelAttribute, Program, Type, AST};
use crate::report::{Report, ReportKind, ReportSender, Result};
use crate::span::Span;
use crate::token::{Token, TokenKind};

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

    fn report(&self, report: Report) {
        self.sender.send(report.into());
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
                .title(format!("expected '{kind:?}' got '{actual:?}'"))
                .span(*span)
                .label(msg)
                .as_err(),
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
            TokenKind::EOF => Ok(()),
            _ => ReportKind::UnexpectedToken
                .title(format!("expected NewLine got '{kind:?}'"))
                .span(*span)
                .as_err(),
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
        let AST { kind: ASTKind::Block(stmts), .. } = self.parse_block(true) else {
            unreachable!("Can't happen nerds!")
        };
        Program { stmts, filename: self.filename }
    }

    fn parse_block(&mut self, global: bool) -> AST {
        let mut stmts: Vec<AST> = Vec::new();
        let until = if global { TokenKind::EOF } else { TokenKind::RBrace };
        let start = &self.current.span;

        while self.current.kind != until {
            match self.parse_statement() {
                Ok(val) => {
                    stmts.push(val);
                    self.consume_newline().map_err(|err| {
                        self.report(*err);
                        self.synchronize(until);
                    });
                },
                Err(report) => {
                    self.report(*report);
                    self.synchronize(until);
                },
            };
        }

        if !global {
            self.consume(until, "block not terminated");
        };

        let end = start.extend(&stmts.last().map_or_else(|| start, |ast| &ast.span));

        ASTKind::Block(stmts).into_ast(start.extend(&end))
    }

    fn parse_statement(&mut self) -> Result<AST> {
        match self.current.kind {
            // TokenKind::Colon      => self.parse_tag(),
            TokenKind::Star => self.parse_interrupt(),
            TokenKind::Identifier => self.parse_label(),
            TokenKind::KeywordRet => self.parse_return(),
            // HACK: this is temporary, this should parse assignments
            // Update: changed to another token because i needed % for modulo
            // test cases should still pass just change it
            TokenKind::Dollar => {
                self.advance();
                let ret = Ok(AST {
                    kind: ASTKind::TypeAnnotation(self.parse_type()?, None),
                    span: self.current.span.clone(),
                });
                self.advance();
                // assert_eq!(self.current.kind, TokenKind::NewLine);
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
            AST { kind: ASTKind::IntegerLiteral(val), .. } => {
                Ok(ASTKind::Interrupt(val).into_ast(self.current.span.clone()))
            },
            _ => ReportKind::SyntaxError
                .title("Expected Integer Literal")
                .span(self.current.span)
                .as_err(),
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
                .title("Expected Identifier")
                .span(self.current.span)
                .as_err();
        }

        let mut attributes = Vec::with_capacity(5); // Could be adjusted

        let label = self.current.text;

        self.advance();
        while !matches!(self.current.kind, TokenKind::Colon | TokenKind::LBrace) {
            match self.parse_label_attribute() {
                Some(attribute) => {
                    if attributes.contains(&attribute) {
                        return ReportKind::DuplicateAttribute
                            .title("Duplicate attribute encountered")
                            .span(self.current.span)
                            .as_err();
                    };

                    self.advance();
                    attributes.push(attribute);
                },
                None => {
                    self.advance();
                    return ReportKind::SyntaxError
                        .title("Invalid Label Attribute")
                        .span(self.current.span)
                        .as_err();
                },
            }
        }
        self.advance();

        Ok(ASTKind::LabelDefinition(Some(label.to_string()), attributes)
            .into_ast(self.current.span.clone()))
    }

    // TODO: update this to be pratt parsing.
    fn parse_expression(&mut self) -> Result<AST> {
        let start = &self.current.span;
        let lhs = self.parse_sum()?;

        match self.current.kind {
            TokenKind::Plus | TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
                // recurse on the expr
                let tok = self.current.kind;
                self.advance();
                let rhs = self.parse_expression()?;

                Ok(AST::new(
                    lhs.span.extend(&rhs.span),
                    ASTKind::BinaryExpr(Box::new(lhs), Box::new(rhs), tok),
                ))
            },
            _ => Ok(lhs),
        }
    }

    fn parse_sum(&mut self) -> Result<AST> {
        let start = &self.current.span;
        let lhs = self.parse_term()?;
        match self.current.kind {
            TokenKind::Plus => {
                let tok = self.current.kind;
                self.advance();
                let rhs = self.parse_product()?;
                Ok(AST::new(
                    start.extend(&self.current.span),
                    ASTKind::BinaryExpr(Box::new(lhs), Box::new(rhs), tok),
                ))
            },
            _ => Ok(lhs),
        }
    }

    fn parse_product(&mut self) -> Result<AST> {
        let start = &self.current.span;
        let lhs = self.parse_sum()?;

        match self.current.kind {
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
                let tok = self.current.kind;
                self.advance();
                let rhs = self.parse_term()?;
                Ok(AST::new(
                    start.extend(&self.current.span),
                    ASTKind::BinaryExpr(Box::new(lhs), Box::new(rhs), tok),
                ))
            },
            _ => Ok(lhs),
        }
    }

    fn parse_term(&mut self) -> Result<AST> {
        match self.current.kind {
            TokenKind::LParen => {
                let mut start = self.current.span.clone();
                self.advance();
                let mut inner = self.parse_expression()?;
                if self.current.kind != TokenKind::RParen {
                    let mut span = self.current.span.clone();
                    span.offset += 1;
                    return ReportKind::SyntaxError
                        .title("Expected closing parenthesies")
                        .span(span)
                        .as_err();
                }
                self.advance();

                inner.span = start.extend(&self.current.span);
                return Ok(inner);
            },
            _ => Ok(self.parse_atom()?),
        }
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
                        .title("Invalid Integer Literal")
                        .span(*span)
                        .as_err(),
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

            TokenKind::EOF => ReportKind::UnexpectedEOF.untitled().as_err(),

            kind => {
                self.advance();
                ReportKind::UnexpectedToken.title(format!("got {kind:?}")).span(*span).as_err()
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
            s if s.len() > 1 => {
                return ReportKind::InvalidEscapeSequence.untitled().span(*span).as_err()
            },
            s => s.as_bytes()[0],
        }) as char)
    }

    fn parse_type(&mut self) -> Result<Type> {
        match self.current.kind {
            // TokenKind::Semicolon => Ok(Type::Size(0)),
            TokenKind::DecimalIntLiteral => {
                // We know it lexed so this has to pass, so we can unwrap
                let Ok(size) = self.current.text.parse::<usize>() else {
                    return ReportKind::SyntaxError
                        .title("You cant have this many bytes, what are you even doing anyways?? stack overflow?")
                        .span(self.current.span)
                        .as_err();
                };

                if size == 0 {
                    return ReportKind::SyntaxError
                        .title("Size cannot be zero")
                        .span(self.current.span)
                        .as_err();
                };

                Ok(Type::Size(size))
            },
            TokenKind::Identifier => Ok(Type::Struct(self.current.text.to_string())),
            TokenKind::LBrace | TokenKind::LBracket => {
                let start_span = self.current.span.clone();
                let is_pointer = self.current.kind == TokenKind::LBracket;
                let start_kind = if is_pointer {TokenKind::LBracket} else {TokenKind::LBrace};
                let end_kind = if is_pointer {TokenKind::RBracket} else {TokenKind::RBrace};
                self.advance();

                //NOTE: idk if 5 is the right number. To be determined
                let mut vec: Vec<Type> = Vec::with_capacity(5);

                while self.current.kind != end_kind {
                    let t = self.parse_type().map_err(|e| {
                        match self.tokens[self.index - 1].kind {
                            TokenKind::Comma => {
                                let mut span = self.current.span.clone();
                                span.offset += 1;
                                ReportKind::SyntaxError
                                    .title("Unclosed heap, found comma")
                                    .span(span)
                                    .label(format!("Replace this , with a {}", if end_kind == TokenKind::RBrace {"}"} else {"]"}))
                                    .note("HINT: Commas are required between types")
                            },
                            a if matches!(a, TokenKind::LBrace | TokenKind::LBracket) => {
                                let opposite = match a {
                                    TokenKind::LBrace => TokenKind::RBracket,
                                    TokenKind::LBracket => TokenKind::RBrace,
                                    _ => unreachable!(),
                                };

                                if opposite != self.current.kind {return e};

                                ReportKind::SyntaxError
                                    .title("Incorrect heap nesting")
                                    .span(self.tokens[self.index - 1].span.clone())
                                    .label("This has no closing pair")
                                    .note("HINT: Inner heaps must terminate before outer ones")
                            }
                            _ => *e
                        }.into()
                    })?;

                    self.advance();

                    if matches!(t, Type::Register{..}) {
                        return ReportKind::SyntaxError
                            .title("Heaps cant contain register bindings")
                            .span(self.current.span)
                            .note("HINT: If they did, then memory would be discontiguous")
                            .as_err();
                    }

                    vec.push(t);

                    if self.current.kind != end_kind {
                        if self.current.kind == (if is_pointer {TokenKind::RBrace} else {TokenKind::RBracket}) {
                            return ReportKind::SyntaxError
                                .title("Mismatched heap brackets")
                                .span(self.current.span)
                                .note("HINT: Be more decisive next time. Is it a pointer or not?")
                                .as_err();
                        }

                        if self.current.kind == TokenKind::Comma {
                            self.advance();
                            continue;
                        }

                        if self.current.kind == TokenKind::NewLine {
                            let mut span = self.current.span.clone();
                            span.offset += 1;
                            return ReportKind::UnexpectedToken
                                .title("Unclosed heap, found newline")
                                .span(span)
                                .note("HINT: Commas are required between types")
                                .as_err();
                        }

                        if self.current.kind != TokenKind::NewLine {
                            let mut span = self.tokens[self.index - 1].span.clone();
                            span.offset += 1;

                            return ReportKind::SyntaxError
                                .title("Expected comma between types")
                                .span(span)
                                .label("Add one here")
                                .note("HINT: Commas are required between types")
                                .as_err();
                        }
                    }
                }

                if !is_pointer && vec.len() == 0 {
                    return ReportKind::SyntaxError
                        .title("Zero-sized heaps are disallowed")
                        .span(start_span.extend(&self.current.span))
                        .note("HINT: Did you mean to do a void pointer: []?")
                        .as_err();
                }

                Ok(Type::Heap { is_pointer, contents: vec })
            },
            TokenKind::NewLine => {
                let mut span = self.current.span.clone();
                span.offset += 1;
                ReportKind::UnexpectedToken
                    .title("Unexpected newline")
                    .span(span)
                    .as_err()
            },
            _ => ReportKind::UnexpectedToken
                .title(format!("Unexpected token: {:?}", self.current.kind))
                .span(self.current.span)
                .note("HINT: We expect literally any type... and you still messed it up")
                .as_err(),
        }
        // After the base type, optionally parse a register or an array, which
        // are mutrually exclusive
        .and_then(|t| {
            match self.peek(1).kind {
                TokenKind::Semicolon => {
                    self.advance();
                    self.advance();
                    let mut span = self.current.span.clone();

                    match t {
                        Type::Heap { is_pointer: true, .. } => Ok(()),
                        Type::Size(a) if a <= /*TODO: max register size here */ 8 => Ok(()),
                        _ => ReportKind::SyntaxError
                            .title("Registers can only be bound to pointer to heaps or sizes under the register's max")
                            .span(self.tokens[self.index - 1].span.extend(&span))
                            .as_err(),
                    }?;

                    match self.current.kind {
                        TokenKind::Identifier => Ok(()),
                        TokenKind::DecimalIntLiteral => ReportKind::SyntaxError
                            .title("Expected register starting with r")
                            .note(format!("HINT: You forgot the r prefix. Do: r{}", self.current.text))
                            .span(self.current.span)
                            .as_err(),
                        _ => ReportKind::UnexpectedToken
                            .title(format!("Expected register, got {}", self.current.text))
                            .note("HINT: Registers follow the format r<reg>. e.g r8 r32")
                            .span(self.current.span)
                            .as_err(),
                    }?;

                    if !self.current.text.starts_with('r') {
                        return ReportKind::SyntaxError
                            .title("Register identifier format is incorrect!")
                            .span(self.current.span)
                            .note("HINT: Registers follow the format r<reg>. e.g r8 r32")
                            .as_err();
                        };

                    match self.current.text[1..].parse::<usize>() {
                        Err(e) => match e.kind() {
                            IntErrorKind::Empty => ReportKind::SyntaxError
                                .title("Expected register identifier after r prefix")
                                .span(self.current.span)
                                .note("HINT: Registers follow the format r<ident>. e.g r8 r32")
                                .as_err(),
                            IntErrorKind::InvalidDigit => {
                                ReportKind::SyntaxError
                                    .title("Register number contains an invalid digit")
                                    .span(self.current.span)
                                    .note("HINT: Registers follow the format r<ident>. e.g r8 r32")
                            }
                            .as_err(),
                            // Here only positive overflow can be omitted by parse::<usize>()
                            // It also doesnt omit Zero because usize can store 0.
                            _ => ReportKind::SyntaxError
                                .title("Register identifier intager overflows")
                                .span(self.current.span)
                                .note("HINT: You dont have this many registers. Trust me")
                                .as_err(),
                        }
                        Ok(i) => {
                            if self.peek(1).kind == TokenKind::Colon {
                                return ReportKind::SyntaxError
                                    .title("Register binding cannot be followed by an array!")
                                    .span(self.tokens[self.index + 1].span.extend(&self.tokens[self.index + 2].span))
                                    .as_err();
                            }
                            Ok(Type::Register { inner: if t == Type::Size(0) {None} else {Some(Box::new(t))}, ident: i })
                        }
                    }
                },
                TokenKind::Colon => {
                    self.advance();
                    self.advance();

                    let elems: Option<usize> = if self.current.kind == TokenKind::DecimalIntLiteral {
                        let elem_size = self.current.text.parse::<usize>().unwrap();
                        if elem_size == 0 {
                            return ReportKind::SyntaxError
                                .title("Array size cannot be zero.")
                                .note(format!("HINT: Did you mean [{t}:]"))
                                .span(self.current.span)
                                .as_err();
                        }
                        Some(elem_size)
                    } else {
                        None
                    };

                    if self.peek(1).kind == TokenKind::Semicolon {
                        return ReportKind::SyntaxError
                            .title("Array cannot be followed by a register binding!")
                            .span(self.tokens[self.index + 1].span.extend(&self.tokens[self.index + 2].span))
                            .as_err();
                    }

                    Ok(Type::Array {inner: Box::new(t), elems})
                },
                _ => Ok(t)
            }
        })
    }
}
