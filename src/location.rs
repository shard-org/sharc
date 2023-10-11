// Copied from havenselph/rattlescript
// https://github.com/HavenSelph/rattlescript/blob/main/src/common.rs
#[derive(Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}", self.line, self.column)
    }
}


impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}


#[derive(Clone, Copy)]
pub struct Span(pub &'static str, pub Location, pub Location);


impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}: {}-{}", self.0, self.1, self.2)
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Span {
    pub fn new(file: &'static str, start: Location, end: Location) -> Span {
        Span(file, start, end)
    }
    pub fn extend(&self, other: &Span) -> Span {
        Span(self.0, self.1, other.2)
    }
}
