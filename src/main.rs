mod args_parser;
mod logger;
mod utils;
mod defs;
mod preprocessor;

use logger::{Level, Debug};

fn main() {
    // Preprocess the file
    let file = preprocessor::process();

    


    todo!();
    // utils::writer(&ARGS.outfile, file);

}
