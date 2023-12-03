use colored::Colorize;
use regex::Regex;

use crate::{enums::LogLevel, print::Pretty};

pub struct Logger {
    arg_log: Option<LogLevel>,
    config_log: LogLevel,
}

impl Logger {
    pub fn new(arg_log: Option<LogLevel>, config_log: LogLevel) -> Self {
        Logger {
            arg_log,
            config_log,
        }
    }

    pub fn log_info(&self, message: String) {
        let level = self.arg_log.unwrap_or(self.config_log);
        if level == LogLevel::Info {
            Pretty::print_info(message)
        };
    }

    pub fn log_warning(&self, message: String) {
        let level = self.arg_log.unwrap_or(self.config_log);

        if level == LogLevel::Warning || level == LogLevel::Info {
            Pretty::print_info(message)
        };
    }

    pub fn log_error(&self, message: String) {
        let formatted = format_error_message(&message);
        Pretty::print_error(formatted);
    }
}

fn format_error_message(error_message: &str) -> String {
    // Find and colorize the word "Error"
    let re_error_word = Regex::new(r"\bError\b").unwrap();
    let colored_message = re_error_word.replace_all(&error_message, |_: &regex::Captures| {
        "Error".red().to_string()
    });

    format!("{}\n", colored_message)
}
