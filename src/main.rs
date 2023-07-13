use ansi_term::Colour::RGB;
use std::fs;

mod asm_compile;
mod args_parser;
mod wrapup;
mod defs;
mod utils;
mod parser;

use crate::wrapup::wrapup;
use crate::asm_compile::*;
use crate::args_parser::*;
use crate::utils::eprintex;
use crate::parser::parser;

fn main() {
    let args = parse_args().unwrap_or_else(|e| eprintex(e));
    // returns a Flags struct, see arg_parser file

    eprint!("Reading File... ");
    let in_file_cont = match reader(args.input_file) {
        Ok(stuff) => {
            eprint!("{}", ok!("Done!\n"));
            stuff
        },
        Err(why) => eprintex(why),
    };

    eprint!("Parsing Code... ");
    match parser(in_file_cont) {
        Ok(_parsed) => todo!(),
        Err(why) => eprintex(&why),
    }

    eprint!("Compiling Assembly... ");
    match post_compile() {
        Ok(()) => eprint!("{}", ok!("Done!\n")),
        Err(why) => eprintex(why),
    }

    eprint!("Linking Object Files... ");
    match post_link(args.output_file) {
        Ok(()) => eprint!("{}", ok!("Done!\n")),
        Err(()) => eprintex("Fuck")
    }
    // removes temp files, cleans shit up
    // all those last minute non-essential things
    eprint!("Removing temp files... ");
    wrapup();
}

fn reader(in_file: String) -> Result<String, &'static str> {
    match fs::metadata(&in_file) {
        Ok(_) => (),
        Err(_) => return Err("File Not Found!"),
    }

    match fs::read_to_string(in_file) {
        Ok(f) => Ok(f),
        Err(_) => return Err("Failed To Read File!"),
    }
}
