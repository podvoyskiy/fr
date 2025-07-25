#![warn(clippy::all)]

mod interactive;
mod settings;
mod commands;
mod state;
mod filters;
mod prelude { 
    pub use colored::*;
    pub use std::{error::Error, fs::read_to_string, path::PathBuf, env, fs::{File}};
}
use prelude::*;
use settings::{AppConfig};
use commands::{CliCommand};

fn main() -> Result<(), String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //remove program name
    run(args)
}

fn run (args: Vec<String>) -> Result<(), String> {
    let mut config = AppConfig::load().map_err(|e| e.to_string())?;
    
    let filter: Box<dyn filters::Filter> = match config.filter_id {
        1 => Box::new(filters::SubstringFilter),
        _ => Box::new(filters::SkimFilter::new()),
    };

    match args.len() { 
        0 => interactive::run(filter, &config.count_choices).map_err(|e| e.to_string()),
        _ => {
            match CliCommand::handle_cli_args(&args)? {
                CliCommand::SetCountChoices(value) => {
                    config.count_choices = value;
                    config.save().map_err(|e| e.to_string())?;
                    println!("{} {}", "Settings updated: max_choices =".green(), value.to_string().green());
                    Ok(())
                },
                CliCommand::SetCurrentFilter(value) => {
                    config.filter_id = value;
                    config.save().map_err(|e| e.to_string())?;
                    println!("{} {}", "Settings updated: current_filter_id =".green(), value.to_string().green());
                    Ok(())
                },
                CliCommand::ShowHelp => {
                    config.print_help();
                    Ok(())
                },
                CliCommand::Invalid(cmd) => Err(format!("Unknown command: {cmd}. use --help"))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    fn unknown_command() {
        let args: Vec<String> = vec![
            "fake_arg".to_string()
        ];
        assert!(run(args).is_err());
    }

    #[test]
    fn set_correct_count_choices() {
        let args: Vec<String> = vec![
            "-c".to_string(),
            10.to_string(),
        ];
        assert!(run(args).is_ok());
    }

    #[test]
    fn set_incorrect_count_choices() {
        let args: Vec<String> = vec![
            "-c".to_string(),
            256.to_string(),
        ];
        assert!(run(args).is_err());
    }
}