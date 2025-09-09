use crate::errors::AppError;

pub enum CliCommand {
    SetCountChoices(u8),
    SetCurrentFilter(u8),
    ShowHelp,
}

impl CliCommand {
    pub fn handle_cli_args(args: &[String]) -> Result<CliCommand, AppError> {
        match args[0].as_str() {
            "--count-choices" | "-c" => {
                if args.len() < 2 {
                    return Err(AppError::IncorrectCommand("Missing value for --count-choices".into()));
                }
                Ok(args[1].parse().map(CliCommand::SetCountChoices)?)
            },
            "--filter" | "-f" => {
                if args.len() < 2 {
                    return Err(AppError::IncorrectCommand("Missing value for --filter".into()));
                }
                let filter_id: u8 = args[1].parse()?;
                match filter_id {
                    1 | 2 => Ok(CliCommand::SetCurrentFilter(filter_id)),
                    _ => Err(AppError::IncorrectCommand("Filter id must be 1 or 2".into())),
                }
            },
            "--help" | "-h" => Ok(CliCommand::ShowHelp),
            other => Err(AppError::IncorrectCommand(format!("Unknown command: {other}. use --help"))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unknown_command() {
        let args: Vec<String> = vec!["fake_arg".to_string()];
        assert!(CliCommand::handle_cli_args(&args).is_err());
    }

    #[test]
    fn correct_count_choices() {
        let args: Vec<String> = vec!["-c".to_string(), 10.to_string()];
        assert!(CliCommand::handle_cli_args(&args).is_ok());
    }

    #[test]
    fn incorrect_count_choices() {
        let args: Vec<String> = vec!["-c".to_string(), 256.to_string()];
        assert!(CliCommand::handle_cli_args(&args).is_err());
    }
}

