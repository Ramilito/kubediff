use colored::Colorize;
use regex::Regex;

use crate::print::Pretty;
use kubediff::LogLevel;

pub struct Logger {
    log_level: LogLevel,
    term_width: Option<usize>,
}

impl Logger {
    pub fn new(log_level: LogLevel, term_width: Option<usize>) -> Self {
        Logger {
            log_level,
            term_width,
        }
    }

    pub fn log_info(&self, message: String) {
        if self.log_level == LogLevel::Info {
            Pretty::print_info(message, self.term_width)
        };
    }

    pub fn log_warning(&self, message: String) {
        if self.log_level == LogLevel::Warning || self.log_level == LogLevel::Info {
            Pretty::print_warning(message, self.term_width)
        };
    }

    pub fn log_error(&self, message: String) {
        let formatted = format_error_message(&message);
        Pretty::print_error(formatted, self.term_width);
    }
}

fn format_error_message(error_message: &str) -> String {
    let re_error_word = Regex::new(r"\bError\b").unwrap();
    let colored_message = re_error_word.replace_all(error_message, |_: &regex::Captures| {
        "Error".red().to_string()
    });
    format!("{}\n", colored_message)
}
