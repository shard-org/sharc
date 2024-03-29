use std::{
    io::{BufRead, BufReader},
    fs::File,
    collections::VecDeque,
};

use crate::{
    token::{Token, TokenKind},
    location::Span,
    logger::{Log, Level},

    debug,
    utils,
};


pub struct Lexer {
    filename: &'static str,

    file:     BufReader<File>,
    li:       usize, // line counter
    nl:       usize, // advanced without returning tokens //??!?

    chars:    VecDeque<char>, // chars of the current line
    ci:       usize, // char counter
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.advance() {
            use TokenKind::*;
            let token = match c {
                '\n' => NL,
                '&' => Ampersand,
                '@' => At,
                '`' => {
                    let Some(mut c) = self.next_char() else {
                        return self.to_span()
                            .to_log()
                            .msg("Invalid end of char literal")
                            .to_token()
                            .some();
                    };

                    if c == '\\' {
                        let Some(cha) = self.next_char() else {
                            return self.to_span()
                                .to_log()
                                .msg("Invalid end of char literal")
                                .to_token()
                                .some();
                        };

                        c = match self.esc_to_char(cha) {
                            Ok(c) => c,
                            Result::Err(e) => return e.to_token().some(),
                        };
                    }

                    if self.next_char() != Some('`') {
                        return self.to_span()
                            .to_log()
                            .msg("Invalid end of char literal")
                            .to_token()
                            .some();
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
                    while let Some(c) = self.next_char() {
                        match c {
                            '\\' => {
                                if let Result::Err(e) = self.esc_to_char(c).map(|c| lit.push(c)) {
                                    return e.to_token().some();
                                }
                                continue;
                            },
                            '"' => return self.to_span()
                                .to_token(StrLit(lit))
                                .some(),
                            _ => lit.push(c),
                        }
                    }

                    return self.to_span()
                        .to_log()
                        .msg("Invalid end of string literal")
                        .to_token()
                        .some();
                },
                '=' => {
                    if self.test_next('>') {
                        if self.test_next('>') { FatDoubleArrow }
                        else { FatArrow }
                    }
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
                    else if self.test_next('-') { SmallArrowLeft }
                    else { LessThan }
                },
                '-' => {
                    if self.test_next('-') { MinusMinus }
                    else if self.test_next('>') { SmallArrowRight }
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
                '/' => {
                    /* block comments */
                    if self.test_next('*') { 
                        let mut last: char = '\0';
                        while let Some(c) = self.advance() {
                            if last == '*' && c == '/' { break }
                            last = c;
                        } continue;
                    }

                    // line comments
                    if self.test_next('/') { 
                        self.chars.clear();
                        self.chars.push_back('\n');
                        continue;
                    }

                    Slash
                },
                '*' => Star,

                ' ' | '\t' => continue,

                c if c.is_ascii_alphabetic() || c == '_' => {
                    let word = self.word();
                    
                    if word.is_empty() {
                        if c == '_' { Underscore }
                        else { Ident(String::from(c)) }
                    }

                    else {
                        let word = format!("{}{}", c, word);
                        match word.as_str() {
                            // keywords
                            "jmp" => Jmp,
                            "ret" => Ret,
                            "end" => End,
                            "entry" => Entry,
                            "inline" => Inline,

                            _ => Ident(word),
                        }
                    }
                },

                c if c.is_numeric() => {
                    let word = format!("{}{}", c, self.word());

                    if word.contains('.') {
                        FloatLit(word.parse::<f64>().unwrap())
                    }

                    else if word.starts_with('-') || word.starts_with('+') {
                        SIntLit(word.parse::<isize>().unwrap())
                    }

                    else {
                        match utils::parse_int(word.clone()) {
                            Ok(n) => IntLit(n),
                            Result::Err(e) => return self.to_span()
                                .col(|x| x - word.len() - 1)
                                .length(word.len())
                                .to_log()
                                .msg("Invalid integer literal")
                                .notes(e)
                                .to_token()
                                .some(),
                        }
                    }
                },

                t => {
                    debug!("Unknown token: {:?}", t);
                    unreachable!()
                },
            };

            // self.nl = 0;
            return self.to_span().to_token(token).some();
        }
        None
    }
}

impl Lexer {
    pub fn new(file: File, filename: &'static str) -> Lexer {
        Lexer { 
            filename, 
            file: BufReader::new(file), 
            li: 1, 
            nl: 0, 
            chars: VecDeque::new(),
            ci: 1
        }
    }

