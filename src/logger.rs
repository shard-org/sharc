use super::*;
use std::fmt::Display;

// FIXME: remove this, this is just a placeholder
#[derive(Clone, Copy, Debug)]
pub struct Location {
    span: Option<(usize, usize)>,
    file: &'static str,
    line: usize,
}


pub const DEBUG: Level = Level::Debug;
pub const OK: Level = Level::Ok;
pub const WARN: Level = Level::Warn;
pub const ERR: Level = Level::Err;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Level {
    Debug, // cyan
    Ok,    // green
    Warn,  // yellow
    Err,   // red
    Fatal, // red, bold
}

#[derive(Debug)]
pub struct Log {
    level:    Level,            // Level::Err
    location: Option<Location>, // Some(Location { span: Some((4, 4)), file: "main.shd", line: 5 })
    msg:      &'static str,     // "Missmatched Parenthesis"
    notes:    &'static str,     // "Expected ')' but found '}'"
}

static mut LOGS: Vec<Log> = Vec::new();

impl Log {
    pub fn new<T: Into<Option<Location>>, M: Display, W: Display>(level: Level, location: T, msg: M, notes: W) -> Self{
        Self {
            level,
            location: location.into(),
            msg: Box::leak(msg.to_string().into_boxed_str()),
            notes: Box::leak(notes.to_string().into_boxed_str()),
        }
    }

    pub fn print(&self) {
        if &self.level < unsafe{ARGS.log_level} { return; }

        match self.location {
            Some(loc) => match loc.span{
                Some(span) => self.print_loc_span(loc, span),
                None => self.print_loc(loc),
            },
            None => println!("{}{}\x1b[0m\x1b[1m: {}\x1b[0m", self.get_level_colour(), self.get_level_prefix(), self.msg),
        }
    }

    pub fn print_all() {
        unsafe{
            LOGS.sort_by(|a, b| a.level.partial_cmp(&b.level).unwrap());
            LOGS.iter().for_each(|log| log.print());
        }
        let errors = LOGS.iter().filter(|log| log.level == Level::Err).count();
        let warns = LOGS.iter().filter(|log| log.level == Level::Warn).count();

        match warns {
            0 if errors > 0 => Log::new(Level::Err, None, format!("Could Not Compile, {} Errors Emmited", errors), "").print(),
            0 => (),
            _ if errors > 0 => Log::new(Level::Warn, None, format!("Could Not Compile, {} Errors and {} warnings Emmited", errors, warns), "").print(),
            _ => Log::new(Level::Warn, None, format!("{} Warnings Emmited", warns), "").print(),
        }

        if errors > 0 {
            std::process::exit(1);
        }
    }

    fn print_loc(&self, loc: Location) {
        let mut form = format!("{}{}\x1b[0m\x1b[1m: {}\x1b[0m\n- <{}>:{}\n\x1b[36m{} | \x1b[0m", self.get_level_colour(), self.get_level_prefix(), self.msg, loc.file, loc.line, loc.line);

        let Some(line) = get_file_line(loc.file, &loc.line) else {
            form.push_str("\x1b[31;1mNo source code available\x1b[0m");
            println!("{}", form);
            return;
        };

        form.push_str(line.trim());
        form.push_str("\n\x1b[36m  | \x1b[0m");

        form.push_str(self.get_level_colour().as_str());
        (0..line.len()).for_each(|_| form.push('^'));
        form.push(' ');
        form.push_str(self.notes);
        form.push_str("\x1b[0m");

        println!("{}", form);
    }

    fn print_loc_span(&self, loc: Location, span: (usize, usize)) {
        let mut form = format!("{}{}\x1b[0m\x1b[1m: {}\x1b[0m\n- <{}>:{}:{}\n\x1b[36m{} | \x1b[0m", self.get_level_colour(), self.get_level_prefix(), self.msg, loc.file, loc.line, span.0, loc.line);

        let Some(line) = get_file_line(loc.file, &loc.line) else {
            form.push_str("\x1b[31;1mNo source code available\x1b[0m");
            println!("{}", form);
            return;
        };

        form.push_str(line.trim());
        form.push_str("\n\x1b[36m  | \x1b[0m");

        form.push_str(self.get_level_colour().as_str());
        (0..span.0).for_each(|_| form.push(' '));
        (span.0..span.1).for_each(|_| form.push('^'));
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
        }.to_string()
    }

    fn get_level_colour(&self) -> String {
        match self.level {
            Level::Debug => "\x1b[34m",
            Level::Ok    => "\x1b[32m",
            Level::Warn  => "\x1b[33m",
            Level::Err   => "\x1b[31m",
            Level::Fatal => "\x1b[31;1m",
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
    (IMMEDIATE: $level:ident, $location:expr, $msg:expr, $notes:expr) => {
        Log::new($level, $location, $msg, $notes).print();
    };
    (IMMEDIATE: $level:ident, $msg:expr, $notes:expr) => {
        Log::new($level, None, $msg, $notes).print();
    };
    (IMMEDIATE: $level:ident, $msg:expr) => {
        Log::new($level, None, $msg, "").print();
    };
    (IMMEDIATE: $msg:expr) => {
        Log::new(Level::Ok, None, $msg, "").print();
    };

    (FATAL, $location:expr, $msg:expr, $notes:expr) => {
        unsafe{
            LOGS.push(Log::new(Level::Fatal, $location, $msg, $notes));
            Log::print_all();
            std::process::exit(1);
        }
    };
    (FATAL, $msg:expr, $notes:expr) => {
        unsafe{
            LOGS.push(Log::new(Level::Fatal, None, $msg, $notes));
            Log::print_all();
            std::process::exit(1);
        }
    };
    (FATAL, $msg:expr) => {
        unsafe{
            LOGS.push(Log::new(Level::Fatal, None, $msg, ""));
            Log::print_all();
            std::process::exit(1);
        }
    };


    ($level:ident, $location:expr, $msg:expr, $notes:expr) => {
        unsafe{LOGS.push(Log::new($level, $location, $msg, $notes))};
    };
    ($level:ident, $msg:expr, $notes:expr) => {
        unsafe{LOGS.push(Log::new($level, None, $msg, $notes))};
    };
    ($level:ident, $msg:expr) => {
        unsafe{LOGS.push(Log::new($level, None, $msg, ""))};
    };
    ($msg:expr) => {
        unsafe{LOGS.push(Log::new(Level::Ok, None, $msg, ""))};
    };


    ($level:ident, fmt: $($fmt:tt)*) => {
        unsafe{LOGS.push(Log::new($level, None, format!($($fmt)*, "")))};
    };
    (fmt: $($fmt:tt)*) => {
        unsafe{LOGS.push(Log::new(Level::Ok, None, format!($($fmt)*, "")))};
    };
}
