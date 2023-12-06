use clap::ArgEnum;

#[derive(Default, Debug, Copy, Clone, PartialEq, ArgEnum)]
pub enum LogLevel {
    Info,
    Warning,
    #[default]
    Error,
}
