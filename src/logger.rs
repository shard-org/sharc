use super::*;
use crate::location::{Location, Span};
use std::fmt::Display;

pub const DEBUG: Level = Level::Debug;
pub const OK: Level = Level::Ok;
pub const WARN: Level = Level::Warn;
pub const ERR: Level = Level::Err;
pub const FATAL: Level = Level::Fatal;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Level {
    Debug, // cyan
    Ok,    // green
    Warn,  // yellow
    Err,   // red
    Fatal, // red, bold
    None,  // reserved by the args parser
}

#[derive(Debug)]
pub struct Log {
    level: Level,        // Level::Err
    span:  Option<Span>, // Some(Span { span: Some((4, 4)), file: "main.shd", line: 5 })
    msg:   &'static str, // "Missmatched Parenthesis"
    notes: &'static str, // "Expected ')' but found '}'"
}

static mut LOGS: Vec<Log> = Vec::new();

impl Log {
    pub fn new<T: Into<Option<Span>>, M: Display, W: Display>(level: Level, span: T, msg: M, notes: W) -> Self{
        Self {
            level,
            span: span.into(),
            msg: Box::leak(msg.to_string().into_boxed_str()),
            notes: Box::leak(notes.to_string().into_boxed_str()),
        }
    }

    //
    // general
    pub fn print(self) {
        let level = self.level.clone();
        self.print_internal();

        if level == Level::Fatal {
            Self::handle_fatal();
        }
    }

    pub fn push(self) {
        let level = self.level.clone();
        unsafe{ LOGS.push(self); }

        if level == Level::Fatal {
            Self::handle_fatal();
        }
    }
    //
    // specific
    pub fn print_all() {
        unsafe{
            LOGS.sort_by(|a, b| a.level.partial_cmp(&b.level).unwrap());
            LOGS.iter().for_each(|log| log.print_internal());

            let errors = LOGS.iter().filter(|log| log.level == Level::Err).count();
            let warns = LOGS.iter().filter(|log| log.level == Level::Warn).count();

            if !LOGS.iter().any(|log| log.level == Level::Fatal) {
                match warns {
                    0 if errors > 0 => Log::new(Level::Err, None, format!("Could Not Compile, {} Errors Emmited", errors), "").print(),
                    0 => (),
                    _ if errors > 0 => Log::new(Level::Warn, None, format!("Could Not Compile, {} Errors and {} warnings Emmited", errors, warns), "").print(),
                    _ => Log::new(Level::Warn, None, format!("{} Warnings Emmited", warns), "").print(),
                }
            }

            LOGS.clear();

            if errors > 0 {
                std::process::exit(1);
            }
        }
    }

    //
    // internal
    fn print_internal(&self) {
        if &self.level < unsafe{&ARGS.log_level} { return; }

        match self.span {
            Some(span) => self.print_highlighted(span),
            None => match self.notes.is_empty() {
                false => println!("{}{}\x1b[0m\x1b[1m: {}\x1b[0m: {}", 
                    self.get_level_colour(),
                    self.get_level_prefix(),
                    self.msg,
                    self.notes
                ),
                true => println!("{}{}\x1b[0m\x1b[1m: {}\x1b[0m",
                    self.get_level_colour(),
                    self.get_level_prefix(),
                    self.msg
                ),
            },
        }
    }

    fn handle_fatal() {
        Log::print_all();
        println!("\x1b[31;1mEXITING!!!\x1b[0m");
        std::process::exit(1);
    }
    
    fn print_highlighted(&self, span: Span) {
        let mut form = format!("{}{}\x1b[0m\x1b[1m: {}\x1b[0m\n- <{}>{}:{}\n\x1b[36m{} | \x1b[0m", 
            self.get_level_colour(),
            self.get_level_prefix(),
            self.msg,
            span.0,
            span.1.line,
            span.1.column,
            span.1.line
        );

        // gets only one line
        let Some(line) = get_file_line(span.0, &span.1.line) else {
            form.push_str("\x1b[31;1mNo source code available\x1b[0m");
            println!("{}", form);
            return;
        };

        form.push_str(line.trim());
        form.push_str("\n\x1b[36m  | \x1b[0m");

        form.push_str(self.get_level_colour().as_str());
        (1..span.1.column).for_each(|_| form.push(' '));
        (span.1.column..=span.2.column).for_each(|_| form.push('^'));
        form.push(' ');
        form.push_str(self.notes);
        form.push_str("\x1b[0m");

        println!("{}", form);
    }

    fn get_level_prefix(&self) -> String {
        match self.level {
            Level::Debug => "[DEBUG]",
            Level::Ok    => "[OK]",
            Level::Warn  => "[WARN]",
            Level::Err   => "[ERR]",
            Level::Fatal => "[FATAL]",
            _ => unreachable!(),
        }.to_string()
    }

    fn get_level_colour(&self) -> String {
        match self.level {
            Level::Debug => "\x1b[34m",
            Level::Ok    => "\x1b[32m",
            Level::Warn  => "\x1b[33m",
            Level::Err   => "\x1b[31m",
            Level::Fatal => "\x1b[31;1m",
            _ => unreachable!(),
        }.to_string()
    }
}

fn get_file_line(file: &str, line: &usize) -> Option<String> {
    let Ok(cont) = std::fs::read_to_string(file) else {
        return None;
    };

    cont.lines().nth(line - 1).map(String::from)
}

#[macro_export]
macro_rules! log {
    ($level:ident, $($fmt:tt)*) => {
        Log::new($level, None, format!($($fmt)*), "")
    };
    ($($fmt:tt)*) => {
        Log::new(Level::Ok, None, format!($($fmt)*), "")
    };
}
