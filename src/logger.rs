use std::fmt::Display;
use crate::args_parser::ARGS;
use ansi_term::Colour::RGB;

pub enum Level {
    Ok,
    Warn,
    Err,
    Debug,
}

pub struct At {
    pub file: String,
    pub line: usize,
}

#[derive(PartialEq)]
pub enum Debug {
    Parser,
    PreProcessor,
    Compiler,
    Assembler,
    Linker,
    Writer,
    Reader,
    Wrapup,
    ArgParser,
    None,
}

pub fn logger<T: Display>(
    lev: Level,
    at: Option<At>,
    debug: &Debug,
    msg: T
) {
    // set the level
    let lev = match lev {
        Level::Ok    => RGB(0, 153, 51).bold().paint("OK"),
        Level::Err   => RGB(179, 0, 0).bold().paint("ERR:"),
        Level::Debug => RGB(46, 184, 184).bold().paint("DEBUG:"),
        Level::Warn  => RGB(230, 230, 0).bold().paint("WARN:"),
        // Level::Info  => RGB(57, 96, 96).bold().paint("INFO:"),
        // Level::Tip   => RGB(255, 179, 255).bold().paint("TIP:"),
    };

    // Print the message if there is no debug info
    if debug == &Debug::ArgParser || !ARGS.debug {
        if at.is_none() {
            println!("{lev} {msg}");
            return;
        }

        let at = at.unwrap();
        println!("{lev} {}:{}: {msg}", at.file, at.line);
        return;
    }

    // set the debug info if it exists, bolded using ANSI escape codes
    let dir = match debug {
        Debug::Parser      => "at \x1b[1mPARSER\x1b[0m",
        Debug::PreProcessor=> "at \x1b[1mPREPROCESSOR\x1b[0m",
        Debug::Compiler    => "at \x1b[1mCOMPILER\x1b[0m",
        Debug::Assembler   => "at \x1b[1mASSEMBLER\x1b[0m",
        Debug::Linker      => "at \x1b[1mLINKER\x1b[0m",
        Debug::Writer      => "at \x1b[1mWRITER\x1b[0m",
        Debug::Reader      => "at \x1b[1mREADER\x1b[0m",
        Debug::Wrapup      => "at \x1b[1mWRAPUP\x1b[0m",
        Debug::ArgParser   => "at \x1b[1mARGPARSER\x1b[0m",
        Debug::None        => "",
    };

    if at.is_none() {
        println!("{lev} \x1b[1m{}:\x1b[0m {msg}", dir);
        return;
    }

    let at = at.unwrap();

    // Print the message
    println!("{lev} \x1b[1m{}:\x1b[0m {}:{}: {msg}", dir, at.file, at.line);
}
