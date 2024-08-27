use std::fmt::{Display, Formatter};

use colored::{Color, Colorize};
pub use progressh::LogHandler;

use crate::scanner::Scanner;
use crate::span::{self, HighlightKind, Span};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Level {
    Fatal,
    Error,
    Warn,
    Note,
    Silent,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ReportKind {
    _NOTE_,
    _WARNING_,
    _ERROR_,
    ArgumentParserError,

    // Lexer
    UnexpectedCharacter,
    UnterminatedMultilineComment,
    UnterminatedStringLiteral,
    UnterminatedCharLiteral,
    EmptyCharLiteral,

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
    MismatchedDelimeter,

    // General
    IOError,
    SyntaxError,

    _FATAL_,
}

impl From<ReportKind> for Level {
    fn from(kind: ReportKind) -> Self {
        match () {
            _ if kind > ReportKind::_FATAL_ => Self::Fatal,
            _ if kind > ReportKind::_ERROR_ => Self::Error,
            _ if kind > ReportKind::_WARNING_ => Self::Warn,
            _ if kind > ReportKind::_NOTE_ => Self::Note,
            _ => Self::Silent,
        }
    }
}

impl ReportKind {
    pub fn untitled(self) -> Report {
        Report {
            kind:      self,
            title:     None,
            span:      None,
            span_mask: Vec::new(),
            label:     None,
            footers:   None,
        }
    }

    pub fn title<T: Display>(self, title: T) -> Report {
        Report {
            kind:      self,
            title:     Some(title.to_string()),
            span:      None,
            span_mask: Vec::new(),
            label:     None,
            footers:   None,
        }
    }
}

#[derive(Clone)]
pub struct Report {
    kind:      ReportKind,
    title:     Option<String>,
    span:      Option<Span>,
    span_mask: Vec<HighlightKind>,
    label:     Option<String>,
    footers:   Option<Vec<String>>,
}

impl Report {
    pub fn span<T: Into<(Span, Vec<HighlightKind>)>>(mut self, span: T) -> Self {
        let (span, mask) = span.into();
        if self.span.is_none() {
            self.span = Some(span);
        }

        self.span_mask = span::combine(self.span_mask, mask);
        self
    }

    pub fn label<T: Display>(mut self, label: T) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn help<T: Display>(mut self, help: T) -> Self {
        self.footers("HELP", help);
        self
    }

    pub fn info<T: Display>(mut self, info: T) -> Self {
        self.footers("INFO", info);
        self
    }

    pub fn note<T: Display>(mut self, note: T) -> Self {
        self.footers("NOTE", note);
        self
    }

    fn footers<T: Display>(&mut self, prefix: &str, text: T) {
        match self.footers {
            Some(ref mut footers) => footers.push(format!("{prefix}: {text}")),
            None => self.footers = Some(vec![format!("{prefix}: {text}")]),
        }
    }

    #[inline]
    pub fn level(&self) -> Level {
        self.kind.into()
    }
}

impl<T> From<Report> for Result<T> {
    #[inline]
    fn from(report: Report) -> Self {
        Err(report.into())
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        assert!(self.span.is_some() || self.label.is_none());

        let (prefix, primary, secondary) = match self.kind.into() {
            Level::Fatal => ("Fatal", Color::Red, Color::BrightRed),
            Level::Error => ("Error", Color::Red, Color::BrightRed),
            Level::Warn => ("Warning", Color::Yellow, Color::BrightYellow),
            Level::Note => ("Note", Color::White, Color::White),
            Level::Silent => unreachable!("Why does a report have the level of silent you idiot."),
        };

        writeln!(
            f,
            "{} {}",
            format!("[{prefix}] {:?}:", self.kind).color(primary).bold(),
            self.title.as_ref().unwrap_or(&String::new()),
        )?;

        let mut padding = String::new();
        if let Some(ref span) = &self.span {
            writeln!(f, " {} {}", "--->".cyan(), self.span.as_ref().unwrap())?;

            padding = format!(
                "{} {} ",
                " ".repeat(span.line_number.to_string().len()),
                "|".cyan().dimmed()
            );

            let Some(line) = Scanner::get(self.span.as_ref().unwrap().filename)
                .lines()
                .nth(self.span.as_ref().unwrap().line_number - 1)
            else {
                return writeln!(
                    f,
                    "{padding}{}",
                    "Could not fetch line.".color(Color::Red).bold()
                );
            };

            let mut mask_iter = self.span_mask.iter().copied().peekable();
            let mut line_out = String::new();
            let mut span_out = String::new();
            let mut line_chars = line.chars().peekable();

            while let Some(char) = line_chars.peek().copied() {
                match mask_iter.next().unwrap_or(HighlightKind::Empty) {
                    HighlightKind::Empty => {
                        span_out.push(' ');
                        line_out.push(char);
                    },
                    HighlightKind::Caret => {
                        span_out.push('^');
                        line_out.push_str(&char.to_string().color(primary).bold().to_string());
                    },
                    HighlightKind::Ghost(c) => {
                        let mut str = String::from(c);
                        span_out.push('^');
                        while let Some(HighlightKind::Ghost(c)) = mask_iter.peek().copied() {
                            span_out.push('^');
                            mask_iter.next();
                            str.push(c);
                        }

                        line_out.push_str(&str.color(Color::Green).bold().to_string());
                        continue;
                    },
                }
                line_chars.next();
            }

            writeln!(f, "{padding}{line_out}")?;

            writeln!(
                f,
                "{padding}{} {}",
                span_out.trim_end().color(primary).bold(),
                self.label.as_ref().unwrap_or(&String::new()).color(secondary),
            )?;
        }

        if let Some(footers) = &self.footers {
            for footer in footers {
                writeln!(f, "{}{}", padding, footer.bright_black().italic())?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Into<(usize, String)> for Report {
    fn into(self) -> (usize, String) {
        (self.level() as usize, self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Box<Report>>;
