use crate::location::Span;
use crate::logger::{Log, Level};

#[derive(Debug)]
pub enum Macro {
    Func(), // TODO
    Def(String, String),
}

impl Macro {
    pub fn parse(input: &str, logs: &mut Vec<Log>, filename: &'static str) -> Vec<Macro> {
        let mut macros: Vec<Macro> = Vec::new();
        macros.push(Macro::Def(String::from("FILE"), filename.to_string()));

        let mut lines = input.lines().enumerate();

        while let Some((li, line)) = lines.next() {
            let mut chars = line.chars().enumerate().peekable();
            let li = li + 1;
            let ci = 1;

            if !line.trim().starts_with('/') { continue; }

            let _ = chars.next();

            let Some((ident_i, ident)) = chars.word() else {
                Span::new(filename, li, ci)
                    .into_log()
                    .msg("Missing macro ident after `/`")
                    .level(Level::Err)
                    .push(logs);
                continue;
            };

            if let Some(w) = chars.word() {
                if w.1 != "="  {
                    // FIXME this only approximates func macros, prob also check if the line
                    // ends with a `{` as well
                    Span::new(filename, li, 1)
                        .length(line.len())
                        .into_log()
                        .msg("Function like macros aren't yet implemented")
                        .level(Level::Err)
                        .push(logs);
                    continue;
                }

                let value = chars
                    .map(|c| c.1)
                    .take_while(|&c| c != '\n')
                    .collect::<String>();

                if value.is_empty() {
                    Span::new(filename, li, line.len() + 1)
                        .length(5)
                        .into_log()
                        .msg("Macro missing a value")
                        .level(Level::Err)
                        .push(logs);
                    continue;
                }

                macros.push(Macro::Def(ident, value));
                continue;
            }

            Span::new(filename, li, ident_i + 2)
                .length(5)
                .into_log()
                .msg("Macro missing a value")
                .level(Level::Err)
                .push(logs);
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
                        start = i + 1;

                        if input.chars().nth(i-1) == Some('\\') { 
                            input.replace_range(i-1..i, "");
                            continue; 
                        }
                        input.replace_range(i..=i+ident.len(), &value);
                    }
                },
                Macro::Func() => todo!("function macros not yet implemented"),
            }
        }
    }
}



pub trait IterExt {
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
