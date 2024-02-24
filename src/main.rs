mod args;
mod logger;
mod utils;

mod location;
mod token;

mod lexer;

mod verbs;
mod macros;
mod project_config;

use logger::{Log, Logs};
use args::Args;
use macros::Macro;
use project_config::Configs;
use verbs::ParsedVerb;
use lexer::Lexer;

use std::process::exit;

lazy_static::lazy_static! {
    // init args
    pub static ref ARGS: Args = Args::parse(std::env::args().skip(1).collect());
}


fn main() {
    let mut logs: Vec<Log> = Vec::new();

    warn!("sharc is still deep in development :p");
    warn!("Please report any bugs, crashes, as well as feature suggestions.");
    print!("\n");

    debug!("{:#?}", *ARGS);



    let main_file = ARGS.file.unwrap_or_else(get_main_file);
    let mut main_file_contents = utils::reader(main_file);

    let mut macros = Macro::parse(&main_file_contents, &mut logs, main_file);
    debug!("{:#?}", macros);
    logs.print();


    let configs = Configs::parse(&main_file_contents, &mut logs, main_file);
    debug!("{:#?}", configs);
    logs.print();

    macros.push(Macro::Def(String::from("NAME"), configs.name));

    if !configs.version.is_empty() {
        macros.push(Macro::Def(String::from("VERSION"), configs.version));
    }


    Macro::apply(macros, &mut main_file_contents);
    debug!("AFTER MACRO:\n{}\n##### END #####", main_file_contents);


    let verbs = ParsedVerb::parse(&main_file_contents, &mut logs, main_file);
    debug!("{:#?}", verbs);
    logs.print();

    // if there's a verb being called
    if let Some(verb) = &ARGS.verb {
        match verbs.iter().find(|v| v.name == verb.verb) {
            Some(verb_def) => {
                verb_def.execute(&verb.args);
                exit(0);
            },
            None => {
                fatal!("Undefined verb `{}`", verb.verb);
                exit(1);
            },
        }
    }

    // if default verb is defined, and we arent bypassing
    if !ARGS.no_default {
        if let Some(verb) = verbs.iter().find(|v| v.name == "_") {
            verb.execute(&[]);
            exit(0);
        }
    }



    let tokens = Lexer::new(&main_file_contents, &mut logs, main_file).lex();
    logs.print();

    let kinds = tokens.iter().fold(Vec::new(), |mut acc, t| {
        acc.push(t.kind.clone()); acc
    });

    debug!("{:?}", kinds);


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
