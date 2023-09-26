use super::*;

use std::path::{PathBuf, Path};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Size {
    Byte = 1,
    Word = 2,
    DWord = 4,
    QWord = 8,
}

#[derive(Debug)]
pub enum RegSize {
    ByteHigh,
    ByteLow,
    Word,
    DWord,
    QWord,
    Arch,  // architecure dependent
}

pub type Name = String;

#[derive(Debug)]
pub enum Token {
    // Directives
    DDefine(Name, String),
    DTxt(String),

    // Labels
    Label(Name),
    Func(Name, Vec<(Name, Size)>),
    Jump(Name),

    StaticVar(Name, Size),
    StackVar(Name, Size, ),
    RegVar(Name, u8),

    Call(Name, Option<Vec<Arg>>),
    ExtCall(Name, Option<Vec<Arg>>),
    Ret(Option<Arg>),

    MutVar(Name, Arg),
    MutReg(u8, RegSize, Arg), 
    Null,
}

#[derive(Debug)]
pub enum Arg {
    Var(Name),
    Reg(u8, RegSize),
    Lit(u64),
    Str(String),
    Call(Name, Option<Vec<Arg>>),  // TODO idk if we want inline calls, see how difficult that's to parse/compile
    // Expr(Vec<MLSymbol>),  TODO laterrrrrrrrr
    None,
}

pub enum MutMethod {
    Set,      // 'foo = 20 
    Add,      // 'foo + 20 
    Sub,      // 'foo - 20 
    Xor,      // 'foo ^ bar
    And,      // 'foo & bar
    Or,       // 'foo | bar
    SetDeref, // 'foo : bar
    ShiftR,   // 'foo > 20 
    ShiftL,   // 'foo < 20 
    Not,      // 'foo ~ bar
    Inc,      // 'foo ++
    Dec,      // 'foo --
}
// pub enum MLSymbol {
//     Add,
//     Sub,
//     Mul,
//     Div,
//     Mod,
//     And,
//     Or,
//     Xor,
//     Not,
//     Lit(u64),
//     Var(Name),
//     Reg(u8, RegSize),
// }

pub type FatToken = (Token, At);

#[derive(Debug)]
pub struct Metadata {
    pub entry: Option<Name>,
}

macro_rules! add {
    ($stream:ident, $at:expr, $tok:expr) => {
        $stream.push(($tok, $at.clone()))
    };
}

pub fn parser(input: String) -> (Vec<FatToken>, Metadata) {
    let mut lines = input.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect::<VecDeque<String>>();

    let mut file_stack: Vec<(usize, PathBuf, usize)> = Vec::new();
    file_stack.push((0, PathBuf::from(unsafe{ARGS.infile}), lines.len()));

    let parent_dir: Box<Path> = file_stack.last().unwrap().1.parent().unwrap().into();

    let mut t = Vec::new(); // token stream
    let mut meta = Metadata {
        entry: None,
    };

    while let Some(line) = lines.pop_front() {
        let last = file_stack.last_mut().unwrap();
        last.0 += 1;

        if last.0 > last.2 {
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
                        .collect::<VecDeque<String>>();

                    file_stack.push((0, path, file.len()));
                    lines.append(&mut file);
                },

                ".con" | ".const" => {
                    log_at!(FATAL, at, "constants not yet implemented");
                },

                // other
                d => log_at!(ERR, at, "Unknown Directive {}", d),
            }
        }

        //
        // Labels
        else if let Some(line) = line.strip_prefix('@') {
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
        else if let Some(line) = line.strip_prefix('!') {
            if line.is_empty() {
                log_at!(ERR, at, "Missing Function Name");
                continue;
            }

            match line.find(' ') {
                Some(i) => {
                    let (name, args) = line.split_at(i);
                    let args = args.split(',')
                        .map(str::trim)
                        .map(|a| parse_arg(&at, a))
                        .collect::<Vec<Arg>>();

                    if args.iter().any(|a| matches!(a, Arg::None)) {
                        continue;
                    }

                    // check for external call
                    if let Some(name) = name.strip_prefix('$') {
                        if validate_name(&at, name) {
                            add!(t, at, Token::ExtCall(name.to_string(), Some(args)));
                        }
                        continue;
                    }

                    add!(t, at, Token::Call(name.to_string(), Some(args)));
                },
                None => {

                    add!(t, at, Token::Call(line.to_string(), None));
                },
            }
        }

        //
        // Returns
        else if let Some(line) = line.strip_prefix("ret").and_then(|l| Some(l.trim_start())) {
            let arg = match parse_arg(&at, line) {
                Arg::None => None,
                a => Some(a),
            };

            add!(t, at, Token::Ret(arg));
        }

        //
        // TODO: Mutations
        else if let Some(line) = line.strip_prefix('\'') {
            log_at!(FATAL, at, "mutations not yet implemented");
        }

        //
        // Statics
        else if let Some(line) = line.strip_prefix('/') {
            log_at!(FATAL, at, "statics not yet implemented");
        }

        else if let Some(line) = line.strip_prefix('#') {
            parse_jump(&at, line).map(|tok| add!(t, at, tok));
        }

        // todo!();

    }
    log!(DEBUG, "{:?}", &meta);
    log!(DEBUG, "{:?}", &t);
    (t, meta)
}

