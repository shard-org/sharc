mod args_parser;
mod logger;
mod utils;
mod defs;
mod parser;

use logger::{Level, logger, DEBUG, OK, WARN, ERR, FATAL, at, At};
use args_parser::ARGS;

fn main() {
    // init args
    args_parser::parse();
    log!(DEBUG, "{:#?}", unsafe{&ARGS});

    let main_file = utils::reader(unsafe{&ARGS.infile});




    todo!();
    // utils::writer(&ARGS.outfile, file);

}

