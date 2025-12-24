#![warn(clippy::all)]

mod interactive;
mod config;
mod cli;
mod command_history;
mod filters;
mod errors;
mod search;
mod prelude { 
    pub use crate::config::AppConfig;
    pub use crate::cli::Command;
    pub use crate::filters::{FilterType, Filtering};
    pub use crate::errors::AppError;
    pub use crate::command_history::CommandHistory;
    pub use crate::search::Search;
}
use prelude::*;

use colored::*;

use std::env;

fn main () -> Result<(), AppError> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //remove program name

    let mut config = AppConfig::load()?;
    
    let filter = config.filter_type.create_filter();

    match args.len() { 
        0 => interactive::run(filter, &config.max_results),
        _ => {
            match Command::parse(&args)? {
                Command::SetMaxResults(value) => {
                    config.max_results = value;
                    config.save()?;
                    println!("{} {}", "Settings updated: max_results =".green(), value.to_string().green());
                    Ok(())
                },
                Command::SetCurrentFilter(value) => {
                    config.filter_type = FilterType::from_id(value).ok_or_else(|| AppError::IncorrectCommand("Invalid filter id".into()))?;
                    config.save()?;
                    println!("{} {}", "Settings updated: current_filter_id =".green(), value.to_string().green());
                    Ok(())
                },
                Command::Stats => {
                    config.print_stats()?;
                    Ok(())
                },
                Command::ShowHelp => {
                    config.print_help();
                    Ok(())
                },
            }
        }
    }
}