use std::collections::{HashMap, HashSet};

use crate::utils;

fn parse_ruleset(rules_str: &[String]) -> HashMap<String, HashSet<String>> {
    let mut ruleset: HashMap<String, HashSet<String>> = HashMap::new();

    for rule in rules_str {
        let (first, other) = rule.split_once('|').unwrap();

        match ruleset.get_mut(first) {
            Some(set) => {
                set.insert(other.to_owned());
            }
            None => {
                ruleset.insert(first.to_owned(), HashSet::from([other.to_owned()]));
            }
        };
    }

    ruleset
}

pub fn run() {
    let lines = utils::parse_file("day_5");
    
    let split_idx = match lines.iter().enumerate().find(|(_, s)| { !s.contains('|') }) {
        Some((idx, _)) => idx,
        None => {
            println!("Could not find split_idx");
            return;
        }
    };

    let (rules_str, printed) = lines.split_at(split_idx);
    let ruleset = parse_ruleset(rules_str);
    let mut sum = 0;

    for manual in printed {
        let mut pages = manual.split(',');
        let mut seen: HashSet<String> = HashSet::new();
        let mut valid = true;
        let mut count = 0;

        pages.clone().for_each(|page| {
            if let Some(rules) = ruleset.get(page) {
                if !rules.is_disjoint(&seen) {
                    valid = false;
                }
            }

            seen.insert(page.to_owned());
            count += 1;
        });

        if valid {
            sum += pages.nth(count / 2).unwrap().parse::<i64>().unwrap();
        }
    }

    println!("Sum is: {sum}");
}
