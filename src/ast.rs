use std::fmt::{Display, Formatter};

use crate::span::Span;
use crate::token::TokenKind;

pub struct Program {
    pub filename: &'static str,
    pub stmts: Vec<AST>,
}

#[derive(Debug)]
pub enum ASTKind {
    // Definitions
    LabelDefinition(Option<String>, Vec<LabelAttribute>),
    FunctionDefinition(String, Vec<LabelAttribute>, Box<AST>),

    // Keywords
    Return(Option<Box<AST>>),

    // Expressions
    BinaryExpr(Box<AST>, Box<AST>, TokenKind),
    Identifier(String),

    IntegerLiteral(usize),
    StringLiteral(String),
    CharLiteral(char),

    Block(Vec<AST>),

    TypeAnnotation(Type, Option<Box<AST>>),

    // Calls
    Interrupt(usize),
    Syscall(String, Vec<AST>),
}

#[derive(Debug, PartialEq)]
pub enum LabelAttribute {
    Entry,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Size(usize),
    Heap { is_pointer: bool, contents: Vec<Type> },
    // NOTE: a size of 0 represents an array of undetermined length e.g [1:]
    Array { inner: Box<Type>, elems: Option<usize> },
    Struct(String),
    Register { inner: Option<Box<Type>>, ident: usize },
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Size(s) => write!(f, "{s}")?,
            Self::Array { inner, elems } => {
                write!(f, "{inner}")?;
                if elems.is_none() {
                    return Ok(());
                };

                write!(f, ":")?;
                if elems.unwrap() != 0 {
                    write!(f, "{}", elems.unwrap())?;
                }
            },
            Self::Heap { is_pointer, contents } => {
                write!(f, "{}", if *is_pointer { "[" } else { "{" })?;
                for (i, t) in contents.iter().enumerate() {
                    write!(f, "{t}")?;

                    if i != contents.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "{}", if *is_pointer { "]" } else { "}" })?;
            },
            Self::Register { inner: t, ident } => {
                if t.is_some() {
                    write!(f, "{}", t.as_ref().unwrap())?;
                }
                write!(f, ";r{ident}")?;
            },
            Self::Struct(ident) => write!(f, "{ident}")?,
        };
        Ok(())
    }
}

impl ASTKind {
    pub fn into_ast(self, span: Span) -> AST {
        AST { span, kind: self }
    }
}

#[derive(Debug)]
pub struct AST {
    pub span: Span,
    pub kind: ASTKind,
}

impl AST {
    pub fn new(span: Span, kind: ASTKind) -> Self {
        Self { span, kind }
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ASTKind::IntegerLiteral(val) => write!(f, "(IntegerLiteral: {val})")?,
            ASTKind::BinaryExpr(rhs, lhs, op) => write!(f, "(BinaryExpr {op:?} {rhs} {lhs})")?,
            ASTKind::Identifier(ident) => write!(f, "(Identifier: {ident})")?,
            ASTKind::Block(stmts) => write!(f, "(Block: {} statements)", stmts.len())?,
            ASTKind::StringLiteral(val) => write!(f, "(StringLiteral: {val:?})")?,
            ASTKind::CharLiteral(val) => write!(f, "(CharLiteral: {val:?})")?,
            ASTKind::TypeAnnotation(ty, Some(ast)) => {
                write!(f, "(TypeAnnotation: {ty:?} ({ast}))")?;
            },
            ASTKind::TypeAnnotation(ty, None) => write!(f, "(TypeAnnotation: {ty:?})")?,

            ASTKind::LabelDefinition(Some(name), attrs) => write!(
                f,
                "(LabelDefinition: {name} ({}))",
                attrs.iter().fold(String::new(), |mut acc, attr| {
                    acc.push_str(&format!("{attr:?} "));
                    acc
                })
            )?,
            ASTKind::LabelDefinition(_, attrs) => write!(
                f,
                "(LabelDefinition: ({}))",
                attrs.iter().fold(String::new(), |mut acc, attr| {
                    acc.push_str(&format!("{attr:?} "));
                    acc
                })
            )?,

            ASTKind::FunctionDefinition(name, attrs, ast) => todo!(),

            ASTKind::Return(Some(val)) => write!(f, "(Return: {val})")?,
            ASTKind::Return(_) => write!(f, "(Return)")?,

            ASTKind::Interrupt(val) => write!(f, "(Interrupt: {val})")?,
            ASTKind::Syscall(name, args) => write!(
                f,
                "(Syscall: {name} ({}))",
                args.iter().fold(String::new(), |mut acc, arg| {
                    acc.push_str(&format!("{arg} "));
                    acc
                })
            )?,
        }
        Ok(())
    }
}
