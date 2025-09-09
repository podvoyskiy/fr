use std::{error, fmt, io, num};

#[derive(Debug)]
pub enum AppError {
    Io(String),
    ParseIntError(String),
    HistoryLoad(String),
    SettingsLoad(String),
    IncorrectCommand(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(msg) => write!(f, "IO error: {msg}"),
            AppError::ParseIntError(msg) => write!(f, "ParseIntError error: {msg}"),
            AppError::HistoryLoad(msg) => write!(f, "Failed to load history commands: {msg}"),
            AppError::SettingsLoad(msg) => write!(f, "Failed to load settings: {msg}"),
            AppError::IncorrectCommand(msg) => write!(f, "Incorrect command: {msg}"),
        }
    }
}

impl error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError::Io(value.to_string())
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(value: num::ParseIntError) -> Self {
        AppError::ParseIntError(value.to_string())
    }
}