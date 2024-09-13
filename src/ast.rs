use std::fmt::{Display, Formatter};

use crate::report::{Report, ReportKind};
use crate::span::Span;
use crate::token::TokenKind;

pub struct Program {
    pub filename: &'static str,
    pub stmts:    Vec<AST>,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    // PREFIX
    MutatePre,

    Increment,
    Decrement,

    AddressOf,

    // SUFFIX
    MutatePost,

    Negative,
    Positive,
    Not,

    Cast,

    // INFIX
    Assign,
    Sequence,

    Or,
    Xor,
    And,

    Eq,
    Neq,

    Lt,
    Le,
    Gt,
    Ge,

    ShiftL,
    ShiftR,

    Add,
    Substract,

    Multiply,
    Divide,
    Modulo,

    Thread, // clojure threading operator

    Access,
    InternalCall,
    ExternalCall,

    // ??
    HeapInitialize,
    Deref,
}

impl Operator {
    pub fn from_prefix(kind: TokenKind) -> Result<Self, ()> {
        Ok(match kind {
            TokenKind::Apostrophe => Self::MutatePre,
            TokenKind::Minus => Self::Negative,
            TokenKind::Plus => Self::Positive,
            TokenKind::Tilde => Self::Not,
            TokenKind::Bang => Self::InternalCall,
            TokenKind::At => Self::ExternalCall,
            _ => {
                return Err(());
            },
        })
    }

    pub fn from_postfix(kind: TokenKind) -> Result<Self, ()> {
        Ok(match kind {
            TokenKind::Apostrophe => Self::MutatePost,
            TokenKind::PlusPlus => Self::Increment,
            TokenKind::MinusMinus => Self::Decrement,
            TokenKind::ArrowRight => Self::Cast,
            _ => {
                return Err(());
            },
        })
    }

    pub fn from_infix(kind: TokenKind) -> Result<Self, ()> {
        Ok(match kind {
            TokenKind::Semicolon => Self::Sequence,
            TokenKind::PipePipe => Self::Or,
            TokenKind::CaretCaret => Self::Xor,
            TokenKind::AmpersandAmpersand => Self::And,
            TokenKind::Equals => Self::Eq,
            TokenKind::NotEquals => Self::Neq,
            TokenKind::LessThan => Self::Lt,
            TokenKind::LessThanEquals => Self::Le,
            TokenKind::GreaterThan => Self::Gt,
            TokenKind::GreaterThanEquals => Self::Ge,
            TokenKind::ShiftLeft => Self::ShiftL,
            TokenKind::ShiftRight => Self::ShiftR,
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Substract,
            TokenKind::Star => Self::Multiply,
            TokenKind::Slash => Self::Divide,
            TokenKind::Percent => Self::Modulo,
            TokenKind::Ampersand => Self::AddressOf,
            TokenKind::FatArrowRight => Self::Thread,
            TokenKind::ArrowLeft => Self::Assign,
            TokenKind::ArrowRight => Self::Cast,
            TokenKind::Dot => Self::Access,
            _ => {
                return Err(());
            },
        })
    }
}

#[derive(Debug)]
pub enum ASTKind {
    // Definitions
    LabelDefinition(Option<String>, Vec<LabelAttribute>),
    FunctionDefinition(String, Vec<LabelAttribute>, Box<AST>),

    // Keywords
    Return(Option<Box<AST>>),

    // Expressions
    BinaryExpr(Operator, Box<AST>, Box<AST>),
    UnaryExpr(Operator, Box<AST>),
    Identifier(String),

    IntegerLiteral(usize),
    StringLiteral(String),
    CharLiteral(char),
    HeapLiteral(Vec<AST>),

    Block(Vec<AST>),

    TypeAnnotation(Type, Box<AST>),

    // Calls
    Interrupt(usize),
    Syscall(String, Vec<AST>),
    Call(Box<AST>, Vec<AST>, bool),
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
            ASTKind::BinaryExpr(op, lhs, rhs) => write!(f, "(BinaryExpr {op:?} {lhs} {rhs})")?,
            ASTKind::UnaryExpr(op, operand) => write!(f, "(UnaryExpr {op:?} {operand})")?,
            ASTKind::Identifier(ident) => write!(f, "(Identifier: {ident})")?,
            ASTKind::Block(stmts) => write!(f, "(Block: {} statements)", stmts.len())?,
            ASTKind::StringLiteral(val) => write!(f, "(StringLiteral: {val:?})")?,
            ASTKind::CharLiteral(val) => write!(f, "(CharLiteral: {val:?})")?,
            ASTKind::HeapLiteral(values) => {
                write!(f, "(HeapLiteral {{ ")?;
                values.into_iter().fold(Ok(()), |_, v| write!(f, "{v} "))?;
                write!(f, "}})")?;
            },
            ASTKind::TypeAnnotation(ty, ast) => write!(f, "(TypeAnnotation: {ty} {ast})")?,
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
            ASTKind::Call(name, args, is_external) => write!(
                f,
                "(Call {} {name} Args: ({}))",
                if *is_external { "external" } else { "internal" },
                args.iter().fold(String::new(), |mut acc, arg| {
                    acc.push_str(&format!("{arg} "));
                    acc
                })
            )?,
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
