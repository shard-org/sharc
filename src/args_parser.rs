use super::*;

pub const HELP: &str = "shdc - Compiler for the Shard Programming Language
Usage: shdc <input_file> [OPTIONS]

Options:
  -h, --help      This Message
  -V, --version   Version Number

  -o, --output    Specify the Output Binary

  -l, --log={opt} Specify the Log Level {fatal, err, warn, info, debug}
  -q, --quiet     log level = err
  -v, --verbose   log level = info
  -d, --debug     log level = debug

  -a, --arch      Specify the target Architecture

  -t, --noclean   Keep Temp Files
  -A, --asm       Compile to Assembly Only";

pub const VERSION: &str = "onyx 0.1.0";

#[derive(Debug)]
pub struct Args {
    pub infile: Vec<&'static str>,
    pub outfile: &'static str,
    pub asm: bool,
    pub log_level: Level,
    pub noclean: bool,
}

impl Args {
    pub fn default() -> Self {
        Args {
            infile: vec![],
            outfile: "output",
            asm: false,
            log_level: Level::Fatal,
            noclean: false,
        }
    }
}

pub fn parse() -> Args {
    let defaults = Args::default();
    let args = std::env::args();

    match args.nth(0) {
        Some(arg) => ARGS.infile = Box::leak(arg.into_boxed_str()),
        None => log!(FATAL, "Missing input file!").push(),
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
                if let Some((_, level)) = arg.split_once('=') {
                    match level {
                        "none" => ARGS.log_level = Level::None,
                        "err" => ARGS.log_level = Level::Err,
                        "warn" => ARGS.log_level = Level::Warn,
                        "info" => ARGS.log_level = Level::Ok,
                        "debug" => ARGS.log_level = Level::Debug,
                        _ => log!(FATAL, "Invalid Log Level: {}", level).push(),
                    }
                } else {
                    log!(FATAL, "expected `=` after the {} flag", arg).push();
                }
            },
            "--debug" | "-d" => ARGS.log_level = Level::Debug,
            "--quiet" | "-q" => ARGS.log_level = Level::Err,
            "--verbose" | "-v" => ARGS.log_level = Level::Ok,
            "--noclean" | "-t" => ARGS.noclean = true,
            "--asm" | "-A" => ARGS.asm = true,
            "--output" | "-o" => {
                if let Some(outfile) = args.next() {
                    {
                        ARGS.outfile = Box::leak(outfile.into_boxed_str())
                    };
                } else {
                    log!(FATAL, "Missing output file argument after the output flag").push();
                }
            },
            "--arch" | "-a" => {
                if let Some(arch) = args.next() {
                    match arch.as_str() {
                        "x86_64" => todo!(),
                        _ => log!(FATAL, "Invalid Architecture: {}", arch).push(),
                    }
                } else {
                    log!(FATAL, "Missing architecture argument after the arch flag").push();
                }
            },
            arg => log!(FATAL, "Invalid Argument: {}", arg).push(),
        }
    }
}
