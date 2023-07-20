use crate::defs::{HELP_MESSAGE, VERSION};
use std::process::exit;

// FIXME: this is a BAD way to do this, ideally we'd have a (&str, &str, u8)
// u8 being the bit packed bools
#[derive(Debug)]
pub struct Args {
    pub temp: bool,
    pub debug: bool,
    pub noasm: bool,
    pub input_file: String,
    pub output_file: Option<String>,
}

pub fn parse_args() -> Result<Args, &'static str> {
    let mut args = std::env::args().skip(1);

    let mut input_file = String::from("");
    let mut output_file: Option<String> = None;
    let mut temp: bool = false;
    let mut debug: bool = false;
    let mut noasm: bool = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{HELP_MESSAGE}");
                exit(1);
            },
            "-v" | "--version" => {
                println!("{VERSION}");
                exit(1);
            },
            "-t" | "--temp" => temp = true,
            "-d" | "--debug" => debug = true,
            "-C" | "--noasm" => noasm = true,
            "-o" | "--output" => {
                if let Some(file) = args.next() {
                    output_file = Some(file);
                } else {
                    return Err("Output File Argument Missing!");
                }
            }
            _ => {
                if !input_file.is_empty() {
                    return Err("Unrecognized Argument!");
                }
                input_file = arg;
            }
        }
    }

    if input_file.is_empty() {
        return Err("Input File Missing!");
    }

    Ok(Args {
        temp,
        debug,
        noasm,
        input_file,
        output_file,
    })
}
