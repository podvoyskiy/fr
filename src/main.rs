#![warn(clippy::all)]

mod interactive;
mod settings;
mod commands;
mod state;
mod filters;
mod errors;
mod prelude { 
    pub use colored::*;
    pub use std::{fs::read_to_string, path::PathBuf, env, fs::{File}};
    pub use crate::errors::AppError;
}
use prelude::*;
use settings::{AppConfig};
use commands::{CliCommand};

use crate::{filters::FilterType};

fn main () -> Result<(), AppError> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //remove program name

    let mut config = AppConfig::load()?;
    
    let filter = config.filter_type.create_filter();

    match args.len() { 
        0 => interactive::run(filter, &config.count_choices),
        _ => {
            match CliCommand::handle_cli_args(&args)? {
                CliCommand::SetCountChoices(value) => {
                    config.count_choices = value;
                    config.save()?;
                    println!("{} {}", "Settings updated: max_choices =".green(), value.to_string().green());
                    Ok(())
                },
                CliCommand::SetCurrentFilter(value) => {
                    config.filter_type = FilterType::from_id(value).ok_or_else(|| AppError::IncorrectCommand("Invalid filter id".into()))?;
                    config.save()?;
                    println!("{} {}", "Settings updated: current_filter_id =".green(), value.to_string().green());
                    Ok(())
                },
                CliCommand::ShowHelp => {
                    config.print_help();
                    Ok(())
                },
            }
        }
    }
}