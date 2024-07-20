use crate::span::Span;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;

use colored::*;

pub enum ExitCode {
    OK = 0,
    Generic = 1,
    ArgParsing = 2,
    FileIO = 3,
    Lexer = 9,
    Parser = 16,
    Macro = 22,
    Codegen = 28,
    EasterEgg = 69,
}

pub fn exit(code: ExitCode) -> ! {
    std::process::exit(code as i32)
}

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
            format!("{}", format!("{prefix}:").color(primary_color)).bold(),
            error.title
        )?;

        match error.label.as_ref() {
            Some(label) if !self.show_context => {
                let span = &label.span;
                writeln!(f, " {} {}", "--->".cyan(), span)?;
                
                if let Some(note) = &error.note {
                    writeln!(f, " {}", note.bright_black().italic())?;
                }
            },
            Some(label) => {
                let span = &label.span;
                writeln!(f, " {} {}", "--->".cyan(), span)?;

                let file = crate::Scanner::get_file(span.filename);

                let line_index = file[..=span.start_index].rfind('\n').unwrap_or(0);

                let slice = &file[line_index..];
                let line = slice.split('\n').nth(0).unwrap_or(slice);
                
                let pad_num = span.line_number.to_string().len();
                

                println!("{} {} {}{}{}", 
                    span.line_number.to_string().dimmed().bold(),
                    "|".cyan().dimmed(),
                    &line[..span.start_index-1],
                    &line[span.start_index-1..span.end_index].color(secondary_color),
                    &line[span.end_index..],
                );

                println!(
                    "{} {} {}{} {}",
                    " ".repeat(pad_num),
                    "|".cyan().dimmed(),
                    " ".repeat(span.start_index - line_index - 1),
                    "^".repeat(span.end_index - span.start_index + 1).color(primary_color).bold(),
                    label.text.as_ref().unwrap_or(&String::with_capacity(0)).color(secondary_color),
                );

                if let Some(note) = &error.note {
                    writeln!(f, "{} {} {}", 
                        " ".repeat(pad_num),
                        "|".cyan().dimmed(),
                        note.bright_black().italic()
                    )?;
                }
            },
            None => if let Some(note) = &error.note {
                writeln!(f, "{}", note.bright_black().italic())?;
            },
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
