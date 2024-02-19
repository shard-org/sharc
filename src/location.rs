use crate::logger::Log;

#[derive(Debug, Default)]
pub struct Span {
    pub file:   Box<str>,
    // both line and col are counted from 1
    pub line:   usize,
    pub col:    usize,
    pub length: Option<usize>,
}

impl Span {
    pub fn new<T: std::fmt::Display>(file: T, line: usize, col: usize) -> Self {
        Self {
            file: file.to_string().into_boxed_str(),
            line,
            col,
            length: None,
        }
    }

    pub fn to_log(self) -> Log {
        Log::new().span(self)
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = Some(length); self
    }

    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.col = 0;
            return;
        } 

        self.col += 1;
    }
}
