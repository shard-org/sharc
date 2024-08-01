use crate::span::Span;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::sync::mpsc::Sender;

use colored::*;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Level {
    Silent,
    Note,
    Warn,
    Error,
    Fatal,
}

#[derive(Clone, Debug)]
pub enum ReportKind {
    ArgumentParserError,

    // Lexer
    UnexpectedCharacter,
    UnterminatedMultilineComment,
    UnterminatedStringLiteral,
    UnterminatedCharLiteral,

    // Preprocessor
    UndefinedMacro,
    InvalidTag,
    ExceededRecursionLimit,

    // Parser
    UnexpectedToken,
    UnexpectedEOF,
    InvalidEscapeSequence,
    DuplicateAttribute,

    // General
    IOError,
    SyntaxError,
}

impl ReportKind {
    pub fn new<T: Into<String>>(self, title: T) -> Report {
        Report::new(self, title)
    }

    pub fn level(&self) -> Level {
        match self {
            // Argument Parsing
            ReportKind::ArgumentParserError => Level::Error,

            // Lexing
            ReportKind::UnexpectedCharacter
            | ReportKind::UnterminatedMultilineComment
            | ReportKind::UnterminatedStringLiteral
            | ReportKind::UnterminatedCharLiteral => Level::Error,

            // Preprocessing
            ReportKind::UndefinedMacro
            | ReportKind::ExceededRecursionLimit
            | ReportKind::InvalidTag => Level::Error,

            // Parsing
            ReportKind::UnexpectedToken
            | ReportKind::UnexpectedEOF
            | ReportKind::DuplicateAttribute
            | ReportKind::InvalidEscapeSequence => Level::Error,

            // General
            ReportKind::IOError | ReportKind::SyntaxError => Level::Error,
        }
    }
}

#[derive(Clone)]
pub struct ReportLabel {
    span: Span,
    text: Option<String>,
}

impl ReportLabel {
    pub fn new(span: Span) -> Self {
        Self { span, text: None }
    }

    pub fn with_text<T: Into<String>>(mut self, label: T) -> Self {
        self.text = Some(label.into());
        self
    }
}

#[derive(Clone)]
pub struct Report {
    kind: ReportKind,
    title: String,
    label: Option<ReportLabel>,
    note: Option<String>,
}

impl Report {
    pub fn new<T: Into<String>>(kind: ReportKind, title: T) -> Self {
        Self { kind, title: title.into(), label: None, note: None }
    }

    pub fn with_label(mut self, label: ReportLabel) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_note<T: Into<String>>(mut self, note: T) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn level(&self) -> Level {
        self.kind.level()
    }

    pub fn display(&self, show_context: bool) {
        eprint!("{}", ReportFormatter { report: self, show_context })
    }
}

impl<T> Into<Result<T>> for Report {
    fn into(self) -> Result<T> {
        Err(self.into())
    }
}

impl PartialEq<Self> for Report {
    fn eq(&self, other: &Self) -> bool {
        self.level().eq(&other.level())
    }
}

impl PartialOrd<Self> for Report {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.level().partial_cmp(&other.level())
    }
}

struct ReportFormatter<'e> {
    report: &'e Report,
    show_context: bool,
}

impl Display for ReportFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let report = &self.report;

        let (prefix, primary_color, secondary_color) = match report.kind.level() {
            Level::Fatal => ("Fatal", Color::Red, Color::BrightRed),
            Level::Error => ("Error", Color::Red, Color::BrightRed),
            Level::Warn => ("Warning", Color::Yellow, Color::BrightYellow),
            Level::Note => ("Note", Color::White, Color::White),
            _ => unreachable!("Why does a report have the level of silent you idiot."),
        };

        writeln!(
            f,
            "{} {}",
            format!("{}", format!("[{prefix}] {:?}:", report.kind).color(primary_color)).bold(),
            report.title
        )?;

        match report.label.as_ref() {
            Some(label) => {
                let span = &label.span;
                let contents = crate::Scanner::get_file(span.filename);
                let line_index = match contents[..=span.start_index].rfind('\n') {
                    Some(val) => val + 1,
                    None => 0,
                };

                let end_line_index = {
                    match contents[span.end_index..].find('\n') {
                        Some(offset) => span.end_index + offset,
                        None => contents.len().saturating_sub(1),
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
                        &contents[span.end_index..=end_line_index].trim_end_matches('\n'),
                    )?;

                    writeln!(
                        f,
                        "{} {} {}{} {}",
                        " ".repeat(pad_num),
                        "|".cyan().dimmed(),
                        " ".repeat(span.start_index - line_index),
                        "^".repeat(span.end_index - span.start_index).color(primary_color).bold(),
                        label.text.as_ref().unwrap_or(&String::new()).color(secondary_color),
                    )?;

                    if let Some(note) = &report.note {
                        writeln!(
                            f,
                            "{} {} {}",
                            " ".repeat(pad_num),
                            "|".cyan().dimmed(),
                            note.bright_black().italic()
                        )?;
                    }
                } else {
                    if let Some(note) = &report.note {
                        writeln!(f, " {}", note.bright_black().italic())?;
                    }
                }
            },
            None => {
                if let Some(note) = &report.note {
                    writeln!(f, "{}", note.bright_black().italic())?;
                }
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

pub type Result<T> = std::result::Result<T, Box<Report>>;

pub struct ReportSender {
    sender: Sender<Box<Report>>,
}

impl ReportSender {
    pub fn new(sender: Sender<Box<Report>>) -> Self {
        Self { sender }
    }

    pub fn send(&self, report: Box<Report>) {
        self.sender.send(report).expect("Error sender failed to send report.")
    }
}

pub trait UnwrapReport<T> {
    fn unwrap_or_fatal(self, report: Box<Report>) -> T;
    fn unwrap_result(self, report: Box<Report>) -> Result<T>;
}

impl<T> UnwrapReport<T> for Option<T> {
    fn unwrap_or_fatal(self, report: Box<Report>) -> T {
        match self {
            Some(val) => val,
            None => {
                report.display(false);
                std::process::exit(1);
            },
        }
    }

    fn unwrap_result(self, report: Box<Report>) -> Result<T> {
        match self {
            Some(val) => Ok(val),
            None => Err(report),
        }
    }
}

impl<T, E> UnwrapReport<T> for std::result::Result<T, E> {
    fn unwrap_or_fatal(self, report: Box<Report>) -> T {
        match self {
            Ok(val) => val,
            Err(_) => {
                report.display(false);
                std::process::exit(1);
            },
        }
    }

    fn unwrap_result(self, report: Box<Report>) -> Result<T> {
        match self {
            Ok(val) => Ok(val),
            Err(_) => Err(report),
        }
    }
}
