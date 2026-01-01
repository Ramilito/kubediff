use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    #[default]
    Error,
}
