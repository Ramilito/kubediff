// Binary-only modules (not exported from library)
mod logger;
mod print;

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

// Import from the library crate
use kubediff::{KubeClient, LogLevel, Process, Settings};

use crate::{logger::Logger, print::Pretty};
use clap::{Parser, ValueEnum};
use colored::Colorize;

/// CLI-specific LogLevel that implements clap's ValueEnum
#[derive(Default, Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum CliLogLevel {
    Info,
    Warning,
    #[default]
    Error,
}

impl From<CliLogLevel> for LogLevel {
    fn from(cli: CliLogLevel) -> Self {
        match cli {
            CliLogLevel::Info => LogLevel::Info,
            CliLogLevel::Warning => LogLevel::Warning,
            CliLogLevel::Error => LogLevel::Error,
        }
    }
}

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser)]
    env: Option<String>,
    #[clap(short, long, value_parser)]
    inplace: bool,
    #[clap(short, long, value_parser)]
    path: Option<String>,
    #[clap(short, long, value_enum)]
    log: Option<CliLogLevel>,
    #[clap(short, long, value_parser)]
    term_width: Option<usize>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut settings = Settings::load().expect("Failed to load config file!");

    // Determine the effective log level
    let log_level = args
        .log
        .map(LogLevel::from)
        .unwrap_or(settings.configs.log);

    // Create logger with resolved log level
    let logger = Arc::new(Mutex::new(Logger::new(log_level, args.term_width)));

    // Initialize Kubernetes client
    let client = match KubeClient::new().await {
        Ok(c) => c,
        Err(e) => {
            logger
                .lock()
                .unwrap()
                .log_error(format!("Failed to connect to Kubernetes cluster: {}", e));
            return Err(e);
        }
    };

    // Get target paths using library function
    let targets = Process::get_entries(
        args.env.clone(),
        args.inplace,
        args.path.clone(),
        &mut settings,
    );

    for target in targets {
        if Path::new(&target).exists() {
            // Print the path header (CLI-only display)
            Pretty::print_path(format!("Path: {}", target), args.term_width);

            // Use library to get structured results
            let result = Process::process_target(&client, &target).await;

            // Handle build errors
            if let Some(error) = result.build_error {
                logger.lock().unwrap().log_error(error);
                continue;
            }

            // Process and display each diff result
            for diff_result in result.results {
                if let Some(ref diff) = diff_result.diff {
                    // Has changes - print the diff
                    Pretty::print(diff.clone(), Some(&diff_result.resource_name), args.term_width);
                } else if let Some(ref error) = diff_result.error {
                    // Error occurred
                    logger.lock().unwrap().log_error(error.clone());
                } else {
                    // No changes
                    logger.lock().unwrap().log_info(format!(
                        "No changes in: {:?} {:?} {:?}\n",
                        diff_result.api_version, diff_result.kind, diff_result.resource_name
                    ));
                }
            }
        } else {
            let message = "Must build at directory: not a valid directory"
                .yellow()
                .to_string();
            logger
                .lock()
                .unwrap()
                .log_warning(format!("\n{}:{}\n", message, &target))
        }
    }

    Ok(())
}
