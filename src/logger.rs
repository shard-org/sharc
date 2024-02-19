#[macro_export]
macro_rules! debug {
    ($($fmt:tt)*) => {
        Log::new().msg(format!($($fmt)*)).print()
    };
}

#[macro_export]
macro_rules! info {
    ($($fmt:tt)*) => {
        Log::new().level($crate::logger::Level::Info).msg(format!($($fmt)*)).print()
    };
}

#[macro_export]
macro_rules! warn {
    ($($fmt:tt)*) => {
        Log::new().level($crate::logger::Level::Warn).msg(format!($($fmt)*)).print()
    };
}

#[macro_export]
macro_rules! err {
    ($($fmt:tt)*) => {
        Log::new().level($crate::logger::Level::Err).msg(format!($($fmt)*)).print()
    };
}

#[macro_export]
macro_rules! fatal {
    ($($fmt:tt)*) => {
        Log::new().level($crate::logger::Level::Fatal).msg(format!($($fmt)*)).print()
    };
}
// ##################################################################


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Level {
    Debug, // cyan
    Info,  // green
    Warn,  // yellow
    Err,   // red
    Fatal, // red, bold
}

use crate::location::Span;
#[derive(Debug)]
pub struct Log {
    level: Level,
    span:  Option<Span>,
    msg:   Box<str>,
    notes: Box<str>,
}

use std::fmt::{self, Display};
impl Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.span {
            Some(span) => {
                let mut form = format!("{}{}\x1b[0m\x1b[1m: {}\x1b[0m\n- <{}> {}:{}\n\x1b[36m{} | \x1b[0m", 
                    self.get_level_colour(),
                    self.get_level_prefix(),
                    self.msg,
                    span.file,
                    span.line,
                    span.col,
                    span.line
                );

                // gets only one line
                let Some(line) = Self::get_file_line(&span.file, &span.line) else {
                    form.push_str("\x1b[31;1mNo source code available\x1b[0m");
                    write!(f, "{}", form)?;
                    return Ok(());
                };

                form.push_str(line.trim());
                form.push_str("\n\x1b[36m  | \x1b[0m");

                form.push_str(self.get_level_colour().as_str());
                (1..span.col).for_each(|_| form.push(' '));
                
                match span.length {
                    Some(length) => (0..length).for_each(|_| form.push('^')),
                    None => form.push('^'),
                }

                write!(f, "{} {}\x1b[0m", form, self.notes)
            },
            None => match self.notes.is_empty() {
                false => write!(f, "{}{}\x1b[0m\x1b[1m: {}\x1b[0m: {}", 
                    self.get_level_colour(),
                    self.get_level_prefix(),
                    self.msg,
                    self.notes
                ),
                true => write!(f, "{}{}\x1b[0m\x1b[1m: {}\x1b[0m",
                    self.get_level_colour(),
                    self.get_level_prefix(),
                    self.msg
                ),
            },
        }
    }
}

use crate::ARGS;
impl Log {
    pub fn new() -> Self {
        Self {
            level: Level::Debug,
            span:  None,
            msg:   Box::default(),
            notes: Box::default(),
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level; self
    }

    pub fn span<T: Into<Option<Span>>>(mut self, span: T) -> Self {
        self.span = span.into(); self
    }

    pub fn msg<T: Display>(mut self, msg: T) -> Self {
        self.msg = msg.to_string().into_boxed_str(); self
    }

    pub fn notes<T: Display>(mut self, notes: T) -> Self {
        self.notes = notes.to_string().into_boxed_str(); self
    }

    
    pub fn push(self, logs: &mut Vec<Log>) {
        logs.push(self);
    }

    pub fn print(&self) {
        // WARN: will halt program if called before ARGS are initialized
        if self.level != Level::Fatal && &self.level < &ARGS.log_lvl 
        { return; }

        println!("{}", self);
    }


    fn get_file_line(file: &str, line: &usize) -> Option<String> {
        let Ok(cont) = std::fs::read_to_string(file) else {
            return None;
        };

        cont.lines().nth(line - 1).map(String::from)
    }

    fn get_level_prefix(&self) -> String {
        match self.level {
            Level::Debug => "[DEBUG]",
            Level::Info  => "[INFO]",
            Level::Warn  => "[WARN]",
            Level::Err   => "[ERR]",
            Level::Fatal => "[FATAL]",
        }.to_string()
    }

    fn get_level_colour(&self) -> String {
        match self.level {
            Level::Debug => "\x1b[34m",
            Level::Info  => "\x1b[32m",
            Level::Warn  => "\x1b[33m",
            Level::Err   => "\x1b[31m",
            Level::Fatal => "\x1b[31;1m",
        }.to_string()
    }
}

pub trait Logs {
    fn print(&self);
    fn sort(self) -> Self;
    fn summary(&self);
}

use std::process::exit;
impl Logs for Vec<Log> {
    fn print(&self) {
        self.iter().for_each(Log::print);
    }

    fn sort(mut self) -> Self {
        self.sort_by(|a, b| a.level.partial_cmp(&b.level).unwrap());
        self
    }

    fn summary(&self) {
        let errors = self.iter().filter(|log| log.level == Level::Err).count();
        let warns = self.iter().filter(|log| log.level == Level::Warn).count();

        // if self.iter().any(|log| log.level == Level::Fatal) { return; }

        if warns != 0 && errors == 0 {
            warn!("{} Warnings Emmited", warns);
        }
        else 
        if warns == 0 && errors != 0 {
            err!("Could Not Compile, {} Errors Emmited", errors);
            exit(1);
        }
        else 
        if warns != 0 && errors != 0 {
            err!("Could Not Compile, {} Errors and {} warnings Emmited", errors, warns);
            exit(1);
        }
    }
}

