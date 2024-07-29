use std::collections::{HashMap, HashSet};
use crate::report::{Report, ReportKind, ReportLabel, ReportSender, Result, Unbox};
use crate::scanner::Scanner;
use crate::span::Span;
use crate::token::{Token, TokenKind};
use crate::ast::{Type};
// use crate::span::Span;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tag {
    Name(String),
    Arch(Vec<String>),
    Macro(String),

    SyscallConv(Vec<Type>, Option<Box<Type>>), // expect registers
    // Syscall(Vec<Box<AST>>, String), // expect TypeAnnotation
}

pub struct PreProcessor<'contents> {
    filename:   &'static str,
    tokens:     std::iter::Peekable<std::vec::IntoIter<Token<'contents>>>,
    sender:     ReportSender,

    output:     Vec<Token<'contents>>,
    tags:       HashSet<Tag>,

    macro_defs: HashMap<String, Vec<Token<'contents>>>,
}

impl<'contents> PreProcessor<'contents> {
    pub fn new(filename: &'static str, tokens: Vec<Token<'contents>>, sender: ReportSender) -> Self {
        let tokens_len = tokens.len();
        let mut tokens = tokens.into_iter().peekable();

        Self {
            filename, tokens, sender,
            output: Vec::with_capacity(tokens_len),
            tags: HashSet::new(),
            macro_defs: HashMap::new(),
        }
    }

    fn report(&mut self, report: Box<Report>) {
        self.sender.send(report)
    }

    fn next(&mut self) -> Result<Token<'contents>> {
        self.tokens.next().ok_or_else(|| {
            let file = Scanner::get_file(self.filename);

            let line_number = file.lines().count();

            ReportKind::UnexpectedEOF
                .new("Unexpected end of file")
                .with_label(ReportLabel::new(
                    Span::new(self.filename, line_number, file.len()-1, file.len())
                )).into()
        })
    }

    fn advance(&mut self) -> Result<()> {
        let token = self.next()?;
        self.output.push(token);
        Ok(())
    }

    fn peek(&mut self) -> &Token<'contents> {
        self.tokens.peek().unwrap()
    }

    pub fn process(mut self) -> (Vec<Token<'contents>>, HashSet<Tag>) {
        match self.index_all().and_then(|_| self.macro_expand(1)) {
            Ok(()) => (self.output, self.tags),
            Err(err) => {
                self.report(err);
                (Vec::new(), HashSet::new())
            }
        }
    }

    fn macro_expand(&mut self, pass: usize) -> Result<()> {
        if pass > 10 {
            return ReportKind::ExceededRecursionLimit
                .new("reached over 10 macro expansion passes")
                .into();
        }

        let mut index = 0;
        let mut is_done = true;

        while let token = &self.output[index] {
            index += 1;
            match token.kind {
                TokenKind::Pound
                    if self.output[index].kind == TokenKind::Identifier => {
                        let ident = self.output[index].text;
                        match self.macro_defs.get(ident) {
                            Some(tokens) => {
                                is_done = false;
                                self.output.splice(index-1..index+1, tokens.iter().cloned());
                            },
                            None => return ReportKind::UndefinedMacro
                                .new(format!("Undefined macro: {ident}"))
                                .with_label(ReportLabel::new(token.span.clone()))
                                .into(),
                        }
                    },
                TokenKind::EOF => break,
                _ => {},
            }
        }

        if !is_done {
            self.macro_expand(pass + 1)?;
        }

        Ok(())
    }

    fn index_all(&mut self) -> Result<()> {
        let mut is_line_start = true;

        while let token = self.next()? {
            match token.kind {
                TokenKind::EOF => {
                    self.output.push(token);
                    break
                },
                TokenKind::Colon if is_line_start => {
                    self.index_tag()?;
                    is_line_start = true;
                },
                TokenKind::NewLine 
                    if self.peek().kind == TokenKind::Colon => {
                        self.next()?;
                        self.index_tag()?;
                        is_line_start = true;
                    },
                _ => {
                    is_line_start = false;
                    self.output.push(token)
                },
            }
        }
        Ok(())
    }

    fn index_tag(&mut self) -> Result<()> {
        let token = self.next()?;
        match token.text.to_lowercase().as_str() {
            "name" => {
                let token = self.next()?;
                if token.kind != TokenKind::StringLiteral {
                    return ReportKind::SyntaxError
                        .new(format!("Expected StringLiteral; got {:?}", token.kind))
                        .with_label(ReportLabel::new(token.span))
                        .into();
                }

                self.tags.insert(Tag::Name(token.text.to_string()));
                self.macro_defs.insert(String::from("NAME"), vec![token]);
                Ok(())
            },

            "arch" => {
                let mut arch = Vec::with_capacity(3);
                while let token = self.next()? {
                    match token.kind {
                        TokenKind::NewLine => break,
                        TokenKind::Identifier => arch.push(token),
                        _ => return ReportKind::SyntaxError
                            .new("Expected identifier")
                            .with_label(ReportLabel::new(token.span))
                            .into(),
                    }
                }

                self.tags.insert(Tag::Arch(arch.clone().into_iter().map(|token| token.text.to_string()).collect()));

                arch.iter_mut().for_each(|token| token.kind = TokenKind::StringLiteral);
                self.macro_defs.insert(String::from("ARCH"), arch);
                Ok(())
            },

            "macro" => {
                let token = self.next()?;
                if token.kind != TokenKind::Identifier {
                    return ReportKind::SyntaxError
                        .new("Expected identifier")
                        .with_label(ReportLabel::new(token.span))
                        .into();
                }

                let name = token.text.to_string();
                let mut tokens = Vec::new();
                while let token = self.next()? {
                    match token.kind {
                        TokenKind::NewLine => break,
                        _ => tokens.push(token),
                    }
                }

                self.macro_defs.insert(name.to_string(), tokens);
                self.tags.insert(Tag::Macro(name));
                Ok(())
            },

            text => ReportKind::InvalidTag
                .new(format!("{text:?}"))
                .with_label(ReportLabel::new(token.span.clone()))
                .into(),
        }
    }
}

