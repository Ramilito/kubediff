mod commands;
mod enums;
mod logger;
mod print;
mod processor;
mod settings;

use std::sync::{Arc, Mutex};

use crate::{enums::LogLevel, logger::Logger, processor::Process, settings::Settings};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_parser)]
    env: Option<String>,
    #[clap(short, long, value_parser)]
    inplace: bool,
    #[clap(short, long, value_parser)]
    path: Option<String>,
    #[clap(short, long, arg_enum)]
    log: Option<LogLevel>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let settings = Settings::load().expect("Failed to load config file!");
    let logger = Arc::new(Mutex::new(Logger::new(args.log, settings.configs.log)));

    let targets = Process::get_entries(args, settings);

    for target in targets {
        Process::process_target(&logger, &target)?;
    }

    Ok(())
}
