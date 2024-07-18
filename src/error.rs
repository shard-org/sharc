use crate::span::Span;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ErrorLevel {
    Fatal,
    Error,
    Warn,
    Note,
    Silent,
}

#[derive(Clone)]
pub enum ErrorKind {
    ArgumentParserError,

    IOError,
    UnexpectedCharacter,
}

impl ErrorKind {
    pub fn new(self, title: String) -> Error {
        Error::new(self, title)
    }
    pub fn level(&self) -> ErrorLevel {
        match self {
            // Error
            ErrorKind::UnexpectedCharacter
            | ErrorKind::ArgumentParserError
            | ErrorKind::IOError => ErrorLevel::Error, // Warning
                                                       // ...
                                                       // Note
                                                       // ...
        }
    }
}

#[derive(Clone)]
pub struct ErrorLabel {
    span: Span,
    text: Option<String>,
}

impl ErrorLabel {
    pub fn new(span: Span) -> Self {
        Self { span, text: None }
    }

    pub fn with_text(mut self, label: String) -> Self {
        self.text = Some(label);
        self
    }
}

#[derive(Clone)]
pub struct Error {
    kind: ErrorKind,
    title: String,
    label: Option<ErrorLabel>,
    note: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, title: String) -> Self {
        Self {
            kind,
            title,
            label: None,
            note: None,
        }
    }

    pub fn with_label(mut self, label: ErrorLabel) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }

    pub fn into_boxed_error(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn display(self, show_context: bool) {
        eprintln!(
            "{}",
            ErrorFormatter {
                error: self,
                show_context
            }
        )
    }
}

struct ErrorFormatter {
    error: Error,
    show_context: bool,
}

impl Display for ErrorFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error = &self.error;
        let prefix = match error.kind.level() {
            ErrorLevel::Fatal => "Fatal",
            ErrorLevel::Error => "Error",
            ErrorLevel::Warn => "Warning",
            ErrorLevel::Note => "Note",
            _ => unreachable!("Why does an error have the level of silent you idiot."),
        };

        writeln!(f, "{}: {}", prefix, error.title)?;
        match error.label.as_ref() {
            Some(label) => {
                writeln!(f, "-----> {}", label.span)?;
                if self.show_context {
                    unimplemented!("Labels are not yet supported. BLAME ANTHONY")
                }
            }
            None => {}
        };
        if error.note.is_some() {
            writeln!(f, "       {}", error.note.as_ref().unwrap())?;
        };
        Ok(())
    }
}

pub(crate) trait Unbox<T> {
    type InnerValue;
    fn unbox(&self) -> Self::InnerValue;
}

impl<T: ToOwned> Unbox<T> for Box<T> {
    type InnerValue = T::Owned;

    fn unbox(&self) -> Self::InnerValue {
        (**self).to_owned()
    }
}

type Result<T> = std::result::Result<T, Box<Error>>;

pub struct ErrorSender {
    sender: Sender<Error>,
}

impl ErrorSender {
    pub fn send(&self, error: Box<Error>) {
        self.sender
            .send(error.unbox())
            .expect("Error sender failed to send error.")
    }
}
