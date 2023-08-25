mod args_parser;
mod logger;
mod utils;
mod defs;
mod preprocessor;
mod indexer;

use logger::{Level, Debug, logger};
use args_parser::ARGS;

fn main() {
    if ARGS.debug {
        log!(Level::Debug, &Debug::ArgParser, format!("{:#?}", *ARGS));
    }

    // Preprocess the file
    // TODO actually use this in somethin
    let files = preprocessor::process();

    logger::check_err();

    let index = indexer::indexer(files);
    


    todo!();
    // utils::writer(&ARGS.outfile, file);

}

