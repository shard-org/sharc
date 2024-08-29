#![allow(forbidden_lint_groups)]
#![forbid(clippy::complexity, clippy::suspicious, clippy::correctness, clippy::perf, clippy::nursery)] 
#![allow(clippy::style, clippy::restriction, clippy::pedantic, clippy::match_bool, clippy::too_many_lines, clippy::single_match_else, clippy::ignored_unit_patterns, clippy::module_name_repetitions, clippy::needless_for_each, clippy::derive_partial_eq_without_eq, clippy::missing_const_for_fn, clippy::cognitive_complexity, clippy::option_if_let_else, clippy::option_map_unit_fn)]

// TODO: iteraror type for progress bar; then .next() for next step

use std::fmt::Display;
use std::thread;
use std::sync::mpsc;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

enum Event {
    Log(Log),
    TestGELog(usize, Arc<AtomicU8>), // 0 = not done, 1 = done + false, 2 = done + true

    EnableBar(String, ProgressBarKind),
    ProgressBarSet(f64),
    ProgressBarMsg(String),
    ProgressBarSetLen(usize),
    ProgressBarSetPad(usize),
    DisableBar,

    LogBar(String),
    AppendLogBar(String),
    ClearLogBar,

    Terminate,
}

struct Log {
    id:  usize, 
    msg: &'static str,
    llen: usize,
}

struct ProgressBar {
    kind: ProgressBarKind,
    msg:  String,
    pad:  usize,
    len:  usize,
}

pub enum ProgressBarKind {
    Percent(f64),
    Tasks(f64, f64),
    None(f64),
}

const SHARK: &str = "|\\";

fn thread_loop(rx: mpsc::Receiver<Event>) {
    let mut logs: Vec<Log> = Vec::new();
    let mut bar: Option<ProgressBar> = None;
    let mut log_bar: String = String::new();

    let get_bar = |bar: &ProgressBar| match bar.kind {
        ProgressBarKind::Percent(p) => format!(
            "{}{}{}{SHARK}{} {p}%",
            bar.msg, 
            " ".repeat(bar.pad - bar.msg.len()), 
            "_".repeat(((bar.len - 2) as f64 * p) as usize), 
            "_".repeat(bar.len - 2 - ((bar.len - 2) as f64 * p) as usize),
        ),

        ProgressBarKind::Tasks(done, tasks) => format!(
            "{}{}{}{SHARK}{} {done}/{tasks}",
            bar.msg, 
            " ".repeat(bar.pad - bar.msg.len()),
            "_".repeat(((bar.len - 2) as f64 / tasks * done) as usize),
            "_".repeat(bar.len - 2 - ((bar.len - 2) as f64 / tasks * done) as usize),
        ),

        ProgressBarKind::None(t) => format!(
            "{}{}{}{SHARK}{}", 
            bar.msg, 
            " ".repeat(bar.pad - bar.msg.len()), 
            "_".repeat(t as usize), 
            "_".repeat(bar.len - 2 - t as usize)
        ),
    };

    let redraw = |offset: usize, logs: &[Log], bar: Option<&ProgressBar>, log_bar: &str| {
        print!("{}", "\x1B[1A\x1B[2K".repeat(offset));
        io::stdout().flush().unwrap();

        logs.iter().for_each(|log| print!("{}", log.msg));
        print!("{log_bar}");
        bar.map(|bar| println!("{}", get_bar(bar))); 
    };

    let offset = |logs: &[Log], bar_enabled: bool, log_bar: &str|
        logs.iter().map(|log| log.llen).sum::<usize>() 
            + if bar_enabled { 1 } else { 0 }
            + log_bar.matches('\n').count();

    loop {
        match rx.recv().expect("Failed to receive event") {
            Event::Log(log) => {
                let i = logs.iter().rposition(|l| l.id < log.id).map_or(0, |i| i + 1);

                let offset = offset(&logs, bar.is_some(), &log_bar);
                logs.insert(i, log);

                redraw(offset, &logs, bar.as_ref(), &log_bar);
            },

            Event::TestGELog(id, out) => {
                if logs.iter().any(|log| log.id <= id) {
                    out.fetch_add(1, Ordering::Relaxed);
                }
                out.fetch_add(1, Ordering::Relaxed);
            },


            Event::EnableBar(msg, kind) => {
                bar = Some(ProgressBar { 
                    kind, msg, 
                    pad: 10, 
                    len: 20,
                });
                println!("{}", get_bar(bar.as_ref().unwrap())); 
            },

            Event::ProgressBarSet(val) => {
                if let Some(bar) = &mut bar {
                    match bar.kind {
                        ProgressBarKind::Percent(ref mut p)  => *p = val,
                        ProgressBarKind::Tasks(ref mut p, _) => *p = val,
                        ProgressBarKind::None(ref mut t)     => *t = val,
                    }
                    redraw(offset(&logs, true, &log_bar), &logs, Some(bar), &log_bar);
                }
            },

            Event::DisableBar => {
                bar = None;
                redraw(offset(&logs, false, &log_bar), &logs, None, &log_bar);
            },

            Event::ProgressBarMsg(msg) => {
                if let Some(bar) = &mut bar {
                    if msg.len() + 1 > bar.pad {
                        panic!("Message length cannot exceed padding length");
                    }

                    let offset = offset(&logs, true, &log_bar);
                    bar.msg = msg;
                    redraw(offset, &logs, Some(bar), &log_bar);
                }
            },

            Event::ProgressBarSetLen(len) => {
                if let Some(bar) = &mut bar {
                    bar.len = len;
                    redraw(offset(&logs, true, &log_bar), &logs, Some(bar), &log_bar);
                }
            },

            Event::ProgressBarSetPad(pad) => {
                if let Some(bar) = &mut bar {
                    bar.pad = pad;
                    redraw(offset(&logs, true, &log_bar), &logs, Some(bar), &log_bar);
                }
            },


            Event::LogBar(msg) => {
                let offset = offset(&logs, bar.is_some(), &log_bar);
                log_bar = msg;
                redraw(offset, &logs, bar.as_ref(), &log_bar);
            },

            Event::AppendLogBar(msg) => {
                let offset = offset(&logs, bar.is_some(), &log_bar);
                log_bar = log_bar + &msg;
                redraw(offset, &logs, bar.as_ref(), &log_bar);
            },

            Event::ClearLogBar => {
                let offset = offset(&logs, bar.is_some(), &log_bar);
                log_bar = String::new();
                redraw(offset, &logs, bar.as_ref(), &log_bar);
            },

            Event::Terminate => {
                std::mem::drop(rx);
                return;
            },
        }
    }
}


