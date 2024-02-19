mod args;
mod logger;
mod utils;
mod location;
mod verbs;
mod macros;

use logger::{Log, Logs};
use location::Span;
use args::Args;
use macros::Macro;


lazy_static::lazy_static! {
    // init args
    pub static ref ARGS: Args = Args::parse(std::env::args().skip(1).collect());
}


fn main() {
    let mut logs: Vec<Log> = Vec::new();

    warn!("sharc is still deep in development :p");
    warn!("Please report any bugs, crashes, as well as feature suggestions.");

    debug!("{:#?}", *ARGS);



    let main_file = ARGS.file.unwrap_or_else(get_main_file);
    let mut main_file_contents = utils::reader(main_file);

    let macros = Macro::parse(&main_file_contents, &mut logs, main_file);
    debug!("{:#?}", macros);

    Macro::apply(macros, &mut main_file_contents);

    logs.print();

    todo!();


    // let token_stream = Lexer::new(main_file, unsafe{ARGS.infile}).lex();
    // Log::print_all();  // Exits if errors are found
    // for token in &token_stream {
    //     Log::new(DEBUG, None, "", format!("{}", token)).print();
    // }

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

    logs.print();
    logs.summary();
}

fn get_main_file() -> &'static str {
    const MAIN_FILE_PATHS: [&str; 2] = ["main.shd", "src/main.shd"];

    for file in MAIN_FILE_PATHS {
        if std::fs::metadata(file).is_ok() {
            return file;
        }
    }

    fatal!("Could not find a main file. Use the `-f` flag, or make sure it's within one of these paths: {:?}", MAIN_FILE_PATHS); 
    std::process::exit(1)
}
