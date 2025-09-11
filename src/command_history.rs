use crate::prelude::{AppError, Filtering};
use std::{collections::HashSet, env, fs::read_to_string, path::PathBuf};

pub struct CommandHistory {
    filter: Box<dyn Filtering>,
    max_results: u8,
    pub commands: Vec<String>,
    pub filtered_indices: Option<Vec<usize>>,
    pub selected_index: usize,
    pub search_query: String,
}

impl CommandHistory {
    pub fn load(filter: Box<dyn Filtering>, &max_results: &u8) -> Result<Self, AppError> {
        let history_path = PathBuf::from(env::var("HOME").unwrap_or_default()).join(".bash_history");

        if !history_path.exists() || history_path.metadata()?.len() == 0 {
            return Err(AppError::HistoryLoad("File ~/.bash_history not found or it is empty".into()));
        }

        let mut seen: HashSet<String> = HashSet::new();
        let commands: Vec<String> = read_to_string(&history_path)?
            .lines()
            .rev()
            .filter(|&line| seen.insert(line.to_string())).map(|line| line.to_string())
            .collect();

        Ok(Self {
            filter,
            max_results,
            commands,
            filtered_indices: None,
            selected_index: 0,
            search_query: String::new(),
        })
    }

    pub fn search(&mut self) {
        self.selected_index = 0;
        if self.search_query.is_empty() {
            self.filtered_indices = None;
            return;
        }
        let matches = self.filter.match_items(&self.commands, &self.search_query);

        self.filtered_indices = Some(
            matches.into_iter()
                .take(self.max_results as usize)
                .map(|(_, idx)| idx)
                .collect()
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::filters::SubstringFilter;

    #[test]
    fn history_load() {
        assert!(CommandHistory::load(Box::new(SubstringFilter), &1).is_ok())
    }
}