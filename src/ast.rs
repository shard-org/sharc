use crate::span::Span;
use std::fmt::{Display, Formatter};

pub struct Program {
    pub filename: &'static str,
    pub stmts: Vec<Box<AST>>,
}

#[derive(Debug)]
pub enum ASTKind {
    // Definitions
    LabelDefinition(Option<String>, Vec<LabelAttribute>),
    FunctionDefinition(String, Vec<LabelAttribute>, Box<AST>),

    // Keywords
    Return(Option<Box<AST>>),

    //
    // Expressions
    Identifier(String),

    IntegerLiteral(usize),
    StringLiteral(String),
    CharLiteral(char),

    Block(Vec<Box<AST>>),

    TypeAnnotation(Type, Option<Box<AST>>),

    // Calls
    Interrupt(usize),
    Syscall(String, Vec<AST>),
}

#[derive(Debug)]
pub enum LabelAttribute {
    Entry,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Size(usize),
    Heap { is_pointer: bool, contents: Vec<(Type, Option<usize>)> },
    Struct(String),
    Register { inner: Option<Box<Type>>, ident: usize, size: usize },
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
            ASTKind::IntegerLiteral(val) => write!(f, "(IntegerLiteral: {})", val)?,
            ASTKind::Identifier(ident) => write!(f, "(Identifier: {})", ident)?,
            ASTKind::Block(stmts) => write!(f, "(Block: {} statements)", stmts.len())?,
            ASTKind::StringLiteral(val) => write!(f, "(StringLiteral: {:?})", val)?,
            ASTKind::CharLiteral(val) => write!(f, "(CharLiteral: {:?})", val)?,
            ASTKind::TypeAnnotation(ty, Some(ast)) => {
                write!(f, "(TypeAnnotation: {:?} ({}))", ty, ast)?
            },
            ASTKind::TypeAnnotation(ty, None) => write!(f, "(TypeAnnotation: {:?})", ty)?,

            ASTKind::LabelDefinition(Some(name), attrs) => write!(
                f,
                "(LabelDefinition: {} ({}))",
                name,
                attrs.iter().fold(String::new(), |mut acc, attr| {
                    acc.push_str(&format!("{:?} ", attr));
                    acc
                })
            )?,
            ASTKind::LabelDefinition(_, attrs) => write!(
                f,
                "(LabelDefinition: ({}))",
                attrs.iter().fold(String::new(), |mut acc, attr| {
                    acc.push_str(&format!("{:?} ", attr));
                    acc
                })
            )?,

            ASTKind::FunctionDefinition(name, attrs, ast) => todo!(),

            ASTKind::Return(Some(val)) => write!(f, "(Return: {})", val)?,
            ASTKind::Return(_) => write!(f, "(Return)")?,

            ASTKind::Interrupt(val) => write!(f, "(Interrupt: {})", val)?,
            ASTKind::Syscall(name, args) => write!(
                f,
                "(Syscall: {} ({}))",
                name,
                args.iter().fold(String::new(), |mut acc, arg| {
                    acc.push_str(&format!("{} ", arg));
                    acc
                })
            )?,
        }
        Ok(())
    }
}
