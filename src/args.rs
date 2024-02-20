use super::*;
use std::process::exit;
use crate::logger::Level;

const USAGE: &str = "Usage: sharc [-AhV] [-l level] [-f file] [verbs...]";
const HELP_MESSAGE: &str = 
"\x1b[1mDESCRIPTION\x1b[0m
\x1b[1msharc\x1b[0m is a compiler for the Shard Programming Language. 
!!!add more description!!!

\x1b[1mOPTIONS\x1b[0m
    -h  Print this message
    -V  Print the version number

    -A  compile to assembly without outputing a binary

    -f FILE
        File to start compiling from. (default is `main.shd` or `src/main.shd`)

    -l [fatal|err|warn|info|debug]
        Specify the log level for the messages the compiler sends.
        fatal - Irrecoverable error, immidiately exits process
        err   - Regular error
        warn  - A warning, highlighting potential errors
        info  - Generic info thrown by the compiler
        debug - Info for us, the compiler developers. Probably not useful to mere mortals";

#[derive(Debug)]
pub struct Verb {
    pub verb: &'static str, 
    pub args: Vec<&'static str>,
}

#[derive(Debug)]
pub struct Args {
    pub file:    Option<&'static str>,
    pub asm:     bool,
    pub log_lvl: logger::Level,
    pub verb:    Option<Verb>,
}

impl Args {
    fn default() -> Self {
        Args {
            file:    None,
            asm:     false,
            log_lvl: Level::Info,
            verb:    None,
        }
    }

    pub fn parse(args: Vec<String>) -> Self {
        if args.contains(&String::from("--help")) {
            println!("{}\n\n{}", USAGE, HELP_MESSAGE);
            exit(0);
        }

        let mut out = Self::default();
        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            if let Some(arg) = arg.strip_prefix('-') {
                for c in arg.chars() {
                    match c {
                        'h' => {
                            println!("{}\n\n{}", USAGE, HELP_MESSAGE);
                            exit(0);
                        },

                        'V' => {
                            println!("{}", env!("CARGO_PKG_VERSION"));
                            exit(0);
                        },

                        /* log level */
                        'l' => out.log_lvl = match args.next() {
                            Some(a) => match a.as_str() {
                                "e" | "err"   => Level::Err,
                                "w" | "warn"  => Level::Warn,
                                "i" | "info"  => Level::Info,
                                "d" | "debug" => Level::Debug,
                                _ => { 
                                    fatal!("Invalid Log Level: {}", a);
                                    exit(1);
                                },
                            },
                            None => {
                                fatal!("Missing argument after `-l`");
                                exit(1);
                            },
                        },

                        /* file */
                        'f' => out.file = match args.next() {
                            Some(f) => Some(Box::leak(f.into_boxed_str())),
                            None => {
                                fatal!("Missing argument after `-f`");
                                exit(1);
                            },
                        },

                        a => {
                            fatal!("Unknown arg `{}`\n{}", a, USAGE);
                            exit(1);
                        },
                    }
                } 
                continue;
            }

            if arg == "shark" {
                println!("\x1b[34m{}\x1b[0m", SHARK_ASCII);
                exit(1);
            }

            out.verb = Some(Verb {
                verb: Box::leak(arg.into_boxed_str()), 
                args: args.clone().fold(Vec::new(), |mut acc, a| {
                    acc.push(Box::leak(a.into_boxed_str()));
                    acc
                }),
            });
            break;
        } 
        out
    }
}


const SHARK_ASCII: &str = 
r#"                                 ,-
                               ,'::|
                              /::::|
                            ,'::::o\                                      _..
         ____........-------,..::?88b                                  ,-' /
 _.--"""". . . .      .   .  .  .  ""`-._                           ,-' .;'
<. - :::::o......  ...   . . .. . .  .  .""--._                  ,-'. .;'
 `-._  ` `":`:`:`::||||:::::::::::::::::.:. .  ""--._ ,'|     ,-'.  .;'
     """_=--       //'doo.. ````:`:`::::::::::.:.:.:. .`-`._-'.   .;'
         ""--.__     P(       \               ` ``:`:``:::: .   .;'
                "\""--.:-.     `.                             .:/
                  \. /    `-._   `.""-----.,-..::(--"".\""`.  `:\
                   `P         `-._ \          `-:\          `. `:\
                                   ""            "            `-._)"#;
