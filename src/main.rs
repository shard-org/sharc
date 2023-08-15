mod args_parser;
mod logger;
mod utils;
mod defs;
mod preprocessor;

use logger::{Level, Debug, logger, ERRORS};
use args_parser::ARGS;

fn main() {
    if ARGS.debug {
        logger(Level::Debug, None, &Debug::ArgParser, format!("{:#?}", *ARGS));
    }

    // Preprocess the file
    let file = preprocessor::process();

    // unsafe caue ERRORS is a mutable static, this will only ever be accessed synchronously so it should be fine
    unsafe {
        if ERRORS > 0 {
            logger(Level::Err, None, &Debug::ArgParser, format!("Could not Compile, {} emmited", ERRORS));
            std::process::exit(1);
        }
    }
    


    todo!();
    // utils::writer(&ARGS.outfile, file);

}
