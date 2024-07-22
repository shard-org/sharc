use std::{cmp, u64::MAX};

use log::error;

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

    pub fn push(&mut self, stmt: Ast<'source>) {
        self.stmts.push(stmt);
    }
}

pub struct Ast<'source> {
    pub start: usize, // used for span construction for reports
    pub end: usize,

    pub kind: Box<AstKind<'source>>,
}

pub struct FuncParam<'source> {
    ident: &'source str,
    param_type: Ast<'source>,
}

pub enum AstKind<'source> {
    Invalid,
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

    pub fn advance(&mut self) {
        if self.current != self.tokens.len() - 1 {
            self.current += 1;
        }
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn peek(&self, offset: isize) -> &Token {
        &self.tokens[cmp::max((self.current as isize + offset) as usize, self.tokens.len() - 1)]
    }

    pub fn new_ast(&self, kind : AstKind<'source>) -> Ast<'source> {
        Ast {
            start: self.current,
            end: self.current,
            kind: kind.into(),
        }
    }

    pub fn new(tokens: Box<[Token<'source>]>) -> Self {
        Parser {
            tokens,
            current: 0,
            program: Program::new(),
        }
    }

    pub fn parse_program(&mut self) {

    }

    pub fn parse_expr_atom(&mut self) -> Option<Ast> {
        let text = self.current().text;
        match self.current().kind {
            TokenKind::DecimalIntLiteral => {
                let mut val = text.parse::<isize>();
                
                if val.is_err() {
                    todo!("emit parse error message");
                }

                self.new_ast(AstKind::IntegerLiteralExpr { val: val.unwrap() }).into()
            }
            _ => None
        }
    }

}
