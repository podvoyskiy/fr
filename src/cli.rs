use crate::prelude::AppError;

pub enum Command {
    SetMaxResults(u8),
    SetCurrentFilter(u8),
    ShowHelp,
}

impl Command {
    pub fn parse(args: &[String]) -> Result<Command, AppError> {
        match args[0].as_str() {
            "--max_results" | "-m" => {
                if args.len() < 2 {
                    return Err(AppError::IncorrectCommand("Missing value for --max_results".into()));
                }
                Ok(args[1].parse().map(Command::SetMaxResults)?)
            },
            "--filter" | "-f" => {
                if args.len() < 2 {
                    return Err(AppError::IncorrectCommand("Missing value for --filter".into()));
                }
                let filter_id: u8 = args[1].parse()?;
                match filter_id {
                    1 | 2 => Ok(Command::SetCurrentFilter(filter_id)),
                    _ => Err(AppError::IncorrectCommand("Filter id must be 1 or 2".into())),
                }
            },
            "--help" | "-h" => Ok(Command::ShowHelp),
            other => Err(AppError::IncorrectCommand(format!("Unknown command: {other}. use --help"))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unknown_arg() {
        let args: Vec<String> = vec!["fake_arg".to_string()];
        assert!(Command::parse(&args).is_err());
    }

    #[test]
    fn correct_max_results() {
        let args: Vec<String> = vec!["-m".to_string(), 10.to_string()];
        assert!(Command::parse(&args).is_ok());
    }

    #[test]
    fn incorrect_max_results() {
        let args: Vec<String> = vec!["-m".to_string(), 256.to_string()];
        assert!(Command::parse(&args).is_err());
    }
}

