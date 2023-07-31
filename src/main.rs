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
mod pre_compiler;

use crate::args_parser::*;
use crate::asm_compile::*;
use crate::parser::parser;
use crate::utils::{logger, Level, At, get_tip, reader};
use crate::wrapup::wrapup;
use crate::compiler::compiler;
use crate::pre_compiler::pre_compiler;

// =================================
// TODO: We'll prob want this kinda structure
// Pre-compiler (check includes, macros, defs, etc)
// Parser (parse it into tokens, checking for syntax)
// Mapper (create hash maps of all the subroputine defs, markers, etc), check if none repeat
// Compiler (generate the asm, most other errors)
// Post-Compiler (generate machine code) (handled by nasm)
// =================================


fn main() {
    let warns: usize = 0;

    // returns a Flags struct, see arg_parser file
    let args = parse_args().unwrap_or_else(|e| {
        logger(Level::Err, &At::ArgParser, e);
        exit_err();
    });

    if args.debug {
        logger(Level::Debug, &At::ArgParser, format!("{args:?}"));
    }


    fn find_path(file: &str) -> Option<(String, String)> {

        None
    }

    let file = &args.input_file;
    let mut file_and_dir: (&str, &str) = ("", file);
    if !file.ends_with('/') { 
        if let Some(i) = file.rfind('/') {
            file_and_dir = file.split_at(i + 1);
        }
    }

    // combines all the include files into one String
    let preparsed_file_cont = match pre_compiler(file_and_dir, args.debug) {
        Ok(cont) => {
            logger(Level::Ok, &At::PreCompiler, "");
            cont
        },
        Err(e) => {
            logger(Level::Info, &At::Parser, format!("Could not Compile `{}`; {e} errors emmited", args.input_file));
            exit_err();
        },
    };

    // converts the file String into tokens Vec<Data>
    let tokens = match parser(preparsed_file_cont, args.debug) {
        Ok(parsed) => {
            logger(Level::Ok, &At::Parser, "");
            parsed
        },
        Err(e) => {
            logger(Level::Info, &At::Parser, format!("Could not Compile `{}`; {e} errors emmited", args.input_file));
            exit_err();
        },
    };

    // compiles the tokens into asm 
    let asm_output = match compiler(tokens, args.debug) {
        Ok(out) => {
            logger(Level::Ok, &At::Compiler, "Done!");
            out
        },
        Err(e) => {
            logger(Level::Info, &At::Parser, format!("Could not Compile `{}`; {e} errors emmited", args.input_file));
            exit_err();
        },
    };

    // writes the asm 
    match writer(asm_output, "temp.asm") {
        Ok(()) => logger(Level::Ok, &At::Writer, "Done!"),
        Err(why) => {
            logger(Level::Err, &At::Writer, why);
            exit_err();
        },
    }

    if args.noasm {
        logger(Level::Warn, &At::Writer, "Compiled only to ASM");
        exit_err();
    }

    // compiles asm into object files (using `nasm`)
    match post_compiler() {
        Ok(()) => logger(Level::Ok, &At::Nasm, "Done!"),
        Err(why) => {
            logger(Level::Err, &At::Nasm, why);
            exit_err();
        },
    }

    // object files into an executable binary
    match linker(args.output_file) {
        Ok(()) => logger(Level::Ok, &At::Ld, "Done!"),
        Err(()) => {
            logger(Level::Err, &At::Ld, "Shit Went Down!");
            exit_err();
        },
    }
    // removes temp files, cleans shit up
    // all those last minute non-essential things
    eprint!("Removing temp files... ");
    wrapup();

    if warns != 0 {
        logger(Level::Info, &At::None, format!("Compiled `{}`; {warns} warnings emmited", args.input_file));
        exit(0);
    }

    logger(Level::Info, &At::None, format!("Successfully Compiled `{}`", args.input_file));
}

// only for writing the temp asm file
pub fn writer(asm: String, filename: &str) -> Result<(), &'static str> {
    let mut new_file = match fs::File::create(filename) {
        Ok(n) => n,
        Err(_) => return Err("Failed to Create temp asm file!"),
    };

    if new_file.write_all(asm.as_bytes()).is_err() {
        return Err("Failed to Write to asm temp File!");
    }

    Ok(())
}

fn exit_err() -> ! {
    logger(Level::Tip, &At::None, get_tip());
    exit(1)
} 
