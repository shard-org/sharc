use std::cmp;

use log::error;

use crate::{ lexer, span::Span, token::{Token, TokenKind} };

// 'source is the lifetime for the source text that backs the lexer and parser
pub struct Parser<'source> {
    pub filename: &'static str,
    pub tokens: Box<[Token<'source>]>,
    pub current: usize,
    pub program: Program<'source>,
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

#[derive(Debug)]
pub struct Ast<'source> {
    pub start: usize, // used for span construction for reports
    pub end: usize,

    pub kind: Box<AstKind<'source>>,
}

impl<'source> Ast<'source> {
    pub fn to_span(&self, p: &Parser) -> Span {
        Span {
            filename:    p.filename,
            line_number: p.tokens[self.start].span.line_number,
            start_index: p.tokens[self.start].span.start_index,
            end_index:   p.tokens[self.end].span.end_index,
        }
    }
}

#[derive(Debug)]
pub struct FuncParam<'source> {
    ident: &'source str,
    param_type: Ast<'source>,
}

#[derive(Debug)]
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

    pub fn new(tokens: Box<[Token<'source>]>, filename: &'static str) -> Self {
        Parser {
            filename,
            tokens,
            current: 0,
            program: Program::new(),
        }
    }

    pub fn parse_program(&mut self) {

    }

    pub fn parse_expr_atom(&mut self) -> Option<Ast> {
        let text = self.current().text;
        let node = match self.current().kind {
            TokenKind::BinaryIntLiteral => {
                let mut val =  isize::from_str_radix(text, 2);
                
                if val.is_err() {
                    todo!("emit parse error message");
                }

                self.new_ast(AstKind::IntegerLiteralExpr { val: val.unwrap() }).into()
            }
            TokenKind::OctalIntLiteral => {
                let mut val =  isize::from_str_radix(text, 8);
                
                if val.is_err() {
                    todo!("emit parse error message");
                }

                self.new_ast(AstKind::IntegerLiteralExpr { val: val.unwrap() }).into()
            }
            TokenKind::DecimalIntLiteral => {
                let mut val =  isize::from_str_radix(text, 10);
                
                if val.is_err() {
                    todo!("emit parse error message");
                }

                self.new_ast(AstKind::IntegerLiteralExpr { val: val.unwrap() }).into()
            }
            TokenKind::HexadecimalIntLiteral => {
                let mut val = isize::from_str_radix(text, 16);
                
                if val.is_err() {
                    todo!("emit parse error message");
                }

                self.new_ast(AstKind::IntegerLiteralExpr { val: val.unwrap() }).into()
            }
            TokenKind::FloatLiteral => {
                let mut val = text.parse::<f64>();

                if val.is_err() {
                    todo!("emit parse error message");
                }

                self.new_ast(AstKind::FloatLiteralExpr { val: val.unwrap() }).into()
            }
            TokenKind::Identifier => {
                self.new_ast(AstKind::Identifier).into() // identifiers just use the text of their first token
            }
            _ => None
        };
        self.advance();
        node
    }

}
