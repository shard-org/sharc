use crate::logger::{Log, Level, Logs};
use crate::location::Span;

use std::iter::{Enumerate, Peekable};
use std::str::{Chars, Lines};

#[derive(Debug)]
pub struct VerbDef {
    pub ident: String,
    pub exec:  Exec,
    pub body:  String,
}

#[derive(Default, Debug)]
pub struct Exec {
    exec: String,
    args: Vec<String>,
}


#[derive(Debug)]
pub struct Macro {
    pub name:  String,
    pub value: String,
}


#[derive(Debug)]
pub struct SysArg {
    value: String,
    kind:  SysArgType,
    desc:  Option<String>,
}

#[derive(Debug)]
pub enum SysArgType {
    Equal,
    Typed,
}

// SHADOWING FOR ALL!!!
#[derive(Default, Debug)]
pub struct Tags {
    // basic
    pub name:    String,
    pub arch:    String, // required!
    pub version: String,

    // ????
    pub nostd: bool,
    pub dep:   Vec<String>, // pulled from git repos
    pub use:   Vec<String>, // other shard files
    pub lib:   Vec<String>, // extern libs to be linked?
    
    // build components
    pub linker:    Exec, // required!
    pub assembler: Exec, // required!
    pub sharc:     Vec<String>,

    pub verbs:     Vec<VerbDef>,

    //
    // architecture specific defs, not *meant* to be used in main files
    // these should be all caps
    pub word:       usize, // word size
    pub attributes: Vec<(String, String)>, // (from, into)
    pub registers:  Vec<(String, String)>, // (from, into)

    pub syscall_addr: usize,
    pub syscall_conv: (Vec<String>, String),
    pub syscalls:     Vec<(String, Vec<SysArg>)>,
}

// i have no idea how to do this bro
// we need to do macros before tags because tags can have macros in them
// but tags also cause macros????

impl Tags {
    // fn to shadow the args from libs with the ones from main.
    // this is required as main needs to be parsed before libs
    fn shadow(&mut self, new: Self) {
    }
}

pub struct Preprocess<'a> {
    filename: &'static str,
    logger:   &'a mut Vec<Log>,

    lines:    Peekable<Enumerate<Lines<'a>>>,
    li:       usize,
    l_len:    usize, // current line length

    chars:    Peekable<Enumerate<Chars<'a>>>,
    ci:       usize, 

    tags:     Tags,
    macros:   Vec<Macro>,
}

impl Preprocess<'_> {
    pub fn new<'a>(input: &'a str, logger: &'a mut Vec<Log>, filename: &'static str) -> Preprocess<'a> {
        let mut lines = input.lines().enumerate().peekable();

        let Some((_, line)) = lines.next() else {
            panic!("file cannot be empty... the reader SHOULD filter this");
        };

        let chars = line.chars().enumerate().peekable();

        Preprocess{ logger, filename, lines, li: 1, l_len: line.len(), chars, ci: 1, tags: Tags::default(), macros: Vec::new()}
    }

    pub fn parse(&mut self) -> (Tags, Vec<Macro>) {
        while self.next_line().is_some() {
            self.trim_leading();
            if self.peek() != Some(':') { continue; }
            let _ = self.next();

            match self.word().as_str() {
                "name" => self.tags.name = self.line().trim().to_string(),
                "arch" => self.tags.arch = self.line().trim().to_string(),
                "version" => self.tags.arch = self.line().trim().to_string(),



                "nostd" => self.tags.nostd = true,



                "linker" => {
                    let args = self.take_words();

                    let Some(exec) = args.remove(0) else {
                        self.to_span()
                            .col(|x| x+1)
                            .length(5)
                            .into_log()
                            .msg("Missing name or path")
                            .push(self.logger);
                        continue;
                    };

                    self.tags.linker.exec = exec;
                    self.tags.linker.args.extend(args);
                },

                "assembler" => {
                    let args = self.take_words();

                    let Some(exec) = args.remove(0) else {
                        self.to_span()
                            .col(|x| x+1)
                            .length(5)
                            .into_log()
                            .msg("Missing name or path")
                            .push(self.logger);
                        continue;
                    };

                    self.tags.assembler.exec = exec;
                    self.tags.assembler.args.extend(args);
                },

                "sharc" => {
                    let args = self.take_words();

                    if args.is_empty() {
                        self.to_span()
                            .col(|x| x+1)
                            .length(5)
                            .into_log()
                            .msg("Missing args")
                            .push(self.logger);
                        continue;
                    }

                    self.tags.sharc.extend(args);
                },




                "WORD" => {
                    self.tags.word = match self.word().parse::<usize>() {
                        Ok(w) => w,
                        Err(e) => {
                            self.to_span()
                                .col(|x| x+1)
                                .length(5)
                                .into_log()
                                .msg("Invalid WORD definition")
                                .notes(e.to_string())
                                .push(self.logger);
                            continue;
                        },
                    }
                },



                s if s.is_empty() => continue,

                s => self.to_span()
                        .into_log()
                        .msg(format!("Invalid Tag `{}`", s))
                        .push(self.logger),
            }
        }


        if self.tags.arch.is_empty() {
            Log::new()
                .level(Level::Fatal)
                .msg("arch must be set")
                .push(self.logger);
            std::process::exit(1);
        }

        if self.tags.linker.exec.is_empty() {
            Log::new()
                .level(Level::Fatal)
                .msg("linker must be set")
                .push(self.logger);
            std::process::exit(1);
        }

        if self.tags.assembler.exec.is_empty() {
            Log::new()
                .level(Level::Fatal)
                .msg("assembler must be set")
                .push(self.logger);
            std::process::exit(1);
        }

        todo!()
    }


    fn take_words(&mut self) -> Vec<String> {
        let words = Vec::new();
        while let Some(word) = self.word().map_if(|w| !w.is_empty()) {
            words.push(word);
        } words
    }

    fn word(&mut self) -> String {
        let mut word = String::new();
        self.trim_leading();

        while let Some(c) = self.peek() {
            if c.is_whitespace() { break; }
            let _ = self.next();
            word.push(c);
        } word
    }

    fn trim_leading(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() { break };
            let _ = self.next();
        }
    }

    fn line(&mut self) -> String {
        let mut word = String::new();
        while let Some(c) = self.next() {
            word.push(c);
        } word
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|c| c.1)
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            self.ci = c.0; c.1
        })
    }

    fn next_line(&mut self) -> Option<()> {
        self.lines.next()
            .map(|(li, l)| {
                self.li = li;
                self.l_len = l.len();
                self.chars = l.chars().enumerate().peekable();
            })
    }

    fn to_span(&self) -> Span {
        Span::new(self.filename, self.li, self.ci + 1)
    }
}

