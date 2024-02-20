use crate::macros::IterExt;
use crate::logger::{Log, Level, Logs};
use crate::location::Span;

#[derive(Debug)]
pub enum Arch {
    X86_64Linux,
    None,
}

#[derive(Debug)]
pub struct Configs {
    pub name:     String,
    pub version:  String,
    pub deps:     Vec<String>,
    pub includes: Vec<String>,

    pub arch:     Arch, // required
    pub linker:   Vec<String>,
}

impl Configs {
    fn default() -> Self {
        Self {
            name:     String::from("out"),
            version:  String::new(),
            deps:     Vec::new(),
            includes: Vec::new(),
            arch:     Arch::None,
            linker:   Vec::new(),
        }
    }

    pub fn parse(input: &str, logs: &mut Vec<Log>, filename: &str) -> Self {
        let mut configs = Configs::default();
        let mut lines = input.lines().enumerate();

        while let Some((li, line)) = lines.next() {
            let mut chars = line.chars().enumerate().peekable();
            let li = li + 1;
            let ci = 1;

            if !line.trim().starts_with(":") { continue; }

            let _ = chars.next();

            let Some((key_i, key)) = chars.word() else {
                Span::new(filename, li, ci)
                    .to_log()
                    .msg("Missing config key after `:`")
                    .level(Level::Err)
                    .push(logs);
                continue;
            };

            match key.as_str() {
                "name" => {
                    if configs.name != "out" {
                        Span::new(filename, li, ci)
                            .length(line.len())
                            .to_log()
                            .msg("Config key `name` overset")
                            .level(Level::Warn)
                            .push(logs);
                    }

                    let name = chars.clone().map(|c| c.1)
                        .take_while(|&c| c != '\n')
                        .collect::<String>();
                    let name = name.trim();

                    if name.is_empty() {
                        Span::new(filename, li, line.len()+1)
                            .length(5)
                            .to_log()
                            .msg("Missing value for config key `name`")
                            .level(Level::Warn)
                            .push(logs);
                    }

                    configs.name = name.to_string();
                },

                "version" => {
                    if !configs.version.is_empty() {
                        Span::new(filename, li, ci)
                            .length(line.len())
                            .to_log()
                            .msg("config key `version` overset")
                            .level(Level::Warn)
                            .push(logs);
                    }

                    let version = chars.clone().map(|c| c.1)
                        .filter(|c| c.is_whitespace())
                        .take_while(|&c| c != '\n')
                        .collect::<String>();
                    let version = version.trim();

                    if version.is_empty() {
                        Span::new(filename, li, line.len()+1)
                            .length(5)
                            .to_log()
                            .msg("Missing value for config key `version`")
                            .level(Level::Warn)
                            .push(logs);
                    }

                    configs.version = version.to_string();
                },

                "use" => {
                    let dep = chars.clone().map(|c| c.1)
                        .take_while(|&c| c != '\n')
                        .collect::<String>();
                    configs.deps.push(dep.trim().to_string());
                },

                "include" => todo!("`:include` not yet implemented"),
                
                "link" => {
                    if !configs.linker.is_empty() {
                        Span::new(filename, li, ci)
                            .length(line.len())
                            .to_log()
                            .msg("config key `link` overset")
                            .level(Level::Warn)
                            .push(logs);
                    }

                    while let Some((_, ln)) = chars.word() {
                        configs.linker.push(ln);
                    }

                    if configs.linker.is_empty() {
                        Span::new(filename, li, line.len()+1)
                            .length(5)
                            .to_log()
                            .msg("Missing value for config key `link`")
                            .level(Level::Warn)
                            .push(logs);
                    }
                },

                "arch" => {
                    let arch_str = chars.clone().map(|c| c.1)
                        .take_while(|&c| c != '\n')
                        .collect::<String>();

                    let arch_str = arch_str.trim();

                    configs.arch = match arch_str {
                        "x86_64 linux" => Arch::X86_64Linux,
                        _ => {
                            Span::new(filename, li, ci)
                                .length(line.len())
                                .to_log()
                                .msg("Architectures other than `x86_64 linux` are not yet supported")
                                .level(Level::Fatal)
                                .push(logs);
                            logs.print();
                            std::process::exit(1)
                        },
                    }
                },

                // verbs get handled at `verbs.rs`. This is because they depend
                // on all macros being set, and we parse config keys before macros
                "verb" => (), 
                
                a => {
                    Span::new(filename, li, ci)
                        .length(key_i)
                        .to_log()
                        .msg(format!("Invalid config key `{}`", a))
                        .level(Level::Err)
                        .push(logs);

                },

            }

        }

        configs
    }
}
