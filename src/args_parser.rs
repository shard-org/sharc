use super::*;

pub const HELP: &str = 
"shdc - Compiler for the Shard Programming Language
Usage: shdc <input_file> [OPTIONS]

Options:
  -h, --help      This Message
  -V, --version   Version Number

  -o, --output    Specify the Output Binary

  -l, --log={opt} Specify the Log Level {none, fatal, err, warn, info, debug}
  -q, --quiet     log level = err
  -v, --verbose   log level = info
  -d, --debug     log level = debug

  -a, --arch      Specify the target Architecture

  -t, --noclean   Keep Temp Files
  -A, --asm       Compile to Assembly Only";

pub const VERSION: &str = "onyx 0.1.0";

#[derive(Debug)]
pub struct Args {
    pub infile:  &'static str,
    pub outfile: &'static str,
    pub asm:   bool,
    pub log_level: Level,
    pub noclean: bool,
}

// the actual args
pub static mut ARGS: Args = Args {
    infile:  "",
    outfile: "output",
    asm:   false,
    log_level: Level::Fatal,
    noclean: false,
};

pub fn parse() {
    let mut args = std::env::args().skip(1);

    match args.nth(0) {
        Some(arg) => unsafe{ARGS.infile = Box::leak(arg.into_boxed_str())},
        None => log!(FATAL, "Missing input file!"),
    }

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{}", HELP);
                std::process::exit(0);
            },
            "--version" | "-V" => {
                println!("{}", VERSION);
                std::process::exit(0);
            },
            c if c.starts_with("-l") || c.starts_with("--log") => {
                if let Some((_, lev)) = arg.split_once('=') {
                    match lev.as_str() {
                        "none" => unsafe { ARGS.log_level = Level::None },
                        "fatal" => unsafe { ARGS.log_level = Level::Fatal },
                        "err" => unsafe { ARGS.log_level = Level::Err },
                        "warn" => unsafe { ARGS.log_level = Level::Warn },
                        "info" => unsafe { ARGS.log_level = Level::Info },
                        "debug" => unsafe { ARGS.log_level = Level::Debug },
                        _ => log!(FATAL, "Invalid Log Level: {}", level),
                    }
                } else {
                    log!(FATAL, "expected `=` after the {} flag", arg);
                }
            },
            "--debug" | "-d" => unsafe { ARGS.log_level = Level::Debug },
            "--quiet" | "-q" => unsafe { ARGS.log_level = Level::Err },
            "--verbose" | "-v" => unsafe { ARGS.log_level = Level::Info },
            "--noclean" | "-t" => unsafe { ARGS.noclean = true },
            "--asm" | "-A" => unsafe { ARGS.asm = true },
            "--output" | "-o" => {
                if let Some(outfile) = args.next() {
                    unsafe { ARGS.outfile = Box::leak(outfile.into_boxed_str()) };
                } else {
                    log!(FATAL, "Missing output file argument after the output flag");
                }
            },
            "--arch" | "-a" => {
                if let Some(arch) = args.next() {
                    match arch.as_str() {
                        "x86_64" => todo!(),
                        _ => log!(FATAL, "Invalid Architecture: {}", arch),
                    }
                } else {
                    log!(FATAL, "Missing architecture argument after the arch flag");
                }
            },
            arg => log!(FATAL, "Invalid Argument: {}", arg),
        }
    }
}
