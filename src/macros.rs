use crate::location::Span;
use crate::logger::{Log, Level};
use crate::debug;

#[derive(Debug)]
pub enum Macro {
    Func(), // TODO
    Def(String, String),
}

impl Macro {
    pub fn parse(input: &str, logs: &mut Vec<Log>, filename: &str) -> Vec<Macro> {
        let mut macros: Vec<Macro> = Vec::new();

        let mut lines = input.lines().enumerate();

        while let Some((li, line)) = lines.next() {
            let mut chars = line.chars().enumerate().peekable();
            let li = li + 1;

            while let Some((ci, ch)) = chars.next() {
                let ci = ci + 1;
                if !line.trim().starts_with('/') {
                    break;
                }

                let Some((ident_i, ident)) = chars.word() else {
                    Span::new(filename, li, ci)
                        .to_log()
                        .msg("Missing macro ident after `/`")
                        .level(Level::Err)
                        .push(logs);
                    break;
                };

                if let Some(w) = chars.word() {
                    if w.1 != "="  {
                        // FIXME this only approximates func macros, prob also check if the line
                        // ends with a `{` as well
                        Span::new(filename, li, 1)
                            .length(line.len())
                            .to_log()
                            .msg("Function like macros aren't yet implemented")
                            .level(Level::Err)
                            .push(logs);
                        break;
                    }

                    let value = chars
                        .map(|c| c.1)
                        .take_while(|&c| c != '\n')
                        .collect::<String>();

                    if value.is_empty() {
                        Span::new(filename, li, line.len() + 1)
                            .length(5)
                            .to_log()
                            .msg("Macro missing a value")
                            .level(Level::Err)
                            .push(logs);
                        break;
                    }

                    macros.push(Macro::Def(ident, value));

                    break;
                }

                Span::new(filename, li, ident_i + 2)
                    .length(5)
                    .to_log()
                    .msg("Macro missing a value")
                    .level(Level::Err)
                    .push(logs);
            }
        }
        macros
    }

    pub fn apply(macros: Vec<Macro>, input: &mut String) {
        for macro_ in macros {
            match macro_ {
                Macro::Def(ident, value) => {
                    let mut start = 0;
                    while let Some(i) = input[start..].find(&format!("#{}", ident)) {
                        let i = start + i;
                        debug!("{}", &input[i..i+ident.len()+1]);
                        start = i + 1;
                    }
                },
                Macro::Func() => todo!("function macros not yet implemented"),
            }
        }
    }
}



trait IterExt {
    fn word(&mut self) -> Option<(usize, String)>;
}

use std::iter::{Peekable, Enumerate};

impl<I: Iterator<Item = char>> IterExt for Peekable<Enumerate<I>> {
    fn word(&mut self) -> Option<(usize, String)> {
        let mut word = String::new();

        let _ = self.skip_while(|&c| c.1.is_whitespace());

        match self.peek() {
            Some((_, '\n')) => return None,
            _ => (),
        }

        let mut i = 0;
        while let Some(c) = self.next() {
            if c.1.is_whitespace() { break; }
            word.push(c.1);
            i = c.0;
        }

        if word.is_empty() {
            return None;
        } 
        Some((i+1, word))
    }
}
