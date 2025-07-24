use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub trait Filter {
    fn match_items(&self, cmds: &[String], pattern: &str) -> Vec<(i64, usize)>;
}

pub struct SkimFilter {
    matcher: SkimMatcherV2,
}

impl SkimFilter {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }
}

impl Filter for SkimFilter {
    fn match_items(&self, cmds: &[String], pattern: &str) -> Vec<(i64, usize)> {
        let mut matches: Vec<(i64, usize)> = cmds
            .iter()
            .enumerate()
            .filter_map(|(i, cmd)| {
                self.matcher.fuzzy_match(cmd, pattern)
                    .map(|score| (score, i))
            })
            .collect();
        
        matches.sort_by(|a, b| b.0.cmp(&a.0));
        matches
    }
}