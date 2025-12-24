use std::{error, fmt, io, num::ParseIntError};
use colored::*;

pub enum AppError {
    Io(String),
    ParseIntError(String),
    HistoryLoad(String),
    SettingsLoad(String),
    IncorrectCommand(String),
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::Io(msg) => format!("IO error | {msg}").red(),
            AppError::ParseIntError(msg) => format!("ParseIntError error | {msg}").red(),
            AppError::HistoryLoad(msg) => format!("Failed to load history commands | {msg}").red(),
            AppError::SettingsLoad(msg) => format!("Failed to load settings | {msg}").red(),
            AppError::IncorrectCommand(msg) => format!("Incorrect command | {msg}").yellow(),
        };
        write!(f, "{message}")
    }
}

impl error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError::Io(value.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        AppError::ParseIntError(value.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_int_error() {
        let result: Result<i32, AppError> = "not a number".parse().map_err(|e: ParseIntError| e.into());
        assert!(result.is_err());
    }
}