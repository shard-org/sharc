mod args_parser;
mod logger;
mod utils;
mod defs;
mod parser;
mod compiler;
mod location;
mod token;
mod lexer;

use logger::{logger, DEBUG, OK, WARN, ERR, FATAL, at, At, WTF};
use lexer::Lexer;
use args_parser::ARGS;
use defs::TEMP_FILE;

fn main() {
    log!(WARN, "The compiler is still in development, expect FREQUENT bugs, crashes, and missing features.");

    // init args
    args_parser::parse();
    log!(DEBUG, "{:#?}", unsafe{&ARGS});

    let main_file = utils::reader(unsafe{&ARGS.infile});


    let token_stream = Lexer::new(main_file, unsafe{ARGS.infile}).lex();
    for token in &token_stream {
        log!(DEBUG, "{}", token);
    }

    //
    //
    // if token_stream.0.is_empty() {
    //     log!(WTF, "File not Empty yet token stream has no data ?!??");
    // }
    //
    // unsafe{logger::check_err();}
    //
    // let output = compiler::compiler(token_stream);
    //
    // unsafe{logger::check_err();}
    //
    //
    //
    //
    // log!(DEBUG, "asm output:\n{:?}", &output);
    //
    // if unsafe{ARGS.asm} {
    //     utils::writer(unsafe{ARGS.outfile}, &output);
    //     log!(OK, "Asm output written to `{}`", unsafe{ARGS.outfile});
    //     std::process::exit(0);
    // }
    //
    // log!(FATAL, "assembler not yet implemented");
    //
    // unsafe{logger::check_warn();}
}

