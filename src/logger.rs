use crate::{enums::LogLevel, print::Pretty};

pub fn log(arg_log: Option<LogLevel>, config_log: LogLevel, message: String) {
    match arg_log {
        Some(LogLevel::Info) => Pretty::print_info(message),
        Some(LogLevel::Warning) => println!("warning"),
        Some(LogLevel::Error) => println!("Error"),
        None => match config_log {
            LogLevel::Info => Pretty::print_info(message),
            LogLevel::Warning => println!("config: warning"),
            LogLevel::Error => println!("config error"),
        },
    };
}
