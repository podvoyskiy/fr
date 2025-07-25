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

pub struct SubstringFilter;

impl Filter for SubstringFilter {
    fn match_items(&self, cmds: &[String], pattern: &str) -> Vec<(i64, usize)> {
        let pattern_lower = pattern.to_lowercase();
        cmds.iter()
            .enumerate()
            .filter_map(|(i, cmd)| {
                let cmd_lower = cmd.to_lowercase();
                cmd_lower.find(&pattern_lower).map(|pos| {
                    let score = 100 - (pos as i64);
                    (score, i)
                })
            })
            .collect()
    }
}