// expects no # prefix
fn parse_jump(at: &At, arg: &str) -> Option<Token> {
    if arg.is_empty() {
        log_at!(ERR, at.clone(), "Missing Jump Adress");
        return None;
    }

    if !validate_name(&at, arg) {
        return None;
    }

    Some(Token::Jump(arg.to_string()))
}

fn parse_arg(at: &At, arg: &str) -> Arg {
    // register
    if let Some(arg) = arg.strip_prefix('r') {
        if arg.is_empty() {
            log_at!(ERR, at.clone(), "No register specified after `r`");
            return Arg::None;
        }

        let size = match arg.chars().last().unwrap() {
            'h' => RegSize::ByteHigh,
            'l' => RegSize::ByteLow,
            'w' => RegSize::Word,
            'd' => RegSize::DWord,
            'q' => RegSize::QWord,
            _   => RegSize::Arch,
        };

        let num = match size {
            RegSize::Arch => arg,
            _ => &arg[..arg.len()-1],
        };

        match num.parse::<u8>() {
            Ok(reg) => return Arg::Reg(reg, size),
            Err(e) => {
                log_at!(ERR, at.clone(), "Invalid Register: {}", e);
                return Arg::None;
            },
        }
    }

    // string
    if let Some(arg) = arg.strip_prefix('"') {
        if !arg.ends_with('"') {
            log_at!(ERR, at.clone(), "Invalid String, Missing closing `\"`");
            return Arg::None;
        }

        return Arg::Str(arg[..arg.len()-1].to_string());
    }

    // decimal literal
    if let Ok(lit) = arg.parse::<u64>() {
        return Arg::Lit(lit);
    }

    // hex literal
    if let Some(arg) = arg.strip_prefix("0x") {
        if let Ok(lit) = u64::from_str_radix(arg, 16) {
            return Arg::Lit(lit);
        }
    }

    // binary literal
    if let Some(arg) = arg.strip_prefix("b") {
        if let Ok(lit) = u64::from_str_radix(arg, 2) {
            return Arg::Lit(lit);
        }
    }

    // TODO func call, this is gonna be pain
    
    // variable
    if validate_name(&at, arg) {
        return Arg::Var(arg.to_string());
    } else { return Arg::None; }
}

fn validate_name(at: &At, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    if name.contains(|c: char| !c.is_alphabetic() || c != '_') {
        log_at!(ERR, at.clone(), "Name may only contain letters and underscores");
        return false;
    }

    if name.starts_with('r') {
        log_at!(ERR, at.clone(), "Name may not start with `r`, this is reserved for registers");
        return false;
    }

    true
}
