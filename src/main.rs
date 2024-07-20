use crate::error::{Error, ErrorKind};
use crate::scanner::Scanner;

mod args;
mod r#const;
mod error;
mod lexer;
mod scanner;
mod span;
mod token;

pub enum ExitCode {
    OK = 0,
    Generic = 1,
    ArgParsing = 2,
    FileIO = 3,
    Lexer = 9,
    Parser = 16,
    Macro = 22,
    Codegen = 28,
    EasterEgg = 69,
}

pub fn exit(code: ExitCode) -> ! {
    std::process::exit(code as i32)
}

fn main() {
    let args = args::Args::parse(std::env::args().skip(1).collect());
    let contents = Scanner::get_file(args.file.unwrap_or("main.shd"));
    println!("{} characters in file.", contents.len());

    let (error_sender, error_receiver) = std::sync::mpsc::channel::<Error>();

    println!("{:#?}", args)
}
