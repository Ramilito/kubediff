use clap::ArgEnum;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, PartialEq, ArgEnum, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    #[default]
    Error,
}
