use std::iter::{Enumerate, Peekable};
use std::str::{Chars, Lines};


use crate::token::{Token, TokenKind, RegSize};
use crate::location::Span;
use crate::logger::{Log, Logs, Level};

use crate::debug;

pub struct Lexer<'a> {
    logger:   &'a mut Vec<Log>,
    filename: &'static str,

    lines:    Lines<'a>, // first element is always the next line
    li:       usize, // line counter
    nl:       usize, // advanced without returning tokens

    chars:    Peekable<Enumerate<Chars<'a>>>, // chars of the current line
    ci:       usize, // char counter

    tokens:   Vec<Token>,
}

impl Lexer<'_> {
    pub fn new<'a>(input: &'a str, logger: &'a mut Vec<Log>, filename: &'static str) -> Lexer<'a> {
        let mut lines = input.lines();

        let Some(line) = lines.next() else {
            panic!("file cannot be empty... the reader should filter this");
        };

        let chars = line.chars().enumerate().peekable();

        Lexer{ logger, filename, lines, li: 1, nl: 0, chars, ci: 1, tokens: Vec::new()}
    }

    pub fn lex(mut self) -> Vec<Token> {
        'main: while let Some(c) = self.advance() {
            use TokenKind::*;
            let token = match c {
                '&' => Ampersand,
                '@' => At,
                '`' => {
                    let Some(mut c) = self.next() else {
                        self.to_span()
                            .to_log()
                            .msg("Invalid end of char literal")
                            .push(self.logger);
                        continue;
                    };

                    if c == '\\' {
                        let Some(cha) = self.next() else {
                            self.to_span()
                                .to_log()
                                .msg("Invalid end of char literal")
                                .push(self.logger);
                            continue;
                        };

                        c = match self.esc_to_char(cha) {
                            Some(c) => c,
                            None => continue,
                        };
                    }

                    if self.next() != Some('`') {
                        self.to_span()
                            .to_log()
                            .msg("Invalid end of char literal")
                            .print();
                            // .push(self.logger);
                        continue;
                    }

                    CharLit(c)
                },
                '\\'=> Backslash,
                '!' => Bang,
                '^' => Caret,
                ':' => Colon,
                ',' => Comma,
                '$' => Dollar,
                '.' => Dot,
                '"' => {
                    let mut lit = String::new();
                    while let Some(c) = self.next() {
                        match c {
                            '\\' => {
                                self.esc_to_char(c).map(|c| lit.push(c));
                                continue;
                            },
                            '"' => {
                                let token = StrLit(lit);
                                self.tokens.push(self.to_span().to_token(token));
                                continue 'main;
                            },
                            _ => lit.push(c),
                        }
                    }

                    self.to_span()
                        .to_log()
                        .msg("Invalid end of string literal")
                        .push(self.logger);
                    continue;
                },
                '=' => {
                    if self.test_next('>') { FatArrow }
                    else { Equals }
                },
                '>' => {
                    if self.test_next('=') { GreaterThanEquals }
                    else { GreaterThan }
                },
                '{' => LeftBrace,
                '[' => LeftBracket,
                '(' => LeftParen,
                '<' => {
                    if self.test_next('=') { LessThanEquals }
                    else { LessThan }
                },
                '-' => {
                    if self.test_next('-') { MinusMinus }
                    else if self.test_next('>') { TinyArrowRight }
                    else { Minus }
                },
                '~' => {
                    if self.test_next('=') { NotEquals }
                    else { Tilde }
                },
                '%' => Percent,
                '|' => Pipe,
                '+' => {
                    if self.test_next('+') { PlusPlus }
                    else { Plus }
                },
                '#' => Pound,
                '?' => Question,
                '}' => RightBrace,
                ']' => RightBracket,
                ')' => RightParen,
                ';' => Semicolon,
                '\''=> SingleQuote,
                '/' => Slash,
                '*' => Star,
                ' ' | '\t' => {
                    // prevent doubled whitespace
                    if self.tokens.last().is_some_and(|t| t.kind == WS) { continue; }
                    WS
                },

                c if c.is_ascii_alphabetic() || c == '_' => {
                    let word = self.word();
                    
                    if word.is_empty() {
                        if c == '_' { Underscore }
                        else { Ident(String::from(c)) }
                    }

                    else {
                        let mut word = format!("{}{}", c, word);

                        // registers
                        if let Some(word) = word.strip_prefix('r') {
                            if word.chars().next().unwrap().is_numeric() { 
                                let Some(token) = self.parse_register(word.to_string()) else {
                                    continue;
                                };

                                self.tokens.push(self.to_span().to_token(token));
                                continue;
                            }
                        }

                        match word.as_str() {
                            // keywords
                            "jmp" => Jmp,
                            "ret" => Ret,
                            "end" => End,
                            "init" => Init,
                            "static" => Static,
                            "const" => Const, 
                            "entry" => Entry,
                            "inline" => Inline,

                            _ => Ident(word),
                        }
                    }
                },

                '0' => if let Some(c) = self.next() { 
                    let word = self.word();
                    match c {
                        'b' => BinLit(self.num(word, 2).unwrap()),
                        'o' => OctLit(self.num(word, 8).unwrap()),
                        'x' => HexLit(self.num(word, 16).unwrap()),

                        n if n.is_numeric() => {
                            let word = format!("{}{}", n, word);
                            DecLit(self.num(word, 10).unwrap())
                        },

                        _ => {
                            self.to_span()
                                .to_log()
                                .msg("Unexpected token in integer literal")
                                .push(self.logger);
                            continue;
                        },
                    }
                } 
                else { DecLit(0) },


                c if c.is_numeric() => {
                    let word = format!("{}{}", c, self.word());

                    if word.contains('.') {
                        FloatLit(word.parse::<f64>().unwrap())
                    }
                    else {
                        DecLit(self.num(word, 10).unwrap()) 
                    }
                },


                t => {
                    debug!("Unknown token: {:?}", t);
                    panic!()
                },
            };


            
            self.nl = 0;
            self.tokens.push(self.to_span().to_token(token));
        }

        self.tokens
    }

    // always TokenKind::Register
    fn parse_register(&mut self, mut reg: String) -> Option<TokenKind> {
        use RegSize::*;
        let size = match reg.pop().unwrap() {
            'd' => Double,
            'l' => Long,
            's' => Short,
            'h' => HighByte, // high byte represented as 0... prob better idea to have some kind of enum
                      // for register size but it is what it is. This is a legacy feature of x86
                      // anyway so we might end up removing it as no compiler actually takes it
                      // into account.. maybe?
            'b' => Byte,
            _ => {
                self.to_span()
                    .length(reg.len())
                    .to_log()
                    .msg("Invalid token in register identifier")
                    .push(self.logger);
                return None;
            },
        };

        let Ok(num) = reg.parse::<usize>() else {
            self.to_span()
                .length(reg.len())
                .to_log()
                .msg("Invalid token in register identifier")
                .push(self.logger);
            return None;
        };

        Some(TokenKind::Register(size, num))
    }

    fn test_next(&mut self, test: char) -> bool {
         self.peek().is_some_and(|c| c == test)
    }

    fn advance(&mut self) -> Option<char> {
        self.next().or_else(|| {
            match self.next_line() {
                Some(l) => self.advance(),
                None => {
                    self.tokens.push(self.to_span().to_token(TokenKind::EOF));
                    None
                },
            }
        })
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            self.ci = c.0; c.1
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|c| c.1)
    }

    fn esc_to_char(&mut self, c: char) -> Option<char> {
        let c = match c {
            'n' => '\n',
            't' => '\t',
            'r' => '\t',
            '\\' => '\\',
            '"' => '"',
            _  => {
                self.to_span()
                    .to_log()
                    .msg(format!("Invalid escaped character `{}`", c))
                    .push(self.logger);
                return None;
            },
        }; Some(c)
    }

    fn num(&mut self, word: String, base: u32) -> Option<usize> {
        usize::from_str_radix(&word, base).map_or_else(|e| {
            self.to_span()
                .length(word.len())
                .to_log()
                .msg("Invalid integer literal")
                .notes(e)
                .push(self.logger);
            None
        }, |n| Some(n))
    }

    fn word(&mut self) -> String {
        let mut word = String::new();
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_'){ break; }

            let _ = self.next();
            word.push(c);
        } word
    }

    fn next_line(&mut self) -> Option<&str> {
        match self.lines.next() { 
            Some(l) if l.trim().is_empty() => self.next_line(),
            Some(l) => {
                self.li += 1;
                self.nl += 1;
                self.chars = l.chars().enumerate().peekable();


                self.tokens.push(self.to_span().to_token(TokenKind::NL));

                Some(l)
            }
            None => None,
        }
    }

    fn to_span(&self) -> Span {
        Span::new(self.filename, self.li - self.nl, self.ci + 1)
    }

}
