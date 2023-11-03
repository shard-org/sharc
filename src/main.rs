mod args_parser;
mod cli;
mod compiler;
mod defs;
mod lexer;
mod location;
mod logger;
mod parser;
mod qbe;
mod token;
mod utils;

use args_parser::ARGS;
use lexer::Lexer;
pub use location::Location;
pub use logger::{Level, Log, DEBUG, ERR, FATAL, OK, WARN};
// use defs::TEMP_FILE;

fn main() {
    log!(WARN, "The compiler is still in development, expect FREQUENT bugs, crashes, and missing features.").print();

    // init args
    let args = args_parser::parse();
    log!(DEBUG, "{:#?}", unsafe { &ARGS }).print();

    let main_file = utils::reader(unsafe { &ARGS.infile });

    let token_stream = Lexer::new(main_file, unsafe { ARGS.infile }).lex();
    Log::print_all(); // Exits if errors are found
    for token in &token_stream {
        Log::new(DEBUG, None, "", format!("{}", token)).print();
    }

    // let output = compiler::compiler(token_stream);
    //
    // unsafe{logger::check_err();}

    // log!(DEBUG, "asm output:\n{:?}", &output);
    //
    // if unsafe{ARGS.asm} {
    //     utils::writer(unsafe{ARGS.outfile}, &output);
    //     Log::print_all();
    //     log!(NOW, OK, "Asm output written to `{}`", unsafe{ARGS.outfile});
    //     std::process::exit(0);
    // }
    //
    // log!(FATAL, "assembler not yet implemented");

    Log::print_all();
}
