#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub span: Option<(usize, usize)>,    // leave both the same to indicate a single point
    pub file: &'static str,
    pub line: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.span {
            Some((start, end)) if start == end => write!(f, "{}:{}:{}", self.file, self.line, start),
            Some((start, end)) => write!(f, "{}:{}:{}-{}", self.file, self.line, start, end),
            None => write!(f, "{}:{}", self.file, self.line),
        }
    }
}

impl Location {
    /// # Panics
    ///
    /// Panics if the span is not set
    pub fn extend_span(&self, other: usize) -> Location {
        Location {
            span: Some((self.span.unwrap().0, other)),
            file: self.file,
            line: self.line,
        }
    }
}
