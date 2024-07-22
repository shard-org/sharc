use crate::span::Span;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;

use colored::*;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ErrorLevel {
    Silent,
    Note,
    Warn,
    Error,
    Fatal,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    ArgumentParserError,

    // Lexer
    UnexpectedCharacter,
    UnterminatedMultilineComment,

    // Parser
    UnexpectedToken,
    UnexpectedEOF,

    // General
    IOError,
    SyntaxError,
}

impl ErrorKind {
    pub fn new<T: Into<String>>(self, title: T) -> Error {
        Error::new(self, title)
    }

    pub fn level(&self) -> ErrorLevel {
        match self {
            // Argument Parsing
            ErrorKind::ArgumentParserError => ErrorLevel::Error,

            // Lexing
            ErrorKind::UnexpectedCharacter | ErrorKind::UnterminatedMultilineComment => {
                ErrorLevel::Error
            }

            // Parsing
            ErrorKind::UnexpectedToken | ErrorKind::UnexpectedEOF => ErrorLevel::Error,

            // General
            ErrorKind::IOError | ErrorKind::SyntaxError => ErrorLevel::Error,
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

    pub fn with_text<T: Into<String>>(mut self, label: T) -> Self {
        self.text = Some(label.into());
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
    pub fn new<T: Into<String>>(kind: ErrorKind, title: T) -> Self {
        Self {
            kind,
            title: title.into(),
            label: None,
            note: None,
        }
    }

    pub fn with_label(mut self, label: ErrorLabel) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_note<T: Into<String>>(mut self, note: T) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn level(&self) -> ErrorLevel {
        self.kind.level()
    }

    pub fn display(&self, show_context: bool) {
        eprint!(
            "{}",
            ErrorFormatter {
                error: self,
                show_context
            }
        )
    }
}

impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self.into())
    }
}

impl PartialEq<Self> for Error {
    fn eq(&self, other: &Self) -> bool {
        self.level().eq(&other.level())
    }
}

impl PartialOrd<Self> for Error {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.level().partial_cmp(&other.level())
    }
}

struct ErrorFormatter<'e> {
    error: &'e Error,
    show_context: bool,
}

impl Display for ErrorFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error = &self.error;

        let (prefix, primary_color, secondary_color) = match error.kind.level() {
            ErrorLevel::Fatal => ("Fatal", Color::Red, Color::BrightRed),
            ErrorLevel::Error => ("Error", Color::Red, Color::BrightRed),
            ErrorLevel::Warn => ("Warning", Color::Yellow, Color::BrightYellow),
            ErrorLevel::Note => ("Note", Color::White, Color::White),
            _ => unreachable!("Why does an error have the level of silent you idiot."),
        };

        writeln!(
            f,
            "{} {}",
            format!(
                "{}",
                format!("[{prefix}] {:?}:", error.kind).color(primary_color)
            )
            .bold(),
            error.title
        )?;

        match error.label.as_ref() {
            Some(label) => {
                let span = &label.span;
                let contents = crate::Scanner::get_file(span.filename);
                let line_index = match contents[..=span.start_index].rfind('\n') {
                    Some(val) => val + 1,
                    None => 0,
                };
                let end_line_index = {
                    match contents[span.end_index..].find('\n') {
                        Some(offset) => span.start_index + offset,
                        None => contents.len() - 1,
                    }
                };
                writeln!(f, " {} {}", "--->".cyan(), span.to_span_printer(line_index))?;

                if self.show_context {
                    let pad_num = span.line_number.to_string().len();

                    writeln!(
                        f,
                        "{} {} {}{}{}",
                        span.line_number.to_string().dimmed().bold(),
                        "|".cyan().dimmed(),
                        &contents[line_index..span.start_index],
                        &contents[span.start_index..span.end_index].color(secondary_color),
                        &contents[span.end_index..=end_line_index],
                    )?;

                    writeln!(
                        f,
                        "{} {} {}{} {}",
                        " ".repeat(pad_num),
                        "|".cyan().dimmed(),
                        " ".repeat(span.start_index - line_index),
                        "^".repeat(span.end_index - span.start_index)
                            .color(primary_color)
                            .bold(),
                        label
                            .text
                            .as_ref()
                            .unwrap_or(&String::with_capacity(0))
                            .color(secondary_color),
                    )?;

                    if let Some(note) = &error.note {
                        writeln!(
                            f,
                            "{} {} {}",
                            " ".repeat(pad_num),
                            "|".cyan().dimmed(),
                            note.bright_black().italic()
                        )?;
                    }
                } else {
                    if let Some(note) = &error.note {
                        writeln!(f, " {}", note.bright_black().italic())?;
                    }
                }
            }
            None => {
                if let Some(note) = &error.note {
                    writeln!(f, "{}", note.bright_black().italic())?;
                }
            }
        }

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

pub type Result<T> = std::result::Result<T, Box<Error>>;

pub struct ErrorSender {
    sender: Sender<Box<Error>>,
}

impl ErrorSender {
    pub fn new(sender: Sender<Box<Error>>) -> Self {
        Self { sender }
    }

    pub fn send(&self, error: Box<Error>) {
        self.sender
            .send(error)
            .expect("Error sender failed to send error.")
    }
}
