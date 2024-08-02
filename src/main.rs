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
    clippy::option_map_unit_fn,
)]
#![allow(dead_code, unused)]

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::report::{Level, Report, ReportSender, Unbox};
use crate::scanner::Scanner;
use std::process::exit;
use std::sync::mpsc::Receiver;

use colored::Colorize;

mod args;
mod ast;
mod lexer;
mod parser;
mod preprocessor;
mod report;
mod scanner;
mod span;
mod token;
// mod linked_list;

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

fn print_reports_and_exit(reports: &mut Vec<Report>, args: &args::Args) {
    if *args.level.field == Level::Silent {
        exit(1);
    }

    reports.sort_by(|left, right| {
        left.level().partial_cmp(&right.level()).expect("Failed to order report kinds.")
    });

    reports.iter().for_each(|report| {
        if *args.level.field <= report.level() {
            report.display(*args.code_context.field);
        }
    });

    exit(1);
}

fn main() {
    let args = args::Args::parse(std::env::args().skip(1).collect());

    if *args.debug.field {
        println!("{args:#?}");
    }

    let mut reports = Vec::<Report>::new();
    let (sender, receiver) = std::sync::mpsc::channel::<Box<Report>>();

    let tokens = {
        let mut lexer = Lexer::new(
            &args.file.field,
            Scanner::get_file(&args.file.field),
            ReportSender::new(sender.clone()),
        );

        lexer.lex_tokens();
        if *args.debug.field {
            println!("\n{}", "LEXER".bold());
            lexer.tokens.iter().for_each(|token| println!("{token:#}"));
        }

        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        }

        lexer.tokens
    };

    let (tokens, tags) = {
        let mut preprocessor = preprocessor::PreProcessor::new(
            &args.file.field,
            tokens,
            ReportSender::new(sender.clone()),
        );

        let (tokens, tags) = preprocessor.process();

        if *args.debug.field {
            println!("\n{}", "PREPROCESSOR".bold());
            tokens.iter().for_each(|token| println!("{token:#}"));
            println!("");
            tags.iter().for_each(|tag| println!("{tag:?}"));
        }

        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        }

        (tokens, tags)
    };

    let program = {
        let mut parser = Parser::new(&args.file.field, &tokens, ReportSender::new(sender));
        let result = parser.parse();

        if *args.debug.field {
            println!("\n{}", "PARSER".bold());
            result.stmts.iter().for_each(|stmt| println!("{stmt:#}"));
        }

        if check_reports(&receiver, &mut reports) {
            print_reports_and_exit(&mut reports, &args);
        };
    };
}
