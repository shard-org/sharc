use crate::span::Span;
use std::fmt::{Display, Formatter};

pub struct Program {
    pub filename: &'static str,
    pub stmts: Vec<Box<AST>>,
}

pub enum ASTKind {
    Tag(Tag),

    Identifier(String),

    IntegerLiteral(usize),
    StringLiteral(String),
    CharLiteral(char),

    Block(Vec<Box<AST>>),

    LabelDefinition(Option<String>, Vec<LabelAttribute>),
    FunctionDefinition(String, Vec<LabelAttribute>, Box<AST>),

    Interrupt(usize),
    Syscall(),

    TypeAnnotation(Type, Box<AST>),

    Return(Option<Box<AST>>),
}

#[derive(Debug)]
pub enum LabelAttribute {
    Entry,
}

#[derive(Debug)]
pub enum Type {
    Size(usize),
    Heap { is_pointer: bool, contents: Vec<(Type, Option<usize>)> },
    Struct(String),
}

#[derive(Debug)]
pub enum Tag {
    Name(String),
    Arch(Vec<String>),
}

impl ASTKind {
    pub fn into_ast(self, span: Span) -> AST {
        AST { span, kind: self }
    }
}

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
            ASTKind::TypeAnnotation(ty, ast) => write!(f, "(TypeAnnotation: {:?}: {})", ty, ast)?,
            ASTKind::Tag(tag) => write!(f, "(Tag: {:?})", tag)?,

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

            ASTKind::FunctionDefinition(name, attrs, ast) => {
                write!(f, "(FunctionDefinition: {} with {} attributes)", name, attrs.len())?
            },

            ASTKind::Return(Some(val)) => write!(f, "(Return: {})", val)?,
            ASTKind::Return(_) => write!(f, "(Return)")?,

            ASTKind::Interrupt(val) => write!(f, "(Interrupt: {})", val)?,
            ASTKind::Syscall() => todo!(),
        }
        Ok(())
    }
}
