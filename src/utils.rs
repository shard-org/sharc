use ansi_term::Colour::RGB;
use std::process::exit;

#[macro_export]
macro_rules! ok {
    ($str:expr) => {
        RGB(0, 153, 51).bold().paint($str)
    };
}
#[macro_export]
macro_rules! err {
    () => {
        RGB(179, 0, 0).bold().paint("ERR: ")
    };
}
#[macro_export]
macro_rules! deb {
    () => {
        RGB(46, 184, 184).bold().paint("DEBUG: ")
    };
}
#[macro_export]
macro_rules! warn {
    () => {
        RGB(230, 230, 0).bold().paint("WARN: ")
    };
}

//
//eprint and exit
pub fn eprintex(error: &str) -> ! {
    eprintln!("{} {error}", err!());
    exit(1)
}

pub fn printlnex(message: &str) -> ! {
    println!("{message}");
    exit(0)
}
