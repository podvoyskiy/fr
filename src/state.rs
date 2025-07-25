use crate::{filters::Filter, prelude::*};
use std::{collections::HashSet};

pub struct State {
    filter: Box<dyn Filter>,
    count_choices: u8,
    pub cmds: Vec<String>,
    pub filtered_indices_cmds: Option<Vec<usize>>,
    pub selected_index_cmd: usize,
    pub current_cmd_mask: String,
}

impl State {
    pub fn load(filter: Box<dyn Filter>, &count_choices: &u8) -> Result<Self, Box<dyn Error>> {
        let history_path = PathBuf::from(env::var("HOME").unwrap_or_default()).join(".bash_history");

        if !history_path.exists() || history_path.metadata()?.len() == 0 {
            return Err("CRITICAL: file ~/.bash_history not found or it is empty".into());
        }

        let mut seen: HashSet<String> = HashSet::new();
        let cmds: Vec<String> = read_to_string(&history_path)?
            .lines()
            .rev()
            .filter(|&line| seen.insert(line.to_string())).map(|line| line.to_string())
            .collect();

        Ok(Self {
            filter,
            count_choices,
            cmds,
            filtered_indices_cmds: None,
            selected_index_cmd: 0,
            current_cmd_mask: String::new(),
        })
    }

    pub fn search(&mut self) {
        self.selected_index_cmd = 0;
        if self.current_cmd_mask.is_empty() {
            self.filtered_indices_cmds = None;
            return;
        }
        let matches = self.filter.match_items(&self.cmds, &self.current_cmd_mask);

        self.filtered_indices_cmds = Some(
            matches.into_iter()
                .take(self.count_choices as usize)
                .map(|(_, idx)| idx)
                .collect()
        );
    }
}