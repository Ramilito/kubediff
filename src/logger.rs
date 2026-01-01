use colored::Colorize;

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
        let formatted = message.replace("Error", &"Error".red().to_string());
        Pretty::print_error(format!("{}\n", formatted), self.term_width);
    }
}
