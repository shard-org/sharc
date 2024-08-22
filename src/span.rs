use std::cmp::Ordering;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq)]
pub struct Span {
    pub filename:    &'static str,
    pub line_number: usize,
    pub offset:      usize,
    pub length:      usize,
}

impl Default for Span {
    fn default() -> Self {
        Self { filename: "", line_number: 1, offset: 0, length: 0 }
    }
}

impl Span {
    pub fn new(filename: &'static str, line_number: usize, offset: usize, length: usize) -> Self {
        Self { filename, line_number, offset, length }
    }

    pub fn len(mut self, len: usize) -> Self {
        self.length = len;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn ghost<T: std::fmt::Display>(self, ghost: T) -> (Self, HighVec) {
        let ghost = ghost.to_string();
        assert!(self.length > 0);
        assert_eq!(ghost.len(), self.length);

        let mut vec = Vec::new();
        (0..self.offset).for_each(|_| vec.push(HighlightKind::Empty));
        ghost.chars().for_each(|c| vec.push(HighlightKind::Ghost(c)));

        (self, vec)
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.length == 0 {
            write!(f, "{}:{}:{}", self.filename, self.line_number, self.offset)
        }
        else {
            write!(
                f,
                "{}:{}:{}-{}",
                self.filename,
                self.line_number,
                self.offset,
                self.offset + self.length - 1
            )
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line_number, self.offset)
    }
}

type HighVec = Vec<HighlightKind>;

impl From<Span> for (Span, HighVec) {
    fn from(val: Span) -> (Span, HighVec) {
        let mut vec = Vec::new();

        (0..val.offset).for_each(|_| vec.push(HighlightKind::Empty));
        (0..val.length).for_each(|_| vec.push(HighlightKind::Caret));

        (val, vec)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum HighlightKind {
    Ghost(char) = 2,
    Caret = 1,
    Empty = 0,
}

pub fn combine(vec_a: HighVec, vec_b: HighVec) -> HighVec {
    let (mut a_iter, mut b_iter) = match vec_a.len().cmp(&vec_b.len()) {
        Ordering::Less => (vec_b.into_iter().peekable(), vec_a.into_iter().peekable()),
        _ => (vec_a.into_iter().peekable(), vec_b.into_iter().peekable()),
    };

    let mut vec = Vec::new();
    while let Some(a) = a_iter.next() {
        let Some(b) = b_iter.next()
        else {
            vec.push(a);
            continue;
        };

        if matches!(a, HighlightKind::Ghost(_)) {
            vec.push(a);
            while let Some(HighlightKind::Ghost(_)) = a_iter.peek() {
                vec.push(a_iter.next().unwrap());
            }
            vec.push(b);
        }
        else if matches!(b, HighlightKind::Ghost(_)) {
            vec.push(b);
            while let Some(HighlightKind::Ghost(_)) = b_iter.peek() {
                vec.push(b_iter.next().unwrap());
            }
            vec.push(a);
        }
        else if a < b {
            vec.push(b);
        }
        else {
            vec.push(a);
        }
    }

    vec
}
