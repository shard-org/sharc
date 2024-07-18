use std::fmt::Formatter;

#[derive(Clone)]
pub struct Span {
    pub filename: &'static str,
    pub line_number: usize,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(
        filename: &'static str,
        line_number: usize,
        line: usize,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            filename,
            line_number,
            line,
            start,
            end,
        }
    }

    pub fn extend(&self, other: &Self) -> Self {
        Self {
            filename: self.filename,
            line_number: self.line_number,
            line: self.line,
            start: self.start,
            end: other.end,
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let column = self.start - self.line;
        write!(f, "{}:{}:{}", self.filename, self.line_number, column)
    }
}
