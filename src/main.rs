mod args;
mod logger;
mod utils;

mod location;
mod token;

mod lexer;

use logger::{Log, Logs};
use args::Args;
use lexer::Lexer;

lazy_static::lazy_static! {
    // init args
    pub static ref ARGS: Args = Args::parse(std::env::args().skip(1).collect());
}


fn main() {
    // let mut logs: Vec<Log> = Vec::new();

    warn!("sharc is still deep in development :p");
    warn!("Please report any bugs, crashes, as well as feature suggestions.");
    print!("\n");

    debug!("{:#?}", *ARGS);



    let main_file_name = ARGS.file.unwrap_or_else(get_main_file);
    let main_file = utils::open(main_file_name);


    let tokens = Lexer::new(main_file, main_file_name);
    // logs.print();

    let file = std::fs::File::open(main_file_name).unwrap();
    let lexer = Lexer::new(file, main_file_name);

    for tok in lexer {
        match tok.kind {
            crate::token::TokenKind::Err(e) => e.print(),
            k => println!("{:?}", k),
        }
    }

    // let kinds = tokens.iter().fold(Vec::new(), |mut acc, t| {
    //     acc.push(t.kind.clone()); acc
    // });
    //
    // debug!("{:?}", kinds);


    todo!()


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
