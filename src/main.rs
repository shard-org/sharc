use ansi_term::Colour::RGB;
use std::process::exit;
use std::fs;

mod args_parser;
mod asm_compile;
mod defs;
mod parser;
mod utils;
mod wrapup;
mod compiler;

use crate::args_parser::*;
use crate::asm_compile::*;
use crate::parser::parser;
use crate::utils::eprintex;
use crate::wrapup::wrapup;
use crate::compiler::compiler;

fn main() {
    let args = parse_args().unwrap_or_else(|e| eprintex(e));
    // returns a Flags struct, see arg_parser file

    if args.debug {
        eprintln!("{}{args:?}", deb!())
    }

    eprint!("Reading File... ");
    let in_file_cont = match reader(args.input_file) {
        Ok(stuff) => {
            eprintln!("{}", ok!("Done!"));
            stuff
        },
        Err(why) => eprintex(&why),
    };

    eprintln!("Parsing Code... ");
    let tokens = match parser(in_file_cont, args.debug) {
        Ok(parsed) => {
            eprintln!("{}", ok!("Done!"));
            parsed
        },
        Err(why) => {
            why.into_iter().rev().for_each(|e| eprintln!("{} {e}", err!()));
            return;
        },
    };


    eprintln!("Compiling... ");
    let asm_output = match parser(tokens, args.debug) {
        Ok(out) => {
            eprintln!("{}", ok!("Done!"));
            out
        },
        Err(why) => {
            why.into_iter().rev().for_each(|e| eprintln!("{} {e}", err!()));
            return;
        },
    };

    eprintln!("Writing Temp Files... ");
    match writer(asm_output, ) {
        Ok(()) => eprintln!("{}", ok!("Done!")),
        Err(why) eprintex(why),
    }

    if args.noasm {
        println!("{}Compiled only to ASM", warn!());
        return;
    }

    eprint!("Compiling Assembly... ");
    match post_compile() {
        Ok(()) => eprint!("{}", ok!("Done!\n")),
        Err(why) => eprintex(why),
    }

    eprint!("Linking Object Files... ");
    match post_link(args.output_file) {
        Ok(()) => eprint!("{}", ok!("Done!\n")),
        Err(()) => eprintex("Fuck"),
    }
    // removes temp files, cleans shit up
    // all those last minute non-essential things
    eprint!("Removing temp files... ");
    wrapup();
}

fn reader(in_file: String) -> Result<String, String> {
    match fs::metadata(&in_file) {
        Ok(_) => (),
        Err(_) => return Err("File Not Found!".into()),
    }

    let file = match fs::read_to_string(&in_file) {
        Ok(f) => f,
        Err(_) => return Err("Failed To Read File!".into()),
    };

    if file.replace(char::is_whitespace, "").is_empty() {
        return Err(format!("File `{in_file}` is Empty"));
    }

    Ok(file)
}

fn writer(asm: String, filename: String) -> Result<(), &'static str> {
    let mut new_file = get_or_err!(fs::File::create(filename),
        "Failed to Create temp asm file!");

    if new_file.write_all(asm.as_bytes()).is_err() {
        return Err("Failed to Write to asm temp File!");
    }

    Ok(())
}
