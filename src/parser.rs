use super::*;

use std::path::{PathBuf, Path};

pub enum Size {
    Byte = 1,
    Word = 2,
    DWord = 4,
    QWord = 8,
}

pub type Name = String;

pub enum Token {
    // Directives
    DDefine(Name, String),
    DInclude(PathBuf),
    DTxt(String),
    DEntry(Name),


    // Labels
    Label(Name),
    Func(Name, Vec<(Name, Size)>),

    StaticVar(Name, Size),
    StackVar(Name, Size),
    RegVar(Name, Register),

    Call(Name, Option<Vec<Arg>>),
    ExtCall(Name, Option<Vec<Arg>>),
    Ret(Arg),

}

pub enum Arg {
    Var(Name),
    Reg(Register),
    Lit(u64),
    Str(String),
}

pub struct FatToken {
    pub token: Token,
    pub at: At,
}

pub struct Metadata {
    pub entry: Option<Name>,
}

pub type Register = u8;

macro_rules! add {
    ($stream:ident, $at:expr, $tok:expr) => {
        $stream.push(FatToken {
            token: $tok,
            at: $at.clone(),
        })
    };
}

pub fn parser(input: String) {
    let mut lines = input.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();

    let mut file_stack: Vec<(usize, PathBuf, usize)> = Vec::new();
    file_stack.push((0, PathBuf::from(unsafe{ARGS.infile}), lines.len()));

    let parent_dir: Box<Path> = file_stack.last().unwrap().1.parent().unwrap().into();

    let mut t = Vec::new(); // token stream
    let mut meta = Metadata {
        entry: None,
    };

    while let Some(line) = lines.pop() {
        let last = file_stack.last_mut().unwrap();
        last.0 += 1;

        if last.0 == last.2 {
            file_stack.pop();
        }

        let file = file_stack.last().unwrap();
        let at = at(file.0, &file.1);

        //
        // Comments
        if line.starts_with("//") {
            continue;
        }
        
        //
        // Directives
        if let Some(line) = line.strip_prefix('.') {
            let Some((dir, arg)) = line.split_once(' ') else {
                log_at!(ERR, at, "Missing Arg");
                continue;
            };

            match dir {
                // Define
                "def" | "define" => {
                    let Some((name, value)) = arg.split_once(' ') else {
                        log_at!(ERR, at, "Missing Arg");
                        continue;
                    };

                    add!(t, at, Token::DDefine(name.to_string(), value.to_string()));
                },

                // Entry
                "ent" | "entry" => {
                    if meta.entry.is_some() {
                        log_at!(ERR, at, "Entry already defined");
                        continue;
                    }

                    meta.entry = Some(arg.to_string());
                },

                // Include
                "inc" | "include" => {
                    let path = parent_dir.join(arg);
                    let mut file = utils::reader(&path.display().to_string())
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty())
                        .map(String::from)
                        .collect::<Vec<String>>();

                    file_stack.push((0, path, file.len()));
                    file.extend(lines);
                    lines = file;

                    continue;
                },

                // Text
                "txt" | "text" 
                    => add!(t, at, Token::DTxt(arg.to_string())),

                // other
                d => log_at!(ERR, at, "Unknown Directive {}", d),
            }
            continue;
        }

        //
        // Labels
        if let Some(line) = line.strip_prefix('@') {
            let args = line.split_whitespace().collect::<Vec<&str>>();
            let Some(name) = args.first() else {
                log_at!(ERR, at, "Missing Name");
                continue;
            };

            if args.len() == 1 {
                add!(t, at, Token::Label(name.to_string()));
                continue;
            }
            
            todo!("parse label args");
        }

        //
        // Calls
        if let Some(line) = line.strip_prefix('!') {

            match line.find(' ') {
                Some(i) => {
                    let (name, args) = line.split_at(i);
                    let args = args.split(',')
                        .map(str::trim)
                        .collect::<Vec<String>>();

                    add!(t, at, Token::Call(name.to_string(), args));
                },
                None => {
                    if line == "." {
                        log_at!(ERR, at, "No function name in call");
                        continue;
                    }

                    add!(t, at, Token::Call(line.to_string(), None));
                    continue;
                },
            }
        }

    }
}

fn parse_args(args: Vec<&str>) -> Vec<Arg> {
    let mut ret = Vec::new();
    for arg in args {
        // register
        if arg.starts_with('r') {
            let reg = arg[1..].parse::<u8>().unwrap();

            // TODO this assumes x86_64
            if reg > 15 {
                log_at!(ERR, at, "Invalid Register r{}", reg);
                continue;
            }

            ret.push(Arg::Reg(reg));
            continue;
        }

        // string
        if let Some(arg) = arg.strip_prefix('"') {
            if !arg.ends_with('"') {
                log_at!(ERR, at, "Invalid String, Missing closing `\"`");
                continue;
            }

            ret.push(Arg::Str(arg[..arg.len()-1].to_string()));
            continue;
        }

        // decimal literal
        if let Ok(lit) = arg.parse::<u64>() {
            ret.push(Arg::Lit(lit));
            continue;
        }

        // hex literal
        if let Some(arg) = arg.strip_prefix("0x") {
            if let Ok(lit) = u64::from_str_radix(arg, 16) {
                ret.push(Arg::Lit(lit));
                continue;
            }
        }

        // binary literal
        if let Some(arg) = arg.strip_prefix("b") {
            if let Ok(lit) = u64::from_str_radix(arg, 2) {
                ret.push(Arg::Lit(lit));
                continue;
            }
        }
    }
}

fn validate_name(at: At, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    if name.contains(|c: char| !c.is_alphabetic() || c != '_') {
        log_at!(ERR, at, "Name may only contain letters and underscores");
        return false;
    }

    if name.starts_with('r') {
        log_at!(ERR, at, "Name may not start with `r`, this is reserved for registers");
        return false;
    }

    true
}
