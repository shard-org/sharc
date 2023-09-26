use super::*;

pub const HELP: &str = 
"shdc - Compiler for the Shard Programming Language
Usage: shdc <input_file> [OPTIONS]

Options:
  -h, --help      This Message
  -v, --version   Version Number

  -o, --output    Specify the Output Binary
  -d, --debug     Not needed for Mere Mortals :v
  -q, --quiet     Print only Fatal Errors
  -a, --arch      Specify the target Architecture

  -t, --noclean   Keep Temp Files
  -A, --asm       Compile to Assembly Only";

pub const VERSION: &str = "onyx 0.1.0";

#[derive(Debug)]
pub struct Args {
    pub infile:  &'static str,
    pub outfile: &'static str,
    pub asm:   bool,
    pub debug:   bool,
    pub quiet:   bool,
    pub noclean: bool,
}

// the actual args
pub static mut ARGS: Args = Args {
    infile:  "",
    outfile: "output",
    asm:   false,
    debug:   false,
    quiet:   false,
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
            "--version" | "-v" => {
                println!("{}", VERSION);
                std::process::exit(0);
            },
            "--debug" | "-d" => unsafe { ARGS.debug = true },
            "--noclean" | "-t" => unsafe { ARGS.noclean = true },
            "--asm" | "-A" => unsafe { ARGS.asm = true },
            "--quiet" | "-q" => unsafe { ARGS.quiet = true },
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
