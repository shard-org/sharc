use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::args_parser::ARGS;
use crate::{utils, logerr, log};
use crate::logger::{Level, Debug, logger, At};

const DBG: &Debug = &Debug::PreProcessor;

pub fn process() -> Vec<Group> {
    let infile = &ARGS.infile;

    let path: Rc<Path> = Rc::from(PathBuf::from(infile).as_path());
    
    // add all files within the project dir
    let mut groups = init_project(path);

    // parse them includes into 
    // Vec<Vec<File>>
    groups = parse_includes(&mut groups);
    
    /*
     * TODO more directive parsing
     */

    groups 
}

pub type Group = Vec<File>;

// these are just for debugging
#[derive(Debug, Clone)]
pub struct File {
    path: Rc<Path>,
    content: String,
}

fn new_group(path: Rc<Path>, content: String) -> Group {
    vec![File { path, content }]
}

fn init_project(path: Rc<Path>) -> Vec<Group> {
    // if there's no parent just read the current dir
    let files_to_parse = utils::rec_reader( {
        let Some(path) = path.parent() else {
            logerr!(DBG, "Could not get parent directory");
            std::process::exit(1);
        };

        if path == Path::new("") {
            Path::new("./")
        } else { path }
    });

    let mut out_files = Vec::new();

    // read all the files
    for file in files_to_parse {
        let path = Rc::from(file.as_path());
        let content = utils::reader(file.to_str().unwrap()).basic_process_pass();

        out_files.push(File { path, content });
    }

    vec![out_files]
} 

// TODO I dont like this but it works, ideally this wouldn't create a new list but rather just
// modify the existing. But uh, rust is pain and ye cant modify and read at the same time. Maybe
// some indexing bs so its only borrowed for a short time? idk
fn parse_includes(groups: &mut Vec<Group>) -> Vec<Group> {
    let mut out_groups = Vec::new();

    while let Some(group) = groups.pop() {
        // group = Vec<File>

        for file in &group {
            // file = Rc<Path>, String
            let fname = file.path.to_str().unwrap();

            for (i, line) in file.content.lines().enumerate() {
                let i = i + 1;

                let Some(args) = line.strip_prefix(".inc") else {
                    continue;
                };

                let args = args.trim();
                match parse_mod_lib(args) {
                    // file? real?!
                    Ok(inc) if !inc.path.exists() => {
                        logerr!(At::new(&i, fname), DBG, format!("File {} does not exist", inc.path.to_str().unwrap()));
                        continue;
                    },

                    Ok(inc) => {
                        match inc.module {
                            // all modules in the lib
                            Some(module) if module == "*" => {
                                utils::rec_reader(&inc.path).into_iter().new_group_all(groups);
                            },

                            // only one module
                            Some(module) => {
                                utils::rec_reader(&inc.path).into_iter()
                                    .filter(|f| f.is_file())
                                    .filter(|f| f.to_str().unwrap().contains(inc.path.join(&module).to_str().unwrap()))
                                    .new_group_all(groups);
                            },

                            // only the base files
                            None => {
                                utils::read_dir(&inc.path).into_iter()
                                    .filter(|f| f.is_file())
                                    .new_group_all(groups);
                            }
                        }
                        
                    },

                    Err(e) => {
                        logerr!(At::new(&i, fname), DBG, e);
                        continue;
                    }
                }
            }
        }

        out_groups.push(group);
    }

    if ARGS.debug {
        log!(Level::Debug, DBG, format!("Groups: {:#?}", out_groups));
    }

    out_groups
}

struct Include {
    path: Rc<Path>,
    module: Option<String>,
}

fn parse_mod_lib(args: &str) -> Result<Include, String> {
    let path: Rc<Path>;
    let module: Option<String>;

    if let Some(args) = args.strip_prefix('"') {
        // split path and module
        let Some((path_str, module_arg)) = args.split_once('"') else {
            return Err(String::from("Missing matching `\"`"));
        };

        if args.ends_with('"') {  
            // ex: .inc "path/to/file".module
            path = Rc::from(Path::new(path_str));
            module = Some(module_arg.trim_start_matches('.').to_string());
        }

        else {
            // ex: .inc "path/to/file"
            let args = &args[..args.len()-1];
            path = Rc::from(Path::new(args));
            module = None;
        }
    }

    else if let Some((libname, module_arg)) = args.split_once('.'){  
        // ex: .inc lib.module
        path = Rc::from(ARGS.syslib.join(libname));
        module = Some(module_arg.trim_start_matches('.').to_string());
    }

    else {
        // ex: .inc lib
        path = Rc::from(ARGS.syslib.join(args));
        module = None;
    }

    Ok(Include { path, module })
}

trait PathBufIteratorExt {
    fn new_group_all(self, groups: &mut Vec<Group>);
}

impl<T: Iterator<Item = PathBuf>> PathBufIteratorExt for T {
    fn new_group_all(self, groups: &mut Vec<Group>){
        self.for_each(|file| {
            let path = Rc::from(file.as_path());
            let content = utils::reader(file.to_str().unwrap()).basic_process_pass();

            groups.push(new_group(path, content));
        });
    }
}

//
// TODO here be dragons
#[derive(PartialEq)]
enum State {
    Normal,
    InComment,
    InSingleLineComment,
    InString,
    InStringEscape,
}

trait FileProcess {
    fn basic_process_pass(self) -> String;
}

impl FileProcess for String {
    fn basic_process_pass(self) -> String {
        // surface level cleaning
        let file = self.lines()
            .map(|l| l.trim())
            .filter(|l| !l.starts_with("//") && !l.is_empty());
     
        let mut state = State::Normal;
        let mut out = String::new();

        for ln in file { 
            let mut out_line = String::new();
            let mut chars = ln.chars().peekable();

            while let Some(c) = chars.next() {
                match state {
                    State::Normal => {
                        // check for comment
                        if c == '/' {
                            match chars.peek() {
                                Some(&'/') => {
                                    state = State::InSingleLineComment;
                                    chars.next();
                                    continue;
                                },
                                Some(&'*') => {
                                    state = State::InComment;
                                    chars.next();
                                    continue;
                                },
                                _ => out_line.push(c),
                            }
                        }

                        out_line.push(c);

                        // if not comment check for string
                        if c == '"' {
                            state = State::InString;
                        }
                    },

                    State::InComment => {
                        // check for end of comment
                        if c == '*' && chars.peek() == Some(&'/') {
                            state = State::Normal;
                            chars.next();
                        }
                    },

                    // do nothing in single line comments
                    State::InSingleLineComment => (),

                    State::InString => {
                        out_line.push(c);

                        // check for escape
                        if c == '\\' {
                            state = State::InStringEscape;
                            continue;
                        }

                        // check for end of string
                        if c == '"' {
                            state = State::Normal;
                        }
                    },

                    State::InStringEscape => {
                        out_line.push(c);
                        state = State::InString;
                    },
                }
            }

            // reset single line comments at the end of the line
            if state == State::InSingleLineComment {
                state = State::Normal;
                continue;
            }

            if !out_line.is_empty() {
                // append to the last line if we are continuing
                out.push_str(&out_line);
            }
        }
        out
    }
}
