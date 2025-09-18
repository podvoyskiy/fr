use crate::prelude::{AppError, FilterType};
use std::{fs::{read_to_string, File}, io::Write, path::PathBuf};
use colored::*;

pub struct AppConfig {
    path_to_file: PathBuf,
    pub max_results: u8,
    pub filter_type: FilterType
}

impl AppConfig {
    fn new() -> Result<Self, AppError> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
        let config_path = xdg_dirs.place_config_file("config")?;

        Ok(Self { 
            path_to_file: config_path,
            max_results: u8::default(),
            filter_type: FilterType::default()
        })
    }

    pub fn load() -> Result<Self, AppError> {
        let mut config = Self::new()?;

        if !config.path_to_file.exists() {
            let mut config_file = File::create(&config.path_to_file)?;
            writeln!(&mut config_file, "max_results=10")?;
            writeln!(&mut config_file, "filter_id=1")?;
        }

        let content = read_to_string(&config.path_to_file)?;

        for line in content.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            match parts[0].trim() {
                "max_results" => config.max_results = parts[1].trim().parse()?,
                "filter_id" => {
                    let id: u8 = parts[1].trim().parse()?;
                    config.filter_type = FilterType::from_id(id).ok_or_else(|| AppError::SettingsLoad("Invalid filter id".into()))?;

                },
                other => return Err(AppError::SettingsLoad(format!("Unknown config key: '{other}'"))),
            }
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), AppError> {
        let mut config_file = File::create(&self.path_to_file)?;
        writeln!(&mut config_file, "max_results={}", &self.max_results)?;
        writeln!(&mut config_file, "filter_id={}", &self.filter_type.id())?;
        Ok(())
    }

    pub fn print_help(&self) {
        println!("{}{}{}", "Usage:".yellow().bold(), " fr".blue().bold(), " [OPTION]".blue());
        println!("{}", "Options:".yellow().bold());
        println!("{}                Show this help", "  -h, --help".blue().bold());
        println!("{}{}     Set maximum number of results to display (current: {})", "  -m, --max_results".blue().bold(), " NUM".blue(), self.max_results);
        println!("{}{}     Set filter algorithm [1 - SkimMatcherV2, 2 - SubstringFilter] (current: {})",
            "  -f, --filter".blue().bold(), "      NUM".blue(), self.filter_type.id());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_load() {
        assert!(AppConfig::load().is_ok())
    }
}