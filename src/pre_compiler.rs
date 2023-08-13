use crate::utils::*;
use crate::reader;
use std::collections::VecDeque;
use crate::err;

const A: &At = &At::PreCompiler;

trait StringReplaceFirst {
    fn replace_first(&self, from: &str, to: &str) -> String;
}

impl StringReplaceFirst for str {
    fn replace_first(&self, from: &str, to: &str) -> String {
        if let Some(index) = self.find(from) {
            let (before, after) = self.split_at(index);
            let replaced = [before, to, &after[from.len()..]].concat();
            return replaced;
        } 

        self.to_string()
    }
}

trait Concat<'a>: Iterator<Item=&'a str> {
    fn parse_backticks(self) -> String;
}

impl<'a, I> Concat<'a> for I where I: Iterator<Item=&'a str> {
    fn parse_backticks(self) -> String {
        let mut result: Vec<String> = Vec::new();

        for ln in self {
            if ln.starts_with('`') {
                let i = result.len() - 1;
                if let Some(e) = result.get_mut(i) {
                    *e = format!("{e}{}", ln.replace_first("`", " "))
                }
                continue;
            }
            result.push(ln.to_string());
        }

        result.into_iter()
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn pre_compiler(dir: &str, main_file: &str, debug: bool) -> Result<String, usize> {
    let mut file_concat = String::new();
    let mut incl = parse_includes(main_file, dir)?;

    if debug {
        incl.iter().for_each(|(f, c)| logger(Level::Debug, A, format!("File: `{f}`\n{c}")));
    }

    file_concat.push_str(&incl.remove(0).1);

    for (name, content) in incl {
        let include_string = &format!(".inc {}", name.strip_prefix(dir).unwrap());
        let include_content = &format!("~FILESTART {name}\n {}\n ~FILEEND", content.trim_end_matches('\n'));

        file_concat = file_concat.replace_first(include_string, include_content);
    }

    file_concat.lines()
        .map(|l| l.trim())
        .filter(|l| !(l.starts_with(".inc") || l.starts_with("//") || l.is_empty()))
        .map(|l| {
            if let Some(i) = l.find("//") {
                return &l[..i];
            } l
        })
        .parse_backticks();


    if debug {
        logger(Level::Debug, A, format!("Final String:\n{}\n", &file_concat));
    }

    Ok(file_concat)
}

fn parse_includes(filename: &str, dir: &str) -> Result<Vec<(String, String)>, usize> {
    let mut includes: Vec<String> = Vec::new();
    let mut includes_queue: VecDeque<String> = VecDeque::new();
    let mut e: usize = 0;

    // parse main
    includes_queue.append(&mut parse_includes_file(filename)?);
    includes.push(filename.to_string());

    while let Some(file) = includes_queue.pop_front() { 
        if includes_queue.contains(&file) { continue; }

        let file = format!("{}{}", dir, file);
        includes_queue.append(&mut parse_includes_file(&file)?);
        includes.push(file);
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

            if ln.find("~~").is_some() {
                logger(Level::Err, A, logfmt(&i, filename, "Usage of `~~` not allowed, as it is an Internal Marker"));
                err!(e);
            }

            includes.push_back(file.to_string());
        }
    }

    if e != 0 { return Err(e); }

    Ok(includes)
}
