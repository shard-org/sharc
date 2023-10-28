use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "shdc")]
#[command(author, version, help, about = "an assembly inspired sequential programming language")]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Specify the target Architecture
    #[arg(short, long)]
    arch: String,

    /// Keep Temp Files
    #[arg(short, long)]
    noclean: String,

    ///Specify the Log Level {fatal, err, warn, info, debug}
    #[arg(short, long, default_value_t = info)]
    log: LogLevel,

    /// log level = err
    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    /// log level = info
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// log level = debug
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Specify the Output Binary
    #[arg(short, long, default_value_t = false)]
    output: String,
}

enum LogLevel {
    Info,
    Warn,
    Fatal,
    Err,
    Debug,
}

#[derive(Debug, SubCommand)]
enum Commands {
    Compile,
    Test,
    Run,
    Bench,
    QBE,
}

pub fn main_cmd() {
    let macthes = Command::new("shdc")
        .about("an assembly inspired sequential programming language")
        .subcommand_required(true)
        .arg_required_else_help(true)
    ;

}
