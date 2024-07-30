use std::fmt::Formatter;

#[derive(Clone)]
pub struct Span {
    pub filename: &'static str,
    pub line_number: usize,
    pub start_index: usize,
    pub end_index: usize,
}

impl Span {
    pub fn new(
        filename: &'static str, line_number: usize, start_index: usize, end_index: usize,
    ) -> Self {
        Self { filename, line_number, start_index, end_index }
    }

    pub fn extend(&self, other: &Self) -> Self {
        Self {
            filename: self.filename,
            line_number: self.line_number,
            start_index: self.start_index,
            end_index: other.end_index,
        }
    }

    pub fn to_span_printer(&self, line_index: usize) -> SpanPrinter {
        SpanPrinter { span: self, line_index }
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}({},{})", self.filename, self.line_number, self.start_index, self.end_index)
    }
}

pub struct SpanPrinter<'s> {
    span: &'s Span,
    line_index: usize,
}

impl std::fmt::Display for SpanPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let column = self.span.start_index.saturating_sub(self.line_index);
        write!(f, "{}:{}:{}", self.span.filename, self.span.line_number, column)
    }
}
