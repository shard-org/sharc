use crate::utils::*;
use crate::{fmtln, reader};
use std::collections::{VecDeque, HashSet};
use crate::err;

const A: &At = &At::PreCompiler;

// TODO: concat all lines ending with `\` together, for multiline code like in lua
// any other char works, just has to be something not used anywhere else
// pub fn pre_compiler(contents: String, debug: bool, main_file: &str) -> Result<String, ()> {
//     let mut e: bool = false;
//     let a = At::PreCompiler;
//     let mut clean_contents: String = contents.split('\n').filter(|l| !l.trim().starts_with(".use")).collect::<Vec<&str>>().join("\n");
//     clean_contents.insert_str(0, &format!("; @FILENAME {main_file}\n"));
//
//     for (i, ln) in contents.split('\n').filter_map(|l| l.trim().strip_prefix(".use")).rev().enumerate() {
//         let ln = ln.trim();
//
//         if ln.is_empty() {
//             logger(Level::Err, &a, fmtln!(i, "`.use` Directive Missing a Path Argument"));
//             e = true;
//             continue;
//         }
//
//         if debug {
//             logger(Level::Debug, &a, &format!("Path {i}: {ln:?}"));
//         }
//
//         // FIXME: parses the first file fine, but the includes are trated in reference of the
//         // workin dir of the compiler, not the actual file
//         let incl_contents = match reader(ln) {
//             Ok(c) => format!("; @FILENAME {ln}\n {c}\n"),
//             Err(why) => {
//                 logger(Level::Err, &At::Reader, &why);
//                 e = true;
//                 continue;
//             },
//         };
//
//         clean_contents.insert_str(0, &incl_contents);
//     }
//
//     if e { return Err(()); }
//
//     Ok(clean_contents)
// }

macro_rules! hfx {
    ($file:expr, $dir:ident) => {
        format!("{}{}", $dir, $file)
    };
}


pub fn pre_compiler((dir, main_file): (&str, &str), debug: bool) -> Result<Vec<(String, String)>, usize> {
    let e: usize = 0;

    let mut incl = parse_includes(main_file, dir)?;
    if debug {
        incl.iter().for_each(|(f, c)| logger(Level::Debug, A, format!("File: `{f}`\n{c}")));
    }
    todo!();

}

fn parse_includes(filename: &str, dir: &str) -> Result<Vec<(String, String)>, usize> {
    let mut includes: Vec<String> = Vec::new();
    let mut includes_queue: VecDeque<String> = VecDeque::new();
    let mut e: usize = 0;

    // parse main
    let filename = hfx!(filename, dir);
    includes_queue.append(&mut parse_includes_file(&filename)?);
    includes.push(filename);

    while let Some(file) = includes_queue.pop_front() { 
        if includes_queue.contains(&file) { continue; }
        includes_queue.append(&mut parse_includes_file(&hfx!(file, dir))?);
        includes.push(hfx!(file, dir));
    }

    let mut thing: Vec<(String, String)> = Vec::new();
    for inc in includes {
        let contents = match reader(&inc) {
            Ok(c) => c,
            Err(why) => {
                logger(Level::Err, A, why);
                err!(e);
            }
        };

        thing.push((inc.to_string(), contents));
    } 

    if e != 0 { return Err(e); }

    Ok(thing)
} 

fn parse_includes_file(filename: &str) -> Result<VecDeque<String>, usize> {
    let mut includes: VecDeque<String> = VecDeque::new();
    let mut e: usize = 0;

    let contents = match reader(filename) {
        Ok(c) => c,
        Err(e) => {
            logger(Level::Err, A, e);
            return Err(1);
        },
    };

    // split into lines and iterate over
    for (i, ln) in contents.lines().peekable().map(|l| l.trim()).enumerate() {
        // if line starts with `.inc`
        if let Some(file) = ln.strip_prefix(".inc") {
            let file = file.trim();
            if file.is_empty() {
                logger(Level::Err, A, logfmt(&i, filename, "Missing Argument for the Include Directive"));
                err!(e);
            }

            if includes.contains(&file.to_string()) {
                logger(Level::Err, A, logfmt(&i, filename, "Duplicate Include"));
                err!(e);
            }

            includes.push_back(file.to_string());
        }
    }

    if e != 0 { return Err(e); }

    // dbg!(&includes);
    Ok(includes)
}
