use ansi_term::Colour::RGB;
use crate::defs::TIPS;
use rand::seq::SliceRandom;
use std::fs;

pub enum Level {
    Ok,
    Warn,
    Err,
    Debug,
    Info,
    Tip,
}

pub enum At {
    Parser,
    ArgParser,
    Compiler,
    Nasm,
    Ld,
    Writer,
    PreCompiler,
    Wrapup,
    None,
}

// this is just to help my obsession with shortening everything
// hate useless verbocity, like `.to_string()`, 12 chars for a really common method?!?
#[macro_export]
macro_rules! st {
    ($cnv:expr) => {
        $cnv.to_string()
    };
} 

#[macro_export]
macro_rules! testo {
    ($chk:expr, $err:block) => {
        match $chk {
            Some(e) => e,
            None => {
                $err
            }
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($expr:expr) => {
        return Err($expr)
    };
}


pub fn logfmt<T: std::fmt::Display>(line: &usize, filename: &str, msg: T) -> String {
    format!("{filename}:{}: {msg}", line+1)
}

pub fn logger<T: std::fmt::Display>(lev: Level, at: &At, msg: T) {
    let dir = match at {
        At::Parser      => format!(" at {}", RGB(255,255,255).bold().paint("PARSER")),
        At::ArgParser   => format!(" at {}", RGB(255,255,255).bold().paint("ARGPARSER")),
        At::Compiler    => format!(" at {}", RGB(255,255,255).bold().paint("COMPILER")),
        At::Nasm        => format!(" at {}", RGB(255,255,255).bold().paint("NASM")),
        At::Ld          => format!(" at {}", RGB(255,255,255).bold().paint("LD")),
        At::Writer      => format!(" at {}", RGB(255,255,255).bold().paint("WRITER")),
        At::PreCompiler => format!(" at {}", RGB(255,255,255).bold().paint("PRECOMPILER")),
        At::Wrapup      => format!(" at {}", RGB(255,255,255).bold().paint("WRAPUP")),
        At::None        => String::new(),
    };

    match lev {
        Level::Ok    => println!("{}{dir} {msg}", RGB(0, 153, 51).bold().paint("OK")), 
        Level::Err   => println!("{}{dir}: {msg}", RGB(179, 0, 0).bold().paint("ERR")), 
        Level::Debug => println!("{}{dir}: {msg}", RGB(46, 184, 184).bold().paint("DEBUG")), 
        Level::Warn  => println!("{}{dir}: {msg}", RGB(230, 230, 0).bold().paint("WARN")), 
        Level::Info  => println!("{}{dir}: {msg}", RGB(57, 96, 96).bold().paint("INFO")), 
        Level::Tip   => println!("{}{dir}: {msg}", RGB(255, 179, 255).bold().paint("TIP")), 
    }
}

pub fn get_tip() -> &'static str {
    TIPS.choose(&mut rand::thread_rng()).unwrap_or(&"Failed to Fetch Tip!")
}

pub fn reader(in_file: &str) -> Result<String, String> {
    match fs::metadata(in_file) {
        Ok(_) => (),
        Err(_) => return Err(format!("File `{in_file}` Not Found!")),
    }

    let file = match fs::read_to_string(in_file) {
        Ok(f) => f,
        Err(_) => return Err("Failed To Read File!".into()),
    };

    if file.replace(char::is_whitespace, "").is_empty() {
        return Err(format!("File `{in_file}` is Empty"));
    }

    Ok(file)
}


pub fn remover(filename: &str) -> Result<(), &'static str> {
    match fs::remove_file(filename) {
        Ok(()) => (),
        Err(_) => return Err("Failed to Remove File"),
    }

    Ok(())
}

// TODO: this is a mess... make it return the exact token that it failed on, prob Option<char>
pub fn validate_str(s: &str) -> bool {
    if s.chars().any(|c| 
        !(c.is_ascii_alphabetic() || c == '_') || 
        (c.is_uppercase() && s.chars().any(|pc| pc.is_lowercase()))
    ) {
        logger(Level::Warn, &At::Parser, "Use snake_case or ANGRY_SNAKE_CASE");
        return false;
    }

    true
}
