use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub struct Span {
    pub filename:    &'static str,
    pub line_number: usize,
    pub offset:      usize,
    pub length:      usize,
}

impl Span {
    pub fn new(filename: &'static str, line_number: usize, offset: usize, length: usize) -> Self {
        Self { filename, line_number, offset, length }
    }

    // pub fn extend(&self, other: &Self) -> Self {
    //     Self {
    //         filename:    self.filename,
    //         mask:        self.mask,
    //         line_number: self.line_number,
    //     }
    // }

    pub fn ghost<T: Into<String>>(mut self, ghost: T) -> SpanWrapper {
        let ghost: String = ghost.into();
        assert_eq!(ghost.len(), self.span.length);

        let wrap: SpanWrapper = self.into();
        wrap.ghost = Some(ghost.into()); 
        wrap
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}-{}", self.filename, self.line_number, self.offset, self.offset + self.length)
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line_number, self.offset)
    }
}

#[derive(PartialEq)]
pub struct SpanWrapper {
    span: Span,
    ghost: Option<String>,
}

impl Into<SpanWrapper> for Span {
    fn into(self) -> SpanWrapper {
        SpanWrapper { span: self, ghost: None }
    }
}

impl PartialOrd for SpanWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.offset().cmp(&other.offset()))
    }
}

impl SpanWrapper {
    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn offset(&self) -> usize {
        self.span.offset
    }

    pub fn length(&self) -> usize {
        self.span.length
    }
}
