use ansi_term::Colour::RGB;

pub enum Level {
    Ok,
    Warn,
    Err,
    Debug,
    Info,
}

pub enum At {
    Reader,
    Parser,
    ArgParser,
    Compiler,
    Nasm,
    Ld,
    Writer,
    PreCompiler,
    Wrapup,
    None,
}

pub fn logfmt(line: &usize, filename: &str, msg: &str) -> String {
    format!("{filename}:{line}: {msg}")
}

pub fn logger(lev: Level, at: &At, msg: &str) {
    let dir = match at {
        At::Reader      => format!(" at {}", RGB(255,255,255).bold().paint("READER:")),
        At::Parser      => format!(" at {}", RGB(255,255,255).bold().paint("PARSER:")),
        At::ArgParser   => format!(" at {}", RGB(255,255,255).bold().paint("ARGPARSER:")),
        At::Compiler    => format!(" at {}", RGB(255,255,255).bold().paint("COMPILER:")),
        At::Nasm        => format!(" at {}", RGB(255,255,255).bold().paint("NASM:")),
        At::Ld          => format!(" at {}", RGB(255,255,255).bold().paint("LD:")),
        At::Writer      => format!(" at {}", RGB(255,255,255).bold().paint("WRITER:")),
        At::PreCompiler => format!(" at {}", RGB(255,255,255).bold().paint("PRECOMPILER:")),
        At::Wrapup      => format!(" at {}", RGB(255,255,255).bold().paint("WRAPUP:")),
        At::None        => "".to_string(),
    };

    match lev {
        Level::Ok => println!("{}{dir} {msg}", RGB(0, 153, 51).bold().paint("OK")), 
        Level::Err => println!("{}{dir} {msg}", RGB(179, 0, 0).bold().paint("ERR")), 
        Level::Debug => println!("{}{dir} {msg}", RGB(46, 184, 184).bold().paint("DEBUG")), 
        Level::Warn => println!("{}{dir} {msg}", RGB(230, 230, 0).bold().paint("WARN")), 
        Level::Info => println!("{}{dir} {msg}", RGB(57, 96, 96).bold().paint("INFO")), 
    }
}
