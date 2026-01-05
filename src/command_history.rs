use crate::prelude::AppError;
use std::{collections::{HashMap, HashSet}, env, fs::read_to_string, path::PathBuf};

#[derive(Default)]
pub struct CommandHistory {
    pub commands: Vec<String>,
    unique: bool
}

impl CommandHistory {
    pub fn new() -> Self {
        CommandHistory::default()
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn load(mut self) -> Result<Self, AppError> {
        let history_path = PathBuf::from(env::var("HOME").unwrap_or_default()).join(".bash_history");

        if !history_path.exists() || history_path.metadata()?.len() == 0 {
            return Err(AppError::HistoryLoad("File ~/.bash_history not found or it is empty".into()));
        }

        let mut seen: HashSet<String> = HashSet::new();
        self.commands = read_to_string(&history_path)?
            .lines()
            .rev()
            .filter(|&line| {
                if self.unique {
                    seen.insert(line.to_string())
                } else {
                    true
                }
            })
            .map(|line| line.to_string())
            .collect();

        Ok(self)
    }

    pub fn get_stats(&self) -> Vec<(&str, usize)> {
        //const MIN_OCCURRENCES: usize = 10;
        const TOP_LIMIT: usize = 40;

        let mut frequency_map = HashMap::new();
        for cmd in &self.commands {
            *frequency_map.entry(cmd.as_str()).or_insert(0) += 1;
        }

        let mut frequent_commands: Vec<(&str, usize)> = frequency_map
            .into_iter()
            //.filter(|(_, count)| *count > MIN_OCCURRENCES)
            .collect();

        frequent_commands.sort_by(|a, b| b.1.cmp(&a.1));
        frequent_commands.into_iter().take(TOP_LIMIT).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn history_load() {
        assert!(CommandHistory::new().load().is_ok())
    }
}