pub struct LogHandler {
    thread: Option<thread::JoinHandle<()>>,
    tx: mpsc::Sender<Event>,
}

impl Clone for LogHandler {
    fn clone(&self) -> Self {
        Self { 
            thread: None, 
            tx: self.tx.clone() 
        }
    }
}

impl LogHandler {
    #[must_use]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let thread = thread::Builder::new()
            .name(String::from("log_handler"))
            .spawn(|| thread_loop(rx))
            .unwrap();
        Self { thread: Some(thread), tx }
    }

    pub fn add_log<T: Display>(&self, priority: usize, log: T) {
        let log = log.to_string();
        let log = Box::leak(log.into_boxed_str());

        let event = Event::Log(Log {
            id: priority,
            llen: log.matches('\n').count(),
            msg: log,
        });

        self.tx.send(event).expect("Failed to send Log event");
    }

    pub fn test_ge_log(&self, id: usize) -> bool {
        let out = Arc::new(AtomicU8::new(0));
        self.tx.send(Event::TestGELog(id, out.clone()))
            .expect("Failed to send TestGELog event");

        while out.load(Ordering::Relaxed) == 0 {
            std::hint::spin_loop();
            std::thread::yield_now(); // ! maybe remove
        }

        match out.load(Ordering::Relaxed) {
            1 => false,
            2 => true,
            _ => unreachable!(),
        }
    }


    pub fn bar<T: Display>(&self, msg: T, kind: ProgressBarKind) {
        self.tx.send(Event::EnableBar(msg.to_string(), kind))
            .expect("Failed to send EnableBar event");
    }

    pub fn set_progress(&self, val: f64) {
        self.tx.send(Event::ProgressBarSet(val))
            .expect("Failed to send ProgressBarSet event");
    }

    pub fn set_bar_msg<T: Display>(&self, msg: T) {
        self.tx.send(Event::ProgressBarMsg(msg.to_string()))
            .expect("Failed to send ProgressBarMsg event");
    }

    pub fn set_bar_len(&self, len: usize) {
        self.tx.send(Event::ProgressBarSetLen(len))
            .expect("Failed to send ProgressBarSetLen event");
    }

    pub fn set_bar_pad(&self, pad: usize) {
        self.tx.send(Event::ProgressBarSetPad(pad))
            .expect("Failed to send ProgressBarSetPad event");
    }

    pub fn disable_bar(&self) {
        self.tx.send(Event::DisableBar)
            .expect("Failed to send DisableBar event");
    }


    pub fn log_bar(&self, msg: String) {
        self.tx.send(Event::LogBar(msg))
            .expect("Failed to send LogBar event");
    }

    pub fn append_log_bar(&self, msg: String) {
        self.tx.send(Event::AppendLogBar(msg))
            .expect("Failed to send AppendLogBar event");
    }

    pub fn clear_log_bar(&self) {
        self.tx.send(Event::ClearLogBar)
            .expect("Failed to send ClearLogBar event");
    }


    pub fn terminate(self) {
        self.tx.send(Event::Terminate)
            .expect("Failed to send Terminate event");
        self.thread
            .expect("Handler not bound to a thread")
            .join().unwrap();
    }
}
