mod args_parser;
mod compiler;
mod defs;
mod lexer;
mod literals;
mod logger;
mod parser;
mod utils;
use args_parser::ARGS;
use logger::{at, logger, At, DEBUG, ERR, FATAL, OK, WARN, WTF};


fn main() {
    log!(WARN, "The compiler is still in development, expect FREQUENT bugs, crashes, and missing features.");

    // init args
    args_parser::parse();
    log!(DEBUG, "{:#?}", unsafe { &ARGS });

    let main_file = utils::reader(unsafe { &ARGS.infile });

    let token_stream = parser::parser(main_file);
    
    if token_stream.0.is_empty() {
        log!(WTF, "File not Empty yet token stream has no data ?!??");
    }
    
    unsafe{logger::check_err();}
    
    let output = compiler::compiler(token_stream);
    
    unsafe{logger::check_err();}
    
    
    
    
    log!(DEBUG, "asm output:\n{:?}", &output);
    
    if unsafe{ARGS.asm} {
        utils::writer(unsafe{ARGS.outfile}, &output);
        log!(OK, "Asm output written to `{}`", unsafe{ARGS.outfile});
        std::process::exit(0);
    }
    
    log!(FATAL, "assembler not yet implemented");
    
    unsafe{logger::check_warn();}
}
