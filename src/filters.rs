use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

#[derive(Clone, Copy)]
pub enum FilterType {
    Skim = 1,
    Substring = 2
}

impl FilterType {
    pub fn default() -> Self {
        FilterType::Skim
    }

    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(FilterType::Skim),
            2 => Some(FilterType::Substring),
            _ => None
        }
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn create_filter(&self) -> Box<dyn Filtering> {
        match self {
            FilterType::Skim => Box::new(SkimFilter::new()),
            FilterType::Substring => Box::new(SubstringFilter)
        }
    }
}

pub trait Filtering {
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

impl Filtering for SkimFilter {
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

impl Filtering for SubstringFilter {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_skim_filter() {
        let filter = SkimFilter::new();
        let cmds = vec!["foo".to_string(), "bar".to_string()];
        let results = filter.match_items(&cmds, "bar");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_substring_filter() {
        let filter = SubstringFilter;
        let cmds = vec![String::from("hello"), String::from("world")];
        let results = filter.match_items(&cmds, "ell");
        assert_eq!(results, vec![(99, 0)]);
    }
}