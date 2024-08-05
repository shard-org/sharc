use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::sync::mpsc::Sender;

use bitvec::{bitvec, vec::BitVec};
use colored::{Color, Colorize};

use crate::span::{Span, SpanWrapper};

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
    SelfReferentialMacro,

    // Parser
    UnexpectedToken,
    UnexpectedEOF,
    InvalidEscapeSequence,
    DuplicateAttribute,
    RegisterWithinHeap,

    // General
    IOError,
    SyntaxError,
}

impl ReportKind {
    pub fn title<T, S>(self, title: T) -> Report 
        where T: Into<Option<S>>,
              S: Into<String>
    {
        Report { 
            kind: self, 
            title: title.into(), 
            span: None, 
            span_mask: BitVec::new(),
            label: None, 
            footer: None, 
        }
    }

    pub fn level(&self) -> Level {
        match self {
            // Argument Parsing
            Self::ArgumentParserError

            // Lexing
            | Self::UnexpectedCharacter
            | Self::UnterminatedMultilineComment
            | Self::UnterminatedStringLiteral
            | Self::UnterminatedCharLiteral

            // Preprocessing
            | Self::UndefinedMacro
            | Self::ExceededRecursionLimit
            | Self::SelfReferentialMacro
            | Self::InvalidTag

            // Parsing
            | Self::UnexpectedToken
            | Self::UnexpectedEOF
            | Self::DuplicateAttribute
            | Self::InvalidEscapeSequence
            | Self::RegisterWithinHeap

            // General
            | Self::IOError | Self::SyntaxError => Level::Error,
        }
    }
}


#[derive(Clone)]
pub struct Report {
    kind:      ReportKind,
    title:     Option<String>,
    spans:     Option<Vec<SpanWrapper>>,
    label:     Option<String>,
    footers:    Option<Vec<String>>,
}

impl Report {
    pub fn span(mut self, span: Span) -> Self {
        match self.spans {
            Some(ref mut spans) => spans.push(span.into()),
            None => self.span = Some(vec![span.into()]),
        }
    }

    pub fn label<T: Into<String>>(mut self, label: T) -> Self {
        self.label = Some(label.into()); self
    }

    pub fn help<T: Display>(mut self, help: T) -> Self {
        self.footers("HELP", help); self
    }

    pub fn info<T: Display>(mut self, info: T) -> Self {
        self.footers("INFO", info); self
    }

    pub fn note<T: Display>(mut self, note: T) -> Self {
        self.footers("NOTE", note); self
    }

    fn footers<T: Display>(mut self, prefix: &str, text: T) {
        match self.footers {
            Some(ref mut footers) => footers.push(format!("{prefix}: {text}")),
            None => self.footers = Some(vec![format!("{prefix}: {text}")]),
        }
    }

    pub fn level(&self) -> Level {
        self.kind.level()
    }


    fn as_span(&self) -> Span {
        self.spans.as_ref().unwrap()[0].span()
    }

    fn fmt_header(&self, prefix: &str, colour: Color) -> String {
        format!("[{prefix}] {:?}:", self.kind).color(colour).bold() 
            + &self.title.unwrap_or_default()
    }

    fn fmt_info_line(&self) -> String {
        format!(" {} {}", "--->".cyan(), self.as_span())
    }

    fn fmt_get_separator(&self, padding: usize) -> String {
    }

    fn fmt_file_line(&self, secondary: Color, padding: usize, mask: &BitVec) -> &'static str {
        let line = crate::Scanner::get(self.span.filename)
            .lines().nth(self.span.line_number - 1)
            .unwrap_or("Could not fetch line.");

        let line = mask.iter().chain(line.chars()).fold(String::new(), |mut line, (bit, char)| {
            match bit {
                true => line.push_str(format!("{}", char.color(secondary))),
                false => line.push(char),
            } line
        });

        fmt_get_separator(padding) + line
        // TODO ghost chars
    }

    fn fmt_span_line(&self, primary: Color, secondary: Color, padding: usize, mask: &BitVec) -> String {
        let caret_str = mask.iter().fold(String::new(), |mut line, bit| {
            line.push_str(if bit { "^" } else { " " });
            line
        });
        
        format!(
            "{}{} {}",
            fmt_get_separator(padding),
            caret_str.color(primary).bold(),
            self.label.text.as_ref().unwrap_or(&String::new()).color(secondary),
        )
    }
}

impl<T> Into<Result<T>> for Report {
    fn into(self) -> Result<T> {
        Err(self.into())
    }
}

// impl PartialEq<Self> for Report {
//     fn eq(&self, other: &Self) -> bool {
//         self.level().eq(&other.level())
//     }
// }
//
// impl PartialOrd<Self> for Report {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.level().partial_cmp(&other.level())
//     }
// }


impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        assert!(self.spans.is_some() || self.label.is_none());

        let (prefix, primary, secondary) = match self.kind.level() {
            Level::Fatal => ("Fatal", Color::Red, Color::BrightRed),
            Level::Error => ("Error", Color::Red, Color::BrightRed),
            Level::Warn => ("Warning", Color::Yellow, Color::BrightYellow),
            Level::Note => ("Note", Color::White, Color::White),
            Level::Silent => unreachable!("Why does a report have the level of silent you idiot."),
        };

        writeln!(f, "{}", self.fmt_header(prefix, primary))?;

        let padding = self.report.label.map(|&label| {
            writeln!(f, "{}", self.fmt_info_line())?;

            let mask = self.spans.iter().fold(BitVec::new(), |mut mask, span|  {
                let mut bit = bitvec![0; span.offset()];
                bit.append(&mut bitvec![1; span.length()]);
                mask |= bit;
                mask
            });

            let padding = self.span().line_number.to_string().len();
            let padding = format!("{} {} ", " ".repeat(padding), "|".cyan().dimmed());

            writeln!(f, "{}", self.fmt_file_line(secondary, padding, &mask))?;
            writeln!(f, "{}", self.fmt_span_line(primary, secondary, padding, &mask))?;

            padding
        }).unwrap_or_default();

        self.footers.map(|footers| {
            for footer in footers {
                writeln!(f, "{}{}", padding, footer.bright_black().italic())?;
            }
        });

        Ok(())
    }
}

pub trait Unbox<T> {
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
        self.sender.send(report).expect("Error sender failed to send report.");
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
