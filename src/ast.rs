use crate::span::Span;
use std::fmt::{Display, Formatter};

pub struct Program {
    pub filename: &'static str,
    pub stmts: Vec<Box<AST>>,
}

pub enum ASTKind {
    Tag(Tag),

    IntegerLiteral(usize),
    Identifier(String),

    StringLiteral(String),
    CharLiteral(char),
    Block(Vec<Box<AST>>),

    LabelDefinition(Option<String>, Vec<LabelAttribute>),
    FunctionDefinition(String, Vec<FunctionAttribute>, Box<AST>),

    TypeAnnotation(Type, Box<AST>),
}

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
            ASTKind::LabelDefinition(name, attrs) => {
                write!(f, "(LabelDefinition: {} with {} attributes)", name.unwrap(), attrs.len())?
            },
            ASTKind::FunctionDefinition(name, attrs, ast) => {
                write!(f, "(FunctionDefinition: {} with {} attributes)", name, attrs.len())?
            },
        }
        Ok(())
    }
}
