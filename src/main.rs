use crate::error::{Error, ErrorKind};
use crate::scanner::Scanner;

mod args;
mod error;
mod scanner;
mod span;
mod token;

fn main() {
    let args = args::Args::parse(std::env::args().skip(1).collect());
    let contents = Scanner::get_file(args.file.unwrap_or("main.shd"));
    println!("{} characters in file.", contents.len());

    let (error_sender, error_receiver) = std::sync::mpsc::channel::<Error>();

    println!("{:#?}", args)
}
