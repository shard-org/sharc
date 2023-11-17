use std::fmt::{Display, Debug};
use std::cmp::{min, max};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub line: usize,

    // col and offset count "char"s, not bytes since this makes
    // more sense since spans and positions are primarily used 
    // for logging. Basically Unicode Scaler Values.
    // see: https://doc.rust-lang.org/std/primitive.char.html
    pub col: usize,
    pub offset: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { line: 0, col: 0, offset: 0 }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset) 
    }
}

impl Position {
    pub fn new(line: usize, col: usize, offset: usize) -> Self {
        Position { line, col, offset }
    }

    /// Creates a new span from this to the new position
    pub fn span_to(self, to: Position) -> Span {
        Span::new(self, to) 
    }

    /// Advances the position by one character
    pub fn advance(&mut self, c: char) {
        self.offset += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }

    /// Prefer advance over this unless you know you will not
    /// cross a newline boundary in the process of moving your
    /// position.
    ///
    /// # Warning:
    /// This function is unsafe as this leads to an invalid
    /// internal state if a newline is present between the
    /// current position and the current position + amount.
    pub fn add_horizontal(&self, amount: usize) -> Position {
        Position { 
            line: self.line, 
            col: self.col + amount, 
            offset: self.offset + amount 
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.line + 1, self.col + 1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: Position,
    end: Position
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}):({})", self.start, self.end)
    }
}

impl Span {
    /// Creates a new span
    ///
    /// # Panics
    /// If start > end, this will panic
    pub fn new(start: Position, end: Position) -> Self {
        assert!(start.offset <= end.offset);
        Span { start, end }
    }

    /// returns the smallest span which contains two spans
    pub fn merge(&self, other: Span) -> Span {
        let start = min(self.start, other.start);
        let end = max(self.end, other.end);
        Span::new(start, end)
    }
}
