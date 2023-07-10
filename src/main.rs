mod asm_compile;
mod wrapup;
mod defs;

use crate::wrapup::wrapup;


fn main() {

    // removes temp files, cleans shit up
    // all those last minute non-essential things
    wrapup()
}
