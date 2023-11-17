use super::position::Span;
use std::rc::Rc;
use std::cell::RefCell;

// A struct which aggregates all diagnostics
#[derive(Default, Clone, Debug)]
pub struct Reporter {
    // These types are necessarily dense: Rc provides
    // shared, immutable access. Refcell provides mutability 
    diagnostics: Rc<RefCell<Vec<Diagnostic>>>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub message: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<Note>,
}


pub struct Builder {
    reporter: Reporter,
    diagnostic: Diagnostic,
}

impl Builder {
    fn new(reporter: &Reporter, severity: Severity, message: String, span: Span) -> Self {
        Builder { 
            reporter: reporter.clone(), 
            diagnostic: Diagnostic { 
                severity, 
                message, 
                span: Some(span),
                notes: Vec::new() 
            } 
        }
    }

    fn push_note(mut self, message: Option<String>, span: Span) -> Self {
        self.diagnostic.notes.push(Note { message, span });
        self
    }

    pub fn span(self, span: Span) -> Self {
        self.push_note(None, span)
    }

    pub fn note<T: Into<String>>(self, message: T, span: Span) -> Self {
        self.push_note(Some(message.into()), span)
    }

    pub fn build(self) {
        assert!(!self.diagnostic.notes.is_empty(), "Built without notes! Use global diagnostics instead");
        self.reporter.diagnostics.borrow_mut().push(self.diagnostic);
    }
}
