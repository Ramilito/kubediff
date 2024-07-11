use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, PartialEq, ValueEnum, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    #[default]
    Error,
}
