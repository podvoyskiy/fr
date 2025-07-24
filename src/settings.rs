use std::{io::Write};
use crate::prelude::*;

pub struct AppConfig {
    path_to_file: PathBuf,
    pub count_choices: u8,
}

impl AppConfig {
    pub fn load() -> Result<AppConfig, Box<dyn Error>> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
        let config_path = xdg_dirs.place_config_file("config").expect("CRITICAL: cannot create configuration directory");

        if !config_path.exists() {
            let mut config_file = File::create(&config_path)?;
            writeln!(&mut config_file, "count_choices=10")?;
        }

        let content = read_to_string(&config_path)?;

        let mut config = AppConfig {
            path_to_file: config_path,
            count_choices: 0,
        };

        for line in content.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            match parts[0].trim() {
                "count_choices" => config.count_choices = parts[1].trim().parse()?,
                other => return Err(format!("Unknown config key: '{other}'").into()),
            }
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut config_file = File::create(&self.path_to_file)?;
        writeln!(&mut config_file, "count_choices={}", &self.count_choices)?;
        Ok(())
    }

    pub fn print_help(&self) {
        println!("{}{}{}", "Usage:".yellow().bold(), " fr".blue().bold(), " [OPTION]".blue());
        println!("{}", "Options:".yellow().bold());
        println!("{}                Show this help", "  -h, --help".blue().bold());
        println!("{}{}   Set count choices to display (current: {})", "  -c, --count-choices".blue().bold(), " NUM".blue(), self.count_choices);
    }
}