    fn test_next(&mut self, test: char) -> bool {
         if self.peek().is_some_and(|c| c == test) {
             self.next_char();
             return true;
         } false
    }

    fn advance(&mut self) -> Option<char> {
        self.next_char().or_else(|| self.next_line().map_or(None, |_| self.advance()))
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.pop_front().map(|c| {
            self.ci += 1; c
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.front().copied()
    }

    fn esc_to_char(&mut self, c: char) -> Result<char, Log> {
        let c = match c {
            '@' | '0' => 0,   // NUL | Null
            'A' =>       1,   // SOH | Start of Heading
            'B' =>       2,   // STX | Start of Text
            'C' =>       3,   // ETX | End of Text
            'D' =>       4,   // EOT | End of Transmission
            'E' =>       5,   // ENQ | Enquiry
            'F' =>       6,   // ACK | Acknowledgement
            'G' | 'a' => 7,   // BEL | Bell
            'H' | 'b' => 8,   // BS  | Backspace
            'I' | 't' => 9,   // HT  | Horizontal Tab
            'J' | 'n' => 10,  // LF  | Line Feed
            'K' | 'v' => 11,  // VT  | Vertical Tab
            'L' | 'f' => 12,  // FF  | Form Feed
            'M' | 'r' => 13,  // CR  | Carriage Return
            'N' =>       14,  // SO  | Shift Out
            'O' =>       15,  // SI  | Shift In
            'P' =>       16,  // DLE | Data Link Escape
            'Q' =>       17,  // DC1 | Device Control 1
            'R' =>       18,  // DC2 | Device Control 2
            'S' =>       19,  // DC3 | Device Control 3 (XOFF)
            'T' =>       20,  // DC4 | Device Control 4
            'U' =>       21,  // NAK | Negative Acknowledgement
            'V' =>       22,  // SYN | Synchronous Idle
            'W' =>       23,  // ETB | End of Transmission Block
            'X' =>       24,  // CAN | Cancel
            'Y' =>       25,  // EM  | End of Medium
            'Z' =>       26,  // SUB | Substitute ||| EOF | End of File
            '[' | 'e' => 27,  // ESC | Escape
            // '\\'=>       28,  // FS  | File Separator  // ??????
            ']' =>       29,  // GS  | Group Selector
            '^' =>       30,  // RS  | Record Separator
            '_' =>       31,  // US  | Unit Separator
            '?' =>       127, // DEL | Delete

            '\\' =>      92,  // \ | Backslash
            '"'  =>      34,  // " | Double Quote
            
            _  => {
                return Err(self.to_span()
                    .to_log()
                    .msg(format!("Invalid escaped character `{}`", c)));
            },
        }; Ok(char::from(c))
    }

    fn word(&mut self) -> String {
        let mut word = String::new();
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_'){ break; }

            let _ = self.next_char();
            word.push(c);
        } word
    }

    fn next_line(&mut self) -> Option<()> {
        let mut line = Vec::new();
        match self.file.read_until(b'\n', &mut line) {
            Ok(b) if b == 0 => {
                self.to_span()
                    .line(|x| x-1)
                    .to_log()
                    .msg(format!("EOF: {:?}", self.filename))
                    .level(Level::Debug)
                    .print();
                return None
            },
            Ok(_) => (),
            Err(e) => {
                self.to_span()
                    .to_log()
                    .msg(format!("line get err: {}", e))
                    .print();
                return None;
            },
        }
    
        match String::from_utf8(line) {
            Ok(l) => {
                self.li += 1;
                // self.nl += 1;
                self.ci = 0;
                self.chars = l.chars().collect::<VecDeque<char>>();
                Some(())
            },
            Err(e) => {
                self.to_span()
                    .to_log()
                    .msg("Invalid utf8 in file")
                    .notes(e)
                    .print();
                None
            }
        }
    }

    fn to_span(&self) -> Span {
        Span::new(self.filename, self.li - self.nl - 1, self.ci - 1)
    }
}
