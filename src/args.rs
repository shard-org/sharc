use crate::error::{ErrorKind, ErrorLevel};
use crate::{exit, ExitCode};

macro_rules! error {
    ($($ident:tt)*) => {
        ErrorKind::ArgumentParserError
            .new(format!($($ident)*))
            .with_note("(Run sharc with \x1b[1m--help\x1b[0m for usage information)".to_string())
            .display(false);
        exit(ExitCode::Generic);
    };
}

#[derive(Debug)]
pub struct Args {
    pub file: Option<&'static str>,
    pub debug: bool,
    pub code_context: bool,
    pub level: ErrorLevel,
}

impl Args {
    pub fn default() -> Self {
        Args {
            file: None,
            debug: false,
            code_context: true,
            level: ErrorLevel::Warn,
        }
    }

    fn handle_arg(&mut self, arg: &str, args: &mut std::vec::IntoIter<String>, is_end: bool) {
        match arg {
            "h" => {
                println!("{}", USAGE);
                exit(ExitCode::OK);
            }
            "help" => {
                println!("{}\n\n{}", USAGE, HELP_MESSAGE);
                exit(ExitCode::OK);
            }
            "V" | "version" => {
                println!("sharc {}", env!("CARGO_PKG_VERSION"));
                exit(ExitCode::OK);
            }
            "d" | "debug" => {
                self.debug = true;
            }
            "l" | "error-level" => {
                if !is_end {
                    error!("flags with parameters must be at the end of a group, or defined separately");
                };
                let Some(level) = args.next() else {
                    error!("expected level");
                };
                self.level = match level.as_str() {
                    "e" | "error" => ErrorLevel::Error,
                    "w" | "warn" => ErrorLevel::Warn,
                    "n" | "note" => ErrorLevel::Note,
                    "s" | "silent" => ErrorLevel::Silent,
                    _ => {
                        error!("invalid level `{}`", level);
                    }
                };
            }
            "no-context" => {
                self.code_context = false;
            }
            "f" | "file" => {
                if !is_end {
                    error!("flags with parameters must be at the end of a group, or defined separately");
                };
                let Some(file) = args.next() else {
                    error!("expected file");
                };
                if self.file.is_some() {
                    error!("unexpected argument '{}'", file);
                }
                self.file = Some(Box::leak(file.into_boxed_str()))
            }
            _ => {
                error!("unrecognized argument '{}'", arg);
            }
        }
    }

    pub fn parse(args: Vec<String>) -> Self {
        let mut out = Self::default();
        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            if let Some(arg) = arg.strip_prefix("--") {
                out.handle_arg(arg, &mut args, true);
            } else if let Some(arg) = arg.strip_prefix("-") {
                for (i, c) in arg.char_indices().map(|(i, _)| &arg[i..i + 1]).enumerate() {
                    out.handle_arg(c, &mut args, i == arg.len() - 1)
                }
            } else {
                match arg.as_str() {
                    "shark" => {
                        println!("\x1b[34m{}\x1b[0m", SHARK_ASCII);
                        exit(ExitCode::EasterEgg);
                    }
                    "verbs" => {
                        error!("no");
                    }
                    _ => {
                        error!("unrecognized argument '{}'", arg);
                    }
                }
            }
        }
        out
    }
}

const USAGE: &str = "Usage: sharc [-hvd] [-l LEVEL] [-f FILE]";
const HELP_MESSAGE: &str = "\x1b[1mDESCRIPTION\x1b[0m
    The compiler for the Shard Programming Language.
    \x1b[1mTODO:\x1b[0m expand description with --help

\x1b[1mOPTIONS\x1b[0m
    -h, --help                  Show only usage with -h
    -v, --version               Show version
    -d, --debug                 Print debug information
        Shows a ton of information not intended for mere mortals.
    -l, --error-level LEVEL     [error|warn|note|silent]
        (default: warn)
    -f, --file FILE             File to compile
        (default: main.shd)

        --no-context            Disable code context";
const SHARK_ASCII: &str = r#"                                 ,-
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
