mod commands;
mod enums;
mod logger;
mod print;
mod processor;
mod settings;

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{enums::LogLevel, logger::Logger, processor::Process, settings::Settings};
use clap::Parser;
use colored::Colorize;

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
    log: Option<LogLevel>,
    #[clap(short, long, value_parser)]
    term_width: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let settings = Settings::load().expect("Failed to load config file!");
    let logger = Arc::new(Mutex::new(Logger::new(args.clone(), settings.configs.log)));

    let targets = Process::get_entries(args.clone(), settings);

    for target in targets {
        if Path::new(&target).exists() {
            Process::process_target(args.clone(), &logger, &target)?;
        } else {
            let message = "Must build at directory: not a valid directory ⚠️"
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
