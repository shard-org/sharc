use std::process::exit;
use crate::logger::{logger, Level, Debug};
use once_cell::sync::Lazy;
use crate::defs::{VERSION, HELP};

#[derive(Debug)]
pub struct Args {
    pub infile:  String,
    pub outfile: String,
    pub nobin:   bool,
    pub debug:   bool,
    pub noclean: bool,
}

// the actual args
pub static ARGS: Lazy<Args> = Lazy::new(parse);

const DBG: &Debug = &Debug::ArgParser;

// parse em!
fn parse() -> Args {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    // Check for help
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{}", HELP);
        exit(0);
    }

    // Check for version
    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("{}", VERSION);
        exit(0);
    }


    // Parse the args
    let mut parsed = Args {
        infile:  args.get(0).unwrap_or_else(|| {
            logger(Level::Err, None, DBG, "No input file specified");
            exit(1);
        }).to_string(),
        outfile: String::from("output"),
        debug:   args.iter().any(|a| a == "--debug" || a == "-d"),
        noclean: args.iter().any(|a| a == "--noclean" || a == "-t"),
        nobin:   args.iter().any(|a| a == "--nobin" || a == "-C"),
    };

    // Check for output file
    if let Some(index) = args.iter().position(|a| a == "--output" || a == "-o") {
        // check if its provided
        if let Some(out) = args.get(index + 1) {
            parsed.outfile = out.to_string();
            return parsed;
        }

        // if not, error
        logger(Level::Err, None, DBG, "No output file specified");
        exit(1);
    }

    dbg!();
    if parsed.debug {
        logger(Level::Debug, None, DBG, format!("{:#?}", parsed));
    }

    parsed
}
