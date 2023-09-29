mod args_parser;
mod logger;
mod utils;
mod defs;
// mod parser;
// mod compiler;
mod location;
mod token;
mod lexer;


pub use logger::{Log, Level, WARN, DEBUG, OK, ERR, FATAL};
pub use location::Location;
use lexer::Lexer;
use args_parser::ARGS;
// use defs::TEMP_FILE;

fn main() {
    log!(WARN, "The compiler is still in development, expect FREQUENT bugs, crashes, and missing features.").print();

    // init args
    args_parser::parse();
    log!(DEBUG, "{:#?}", unsafe{&ARGS}).print();

    let main_file = utils::reader(unsafe{&ARGS.infile});


    let token_stream = Lexer::new(main_file, unsafe{ARGS.infile}).lex();
    for token in &token_stream {
        Log::new(DEBUG, None, "", format!("{}", token)).print();
    }

    Log::print_all_checked();

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

