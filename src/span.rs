use std::fmt::Formatter;

#[derive(Clone)]
pub struct Span {
    pub filename: &'static str,
    pub line_number: usize,
    pub line_index: usize,
    pub start_index: usize,
    pub end_index: usize,
}

impl Span {
    pub fn new(
        filename: &'static str,
        line_number: usize,
        line_index: usize,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        Self {
            filename,
            line_number,
            line_index,
            start_index,
            end_index,
        }
    }

    pub fn extend(&self, other: &Self) -> Self {
        Self {
            filename: self.filename,
            line_number: self.line_number,
            line_index: self.line_index,
            start_index: self.start_index,
            end_index: other.end_index,
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let column = self.start_index - self.line_index;
        write!(f, "{}:{}:{}", self.filename, self.line_number, column)
    }
}
