#![allow(forbidden_lint_groups)]
#![forbid(
    clippy::complexity,
    clippy::suspicious,
    clippy::correctness,
    clippy::cargo,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(
    clippy::style,
    clippy::restriction,
    clippy::match_bool,
    clippy::too_many_lines,
    clippy::single_match_else,
    clippy::ignored_unit_patterns,
    clippy::module_name_repetitions,
    clippy::needless_for_each,
    clippy::derive_partial_eq_without_eq,
    clippy::missing_const_for_fn,
    clippy::cognitive_complexity,
    clippy::option_if_let_else,
    clippy::option_map_unit_fn
)]
#![allow(dead_code, unused)]

use std::process::exit;
use std::sync::mpsc::Receiver;

use colored::Colorize;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::report::{Level, Report, ReportSender, Unbox};
use crate::scanner::Scanner;

mod args;
mod ast;
mod lexer;
mod parser;
mod preprocessor;
mod report;
mod scanner;
mod span;
mod token;

fn check_reports(receiver: &Receiver<Box<Report>>, reports: &mut Vec<Report>) -> bool {
    let mut had_error = false;
    receiver.try_iter().for_each(|report| {
        if report.level() >= Level::Error {
            had_error = true;
        }
        reports.push(report.unbox());
    });
    had_error
}

fn print_reports_and_exit(reports: &mut Vec<Report>, args: &args::Args) -> ! {
    if *args.level == Level::Silent {
        exit(1);
    }

    reports.sort_by(|left, right| {
        left.level().partial_cmp(&right.level()).expect("Failed to order report kinds.")
    });
    reports.iter().for_each(|report| {
        if *args.level <= report.level() {
            report.display(*args.code_context);
        }
    });

    exit(1)
}

fn main() {
    let args = args::Args::parse(std::env::args().skip(1).collect());

    if *args.debug {
        println!("{args:#?}");
    }

    let mut reports = Vec::<Report>::new();
    let (sender, receiver) = std::sync::mpsc::channel::<Box<Report>>();

    let tokens = {
        let mut lexer =
            Lexer::new(*args.file, Scanner::get(*args.file), ReportSender::new(sender.clone()));
        lexer.lex_tokens();
        lexer.tokens.goto_front();

        if *args.debug {
            println!("\n{}", "LEXER".bold());
            let mut index = 0;
            while let Some(token) = lexer.tokens.get_offset(index) {
                println!("{token:#}");
                index += 1;
            }
        }

        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        }

        lexer.tokens
    };

    let (tokens, tags) = {
        let mut preprocessor =
            preprocessor::PreProcessor::new(*args.file, tokens, ReportSender::new(sender.clone()));

        let (tokens, tags) = preprocessor.process();

        if *args.debug {
            println!("\n{}", "PREPROCESSOR".bold());
            tokens.iter().for_each(|token| println!("{token:#}"));
            println!();
            tags.iter().for_each(|tag| println!("{tag:?}"));
        }

        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        }

        (tokens, tags)
    };

    let program = {
        let mut parser = Parser::new(&args.file, &tokens, ReportSender::new(sender));
        let result = parser.parse();

        if *args.debug {
            println!("\n{}", "PARSER".bold());
            result.stmts.iter().for_each(|stmt| println!("{stmt:#}"));
        }

        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        };
    };
}
