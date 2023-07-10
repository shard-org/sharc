use crate::{eprintex, printlnex};
use crate::defs::{HELP_MESSAGE, VERSION};

// FIXME: this is a BAD way to dom this, ideally we'd have a (&str, &str, u8)
// u8 being the bit packed bools
pub struct Flags {
    pub temp: bool,
    pub debug: bool,
    pub noasm: bool,
    pub input_file: String,
    pub output_file: Option<String>,
}

pub fn parse_args() -> Flags {
    let mut args = std::env::args().skip(1);
    
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut temp: bool = false;
    let mut debug: bool = false;
    let mut noasm: bool = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => printlnex(HELP_MESSAGE),
            "-v" | "--version" => printlnex(VERSION),
            "-t" | "--temp" => temp = true,
            "-d" | "--debug" => debug = true,
            "-C" | "--noasm" => noasm = true,
            "-o" | "--output" => {
                if let Some(file) = args.next() {
                    output_file = Some(file);
                } else {
                    eprintex("Output File Argument Missing!");
                }
            },
            _ => {
                if input_file.is_some() {
                    eprintex(&format!("Unrecognized Argument `{}`", arg));
                }
                input_file = Some(arg);
            },
        }
    }

    let input_file = match input_file {
        Some(file) => file,
        None => eprintex("No Input File Provided!"),
    };

    Flags {
        temp,
        debug,
        noasm,
        input_file,
        output_file,
    }
}


