use crate::{
    lexer,
    token::{Token, TokenKind},
};

// 'source is the lifetime for the source text that backs the lexer and parser
pub struct Parser<'source> {
    tokens: Box<[Token<'source>]>,
    current: usize,
    program: Program<'source>,
}

pub struct Program<'source> {
    stmts: Vec<Ast<'source>>,
}

impl<'source> Program<'source> {
    pub fn new() -> Self {
        Program { stmts: Vec::new() }
    }
}

pub struct Ast<'source> {
    start: usize, // used for span construction for reports
    end: usize,

    kind: Box<AstKind<'source>>,
}

pub struct FuncParam<'source> {
    ident: &'source str,
    param_type: Ast<'source>,
}

pub enum AstKind<'source> {
    Identifier,
    IntegerLiteralExpr {
        val: isize,
    },
    FloatLiteralExpr {
        val: f64,
    },
    StringLiteralExpr {
        val: String,
    },
    BinopExpr {
        kind: TokenKind,
        rhs: Ast<'source>,
        lhs: Ast<'source>,
    },
    UnopExpr {
        kind: TokenKind,
        inner: Ast<'source>,
    },

    BlockStmt {
        stmts: Vec<Ast<'source>>,
    },
    FuncDeclStmt {
        ident: &'source str,
        params: Vec<FuncParam<'source>>,
    },

    // only doing simple type structures for now, no arrays or structs or w/e
    SimpleType {
        size: usize,
    },
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Box<[Token<'source>]>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
            program: Program::new(),
        }
    }
}
