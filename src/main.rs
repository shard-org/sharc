use std::process::exit;

use ansi_term::Colour::RGB;

mod asm_compile;
mod args_parser;
mod wrapup;
mod defs;

use crate::wrapup::wrapup;
use crate::asm_compile::*;
use crate::args_parser::*;

macro_rules! done {
    () => {
        eprint!("{}", RGB(0, 153, 51).bold().paint("Done!\n"))
    };
}

fn main() {
    let flags = parse_args();
    // returns a Flags struct, see arg_parser file


    eprint!("Compiling Assembly... ");
    match post_compile() {
        Ok(()) => done!(),
        Err(why) => eprintex(why),
    }

    eprint!("Linking Object Files... ");
    match post_link(flags.output_file) {
        Ok(()) => done!(),
        Err(()) => eprintex("Fuck")
    }
    // removes temp files, cleans shit up
    // all those last minute non-essential things
    eprint!("Removing temp files... ");
    wrapup();
}

//
//eprint and exit
fn eprintex(error: &str) -> ! {
    eprint!("{} {}\n", RGB(179, 0, 0).bold().paint("ERR:"), error);
    exit(1)
}

fn printlnex(message: &str) -> ! {
    println!("{message}");
    exit(0)
}
