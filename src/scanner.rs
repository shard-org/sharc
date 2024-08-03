use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::sync::{LazyLock, RwLock};

use crate::report::{ReportKind, ReportLabel, UnwrapReport};

pub struct Scanner {
    filename: &'static str,
    index:    usize,
    contents: &'static mut String,
    reader:   BufReader<File>,
}

static CACHE: LazyLock<RwLock<HashMap<&'static str, &'static str>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

impl Scanner {
    pub fn get(filename: &'static str) -> &'static str {
        if let Some(contents) = CACHE.read().unwrap().get(&filename) {
            return contents;
        }

        let contents = Self::new(filename)
            .unwrap_or_fatal(
                ReportKind::IOError.new(format!("Failed to open file: '{filename}'")).into(),
            )
            .read()
            .unwrap_or_fatal(
                ReportKind::IOError.new(format!("Failed to read file: '{filename}'")).into(),
            )
            .contents;

        CACHE.write().unwrap().insert(filename, contents);
        contents
    }

    fn new(filename: &'static str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let file_size = usize::try_from(file.metadata()?.len()).unwrap();

        Ok(Self {
            filename,
            index: 0,
            contents: Box::leak(String::with_capacity(file_size).into()),
            reader: BufReader::new(file),
        })
    }

    fn read(mut self) -> io::Result<Self> {
        let mut buf = [0u8; 1];

        while self.reader.read(&mut buf)? > 0 {
            match std::str::from_utf8(&buf) {
                Ok(s) => match s {
                    "\r" => continue,
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
                                },
                                _ => {},
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
                },
            }
            self.index += 1;
        }

        Ok(self)
    }
}
