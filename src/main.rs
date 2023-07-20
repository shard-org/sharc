use std::process::exit;
use std::fs;
use std::io::Write;

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
use crate::utils::{logger, Level, At};
use crate::wrapup::wrapup;
use crate::compiler::compiler;

fn main() {
    // returns a Flags struct, see arg_parser file
    let args = parse_args().unwrap_or_else(|e| {
        logger(Level::Err, &At::ArgParser, e);
        exit(1);
    });

    if args.debug {
        logger(Level::Debug, &At::ArgParser, &format!("args:?"));
    }

    eprint!("Reading File... ");
    let in_file_cont = match reader(args.input_file) {
        Ok(stuff) => {
            logger(Level::Ok, &At::Reader, "Done!");
            stuff
        },
        Err(why) => {
            logger(Level::Err, &At::Reader, &why);
            exit(1);
        }
    };

    // TODO: make this run for every file
    // "Parsing {filename}..."
    eprintln!("Parsing Code... ");
    let tokens = match parser(in_file_cont, args.debug) {
        Ok(parsed) => {
            logger(Level::Ok, &At::Parser, "Done!");
            parsed
        },
        Err(_) => exit(1),
    };

    // TODO: make this run for every file
    eprintln!("Compiling... ");
    let asm_output = match compiler(tokens, args.debug) {
        Ok(out) => {
            logger(Level::Ok, &At::Compiler, "Done!");
            out
        },
        Err(()) => exit(1),
    };

    // TODO: have this write per file
    // FIXME: "temp.asm" is a placeholder
    eprintln!("Writing Temp Files... ");
    match writer(asm_output, "temp.asm".to_string()) {
        Ok(()) => logger(Level::Ok, &At::Writer, "Done!"),
        Err(why) => {
            logger(Level::Err, &At::Writer, why);
            exit(1);
        },
    }

    if args.noasm {
        logger(Level::Warn, &At::Writer, "Compiled only to ASM");
        exit(1);
    }

    eprint!("Compiling Assembly... ");
    match post_compile() {
        Ok(()) => logger(Level::Ok, &At::Nasm, "Done!"),
        Err(why) => {
            logger(Level::Err, &At::Nasm, why);
            exit(1);
        },
    }

    eprint!("Linking Object Files... ");
    match post_link(args.output_file) {
        Ok(()) => logger(Level::Ok, &At::Ld, "Done!"),
        Err(()) => {
            logger(Level::Err, &At::Ld, "Shit Went Down!");
            exit(1);
        },
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
    let mut new_file = match fs::File::create(filename) {
        Ok(n) => n,
        Err(_) => return Err("Failed to Create temp asm file!"),
    };

    if new_file.write_all(asm.as_bytes()).is_err() {
        return Err("Failed to Write to asm temp File!");
    }

    Ok(())
}
