use std::process::exit;

mod asm_compile;
mod wrapup;
mod defs;

use crate::wrapup::wrapup;
use crate::asm_compile::*;


fn main() {

    // TODO: add reading project file!, or use input filename
    let out_file = "PLACEHOLDER";

    // TODO: colours for Ok/Err, prob use AnsiTerm
    eprint!("Compiling Assembly... ");
    match post_compile() {
        Ok(()) => eprint!("Done!\n"),
        Err(why) => {
            eprint!("ERR: {}\n", why);
            exit(1);
        },
    }

    eprint!("Linking Object Files... ");
    match post_link(out_file) {
        Ok(()) => eprint!("Done!\n"),
        Err(()) => {
            eprint!("ERR!\n");
            exit(1);
        },
    }
    // removes temp files, cleans shit up
    // all those last minute non-essential things
    eprint!("Removing temp files... ");
    wrapup()
}
