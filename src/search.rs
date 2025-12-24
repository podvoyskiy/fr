use crate::prelude::{AppError, CommandHistory, Filtering};

pub struct Search {
    filter: Box<dyn Filtering>,
    max_results: u8,
    pub commands: Vec<String>,
    pub filtered_indices: Option<Vec<usize>>,
    pub selected_index: usize,
    pub search_query: String,
}

impl Search {
    pub fn init(filter: Box<dyn Filtering>, &max_results: &u8) -> Result<Self, AppError> {
        let command_history = CommandHistory::load(true)?;

        Ok(Self {
            filter,
            max_results,
            commands: command_history.commands,
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