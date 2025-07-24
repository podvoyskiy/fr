use crate::prelude::*;
use std::{collections::HashSet};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub struct State {
    matcher: SkimMatcherV2,
    count_choices: u8,
    pub cmds: Vec<String>,
    pub filtered_indices_cmds: Option<Vec<usize>>,
    pub selected_index_cmd: usize,
    pub current_cmd_mask: String,
}

impl State {
    pub fn load(&count_choices: &u8) -> Result<Self, Box<dyn Error>> {
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
            matcher: SkimMatcherV2::default(),
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
        let matches = self.process_matches();

        self.filtered_indices_cmds = Some(
            matches.into_iter()
                .take(self.count_choices as usize)
                .map(|(_, idx)| idx)
                .collect()
        );
    }

    fn process_matches(&self) -> Vec<(i64, usize)> {
        let mut matches: Vec<(i64, usize)> = self.cmds
            .iter()
            .enumerate()
            .filter_map(|(i, cmd)| {
                self.matcher.fuzzy_match(cmd, &self.current_cmd_mask)
                    .map(|score| (score, i))
            })
            .collect();
        
        matches.sort_by(|a, b| b.0.cmp(&a.0));
        matches
    }
}