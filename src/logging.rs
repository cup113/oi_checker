//! Customized single-threaded terminal logger module based on `console`.
//! It can be embed into a program and change its config.

use console::Style;
use std::{fmt::Display, time::Instant};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Level {
    Trace = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
}

impl Level {
    fn as_value(&self) -> u32 {
        *self as u32
    }

    fn to_str(&self) -> &'static str {
        match self.as_value() {
            1 => "TRACE",
            2 => "INFO",
            3 => "WARNING",
            4 => "ERROR",
            5 => "FATAL",
            _ => unreachable!(),
        }
    }
}

impl From<u32> for Level {
    fn from(v: u32) -> Self {
        match v {
            1 => Level::Trace,
            2 => Level::Info,
            3 => Level::Warning,
            4 => Level::Error,
            5 => Level::Fatal,
            _ => panic!("Level value should between 1 and 5."),
        }
    }
}

/// A single-threaded logging class.
///
/// the length of `content` had better be less than or equal 50.
#[derive(Debug, Clone)]
pub struct Logger {
    start: Instant,
    name: String,
    min_level: Level,
}

#[allow(dead_code)]
impl Logger {
    pub fn new(name: String, min_level: Level) -> Self {
        Self {
            start: Instant::now(),
            name,
            min_level,
        }
    }
    fn print_log<T: Display + ?Sized>(&self, level: &Level, content: &T) {
        if level.as_value() >= self.min_level.as_value() {
            println!(
                "[{2:^7}] {0:>4} {1:^7} {3}",
                self.start.elapsed().as_millis(),
                level.to_str(),
                self.name,
                content
            );
        }
    }
    fn error_log<T: Display + ?Sized>(&self, level: &Level, content: &T) {
        if level.as_value() >= self.min_level.as_value() {
            eprintln!(
                "[{2:^7}] {0:>4} {1:^7} {3}",
                self.start.elapsed().as_millis(),
                level.to_str(),
                self.name,
                content
            );
        }
    }
    #[inline(always)]
    pub fn trace<T: Display + ?Sized>(&self, content: &T) {
        self.print_log(&Level::Trace, &content);
    }
    #[inline(always)]
    pub fn info<T: Display + ?Sized>(&self, content: &T) {
        self.print_log(&Level::Info, &content);
    }
    pub fn warning<T: Display + ?Sized>(&self, content: &T) {
        let s = Style::new().yellow().bold();
        let c = s.apply_to(content);
        self.print_log(&Level::Warning, &c);
    }
    pub fn error<T: Display + ?Sized>(&self, content: &T) {
        let s = Style::new().red().on_white().bold();
        let c = s.apply_to(content);
        self.error_log(&Level::Error, &c);
    }
    pub fn fatal<T: Display + ?Sized>(&self, content: &T) {
        let s = Style::new().red().on_white().bold().underlined();
        let c = s.apply_to(content);
        self.error_log(&Level::Fatal, &c);
    }
}
