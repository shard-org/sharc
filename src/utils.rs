use std::process::exit;
use ansi_term::Colour::RGB;

//
// Colour codes ================
// OK       RGB(0, 153, 51)
// ERR      RGB(179, 0 ,0)
// INFO     
// WARNING  

#[macro_export]
macro_rules! ok {
    ($str:expr) => {
        RGB(0,153,51).bold().paint($str)
    };
}
#[macro_export]
macro_rules! err {
    ($str:expr) => {
        RGB(179,0,0).bold().paint($str)
    };
}
#[macro_export]
macro_rules! info {
    ($str:expr) => {
        RGB(230,230,0).bold().paint($str)
    };
}
#[macro_export]
macro_rules! warn {
    ($str:expr) => {
        RGB(46,184,184).bold().paint($str)
    };
}


//
//eprint and exit
pub fn eprintex(error: &str) -> ! {
    eprintln!("{} {}", RGB(179, 0, 0).bold().paint("ERR:"), error);
    exit(1)
}

pub fn printlnex(message: &str) -> ! {
    println!("{message}");
    exit(0)
}

