use std::process::{Command, exit, Stdio};
use std::io::{Write, BufRead, BufReader};
use std::thread;

use crate::{
    logger::{Log, Level},
    location::Span,
    macros::IterExt,
    fatal,
};


#[derive(Debug, Default)]
pub struct ParsedVerb {
    pub name: String,
    pub exec: String,
    pub args: Vec<String>,
    pub body: Option<String>,
}

impl ParsedVerb {
    pub fn parse(input: &str, logs: &mut Vec<Log>, filename: &'static str) -> Vec<Self> {
        let mut verbs: Vec<Self> = Vec::new();
        let mut lines = input.lines().enumerate();

        while let Some((li, line)) = lines.next() {
            let mut chars = line.chars().enumerate().peekable();
            let li = li+1;

            if !line.trim().starts_with(":verb") { continue; }

            let mut verb = ParsedVerb::default();

            let Some((ci, _)) = chars.word() else {
                unreachable!();
            };

            verb.name = {
                let Some((_, name)) = chars.word() else {
                    Span::new(filename, li, ci+2)
                        .length(5)
                        .into_log()
                        .msg("Missing verb identifier")
                        .level(Level::Err)
                        .push(logs);
                    continue;
                }; name
            };

            verb.exec = {
                let Some((_, exec)) = chars.word() else {
                    Span::new(filename, li, ci+2)
                        .length(5)
                        .into_log()
                        .msg("Missing verb identifier")
                        .level(Level::Err)
                        .push(logs);
                    continue;
                }; exec
            };

            while let Some((_, arg)) = chars.word() {
                if arg != "{" { 
                    verb.args.push(arg);
                    continue;
                }

                let mut body = String::new();
                let mut lines_clone = lines.clone();
                while let Some((_, line)) = lines_clone.next() {
                    // TODO: Add error handling is the closing brace is missing
                    if let Some(line) = line.trim().strip_suffix('}') {
                        body.push_str(line);
                        body.push('\n');
                        break;
                    }

                    body.push_str(line);
                    body.push('\n');
                }

                verb.body = Some(body);
                break;
            }
    
            verbs.push(verb);
        }

        verbs
    }

    pub fn execute(&self, args: &[&str]) {
        let args = &self.args.iter()
            .flat_map(|a| {
                if a == "*" {
                    args.to_vec()
                } else { 
                    vec![a.as_str()] 
                }
            }).collect::<Vec<&str>>();


        let Ok(mut exec) = Command::new(&self.exec)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() else {
                fatal!("Failed to run process `{}`.", self.exec);
                exit(1)
            };

        if let Some(stdout) = exec.stdout.take() {
            let stdout_reader = BufReader::new(stdout);
            thread::spawn(move || {
                stdout_reader.lines().for_each(|l| println!("{}", l.unwrap()));
            });
        }

        if let Some(stderr) = exec.stderr.take() {
            let stderr_reader = BufReader::new(stderr);
            thread::spawn(move || {
                stderr_reader.lines().for_each(|l| eprintln!("{}", l.unwrap()));
            });
        }

        if let Some(body) = &self.body {
            if let Some(mut stdin) = exec.stdin.take() {
                stdin.write_all(body.as_bytes()).unwrap();
            }
        }

        exec.wait().expect("failed to wait for child process");
    }
}


