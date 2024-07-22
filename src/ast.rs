use crate::span::Span;
use std::fmt::{Display, Formatter};

pub struct Program {
    pub filename: &'static str,
    pub stmts: Vec<Box<AST>>,
}

pub enum ASTKind {
    IntegerLiteral(usize),
    Identifier(String),
    // StringLiteral(String),
    // CharLiteral(char),
    Block(Vec<Box<AST>>),
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
            ASTKind::IntegerLiteral(val) => write!(f, "{}", val)?,
            ASTKind::Identifier(ident) => write!(f, "{}", ident)?,
            ASTKind::Block(stmts) => write!(f, "<block with {} statements>", stmts.len())?,
        }
        Ok(())
    }
}
