use std::collections::{HashMap, HashSet};

use crate::utils;

fn parse_ruleset(rules_str: &[String]) -> Option<HashMap<String, HashSet<String>>> {
    let mut ruleset: HashMap<String, HashSet<String>> = HashMap::new();

    for rule in rules_str {
        let Some((first, other)) = rule.split_once('|') else { return None };
    }

    Some(ruleset)
}

pub fn run() {
    let lines = utils::parse_file("day_5");
    
    let split_idx = match lines.iter().enumerate().find(|(_, s)| { s.is_empty() }) {
        Some((idx, _)) => idx,
        None => {
            println!("Could not find split_idx");
            return;
        }
    };

    let (rules_str, printed) = lines.split_at(split_idx);
}
