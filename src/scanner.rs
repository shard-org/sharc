use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::sync::{LazyLock, RwLock};

use crate::report::{ReportKind, ReportLabel, UnwrapReport};

pub struct Scanner {
    filename: &'static str,
    index:    usize,
    contents: String,
    reader:   BufReader<File>,
}

static CACHE: LazyLock<RwLock<HashMap<&'static str, &'static str>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

impl Scanner {
    pub fn get_cached(filename: &'static str) -> Option<&str> {
        let cache = CACHE.read().expect("failed to lock on cache");

        cache.get(&filename).copied()
    }

    pub fn get_file(filename: &'static str) -> &str {
        if let Some(contents) = Scanner::get_cached(filename) {
            return contents;
        }

        let contents = Scanner::new(filename)
            .unwrap_or_fatal(
                ReportKind::IOError
                    .new(format!("Failed to open file: '{}'", filename))
                    .into(),
            )
            .read()
            .unwrap_or_fatal(
                ReportKind::IOError
                    .new(format!("Failed to read file: '{}'", filename))
                    .into(),
            )
            .leak();

        let mut cache = CACHE.write().expect("failed to lock on cache");

        cache.insert(filename, contents);
        contents
    }

    fn new(filename: &'static str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let file_size = file.metadata()?.len() as usize;

        Ok(Self {
            filename,
            index: 0,
            contents: String::with_capacity(file_size),
            reader: BufReader::new(file),
        })
    }

    fn read(mut self) -> io::Result<Self> {
        let mut buf = [0; 1];

        while self.reader.read(&mut buf)? > 0 {
            match std::str::from_utf8(&buf) {
                Ok(s) => match s {
                    "\r" => {
                        continue;
                    }
                    _ => self.contents.push_str(s),
                },
                Err(_) => {
                    let (line_index, line_number) = self.contents.chars().enumerate().fold(
                        (0, 1),
                        |(mut li, mut ln), (index, c)| {
                            match c {
                                '\n' => {
                                    li = index;
                                    ln += 1;
                                }
                                _ => {}
                            };
                            (li, ln)
                        },
                    );
                    let span =
                        crate::span::Span::new(self.filename, line_number, line_index, self.index);
                    ReportKind::IOError
                        .new("Invalid UTF-8 data")
                        .with_label(ReportLabel::new(span))
                        .display(false);
                    std::process::exit(1);
                }
            }
            self.index += 1;
        }

        Ok(self)
    }

    fn leak(self) -> &'static str {
        Box::leak(self.contents.into_boxed_str())
    }
}
