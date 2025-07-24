pub enum CliCommand {
    SetCountChoices(u8),
    ShowHelp,
    Invalid(String),
}

impl CliCommand {
    pub fn handle_cli_args(args: &[String]) -> Result<CliCommand, String> {
        match args[0].as_str() {
            "--count-choices" | "-c" => {
                if args.len() < 2 {
                    return Err("Missing value for --count-choices".to_string());
                }
                args[1].parse()
                    .map(CliCommand::SetCountChoices)
                    .map_err(|_| "Invalid number for count choices".to_string())
            },
            "--help" | "-h" => Ok(CliCommand::ShowHelp),
            other => Ok(CliCommand::Invalid(other.to_string())),
        }
    }
}

