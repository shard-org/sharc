use super::*;
use std::fmt::Display;
use std::rc::Rc;
use std::path::Path;


#[macro_export]
macro_rules! log {
    ($lev:expr, $($fmt:tt)*) => {
        logger($lev, None, format!($($fmt)*))
    };
}

#[macro_export]
macro_rules! log_at {
    ($lev:expr, $at:expr, $($fmt:tt)*) => {
        logger($lev, $at, format!($($fmt)*))
    };
}



#[derive(Debug, Clone)]
pub struct At {
    pub file: Rc<Path>,
    pub line: usize,
}

impl Display for At {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.file, self.line)
    }
}

pub struct Exceptions {
    errors: usize,
    warns: usize,
}

#[derive(PartialEq)]
pub enum Level {
    Debug, // cyan
    Ok,    // green
    Warn,  // yellow
    Err,   // red
    Fatal, // red, bold
    WTF,   // purple, bold
}



pub const DEBUG: Level = Level::Debug;
pub const OK: Level = Level::Ok;
pub const WARN: Level = Level::Warn;
pub const ERR: Level = Level::Err;
pub const FATAL: Level = Level::Fatal;
pub const WTF: Level = Level::WTF;


static mut EXC: Exceptions = Exceptions {
    errors: 0,
    warns: 0,
};


pub fn at(line: usize, file: &Path) -> At {
    At { file: file.into(), line, }
}


pub fn logger<T: Display, A: Into<Option<At>>>(lev: Level, at: A, msg: T) {
    unsafe{
        if lev == Level::Debug && !ARGS.debug { return; }
        if (lev != Level::Fatal || lev != Level::WTF) && ARGS.quiet { return; }
    }

    let lev_str = match lev {
        Level::Debug => "\x1b[36m[DEBUG]\x1b[0m",
        Level::Ok    => "\x1b[32m[OK]\x1b[0m",
        Level::Warn  => {unsafe{EXC.warns += 1}; "\x1b[33m[WARN]\x1b[0m" },
        Level::Err   => {unsafe{EXC.errors += 1}; "\x1b[31m[ERR]\x1b[0m" },
        Level::Fatal => "\x1b[31;1m[FATAL]\x1b[0m",
        Level::WTF   => "\x1b[35;1m[WTF]\x1b[0m",
    };

    match at.into() {
        Some(at) => println!("{lev_str} {at}: {msg}"),
        None     => println!("{lev_str} {msg}"),
    }

    match lev {
        Level::Fatal => std::process::exit(1),
        Level::WTF   => log!(FATAL, "If you see this message something went TERRIBLY wrong, please report this"),
        _            => (),
    }
}

pub unsafe fn check_err() {
    if EXC.errors > 0 {
        if EXC.warns > 0 {
            log!(FATAL, "Could not Compile, {} Errors and {} Warnings emmited", EXC.errors, EXC.warns);
        }
        log!(FATAL, "Could not Compile, {} Errors emmited", EXC.errors);
    }
}

pub unsafe fn check_warn() {
    if EXC.warns > 0 {
        log!(WARN, "{} Warnings emmited", EXC.warns);
    }
}
