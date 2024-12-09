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
    
    let Some((split_idx, _)) = lines.iter().enumerate().find(|(_, s)| { !s.contains('|') }) else {
        println!("Could not find split_idx");
        return;
    };
    
    let (rules_str, printed) = lines.split_at(split_idx);
    let ruleset = parse_ruleset(rules_str);

    // part 1
    let mut sum = 0;

    let mut invalids: Vec<String> = vec![];
    for manual in printed {
        let pages: Vec<String> = manual.split(',').map(&str::to_string).collect();
        let mut seen: HashSet<String> = HashSet::new();
        let mut valid = true;

        for page in &pages {
            if let Some(rules) = ruleset.get(page) {
                if !rules.is_disjoint(&seen) {
                    valid = false;
                    break;
                }
            }

            seen.insert(page.to_owned());
        }

        if valid {
            sum += pages[pages.len() / 2].parse::<i64>().unwrap();
        } else {
            invalids.push(manual.to_string());
        }
    }

    println!("Sum is: {sum}");

    // part 2
    let mut corrected_sum = 0;

    for manual in invalids {
        let mut new_manual: Vec<String> = manual.split(',').map(&str::to_string).collect();

        let mut valid = false;

        while !valid {
            valid = rebalance(&mut new_manual, &ruleset);
        }

        corrected_sum += new_manual[new_manual.len() / 2].parse::<i64>().unwrap();
    }

    println!("Corrected sum is: {corrected_sum}");
}

struct RebalanceInfo {
    remove_idx: usize,
    insert_idx: usize,
    page: String,
}

fn rebalance(new_manual: &mut Vec<String>, ruleset: &HashMap<String, HashSet<String>>) -> bool {
    let mut rebalance_info: Option<RebalanceInfo> = None;

    'outer: for (idx, page) in new_manual.iter().enumerate() {
        if let Some(rules) = ruleset.get(page) {
            for (i, _) in new_manual.iter().enumerate().take(idx) {
                if rules.contains(&new_manual[i]) {
                    rebalance_info = Some(RebalanceInfo{remove_idx: idx, insert_idx: i, page: page.to_string()});
                    break 'outer;
                }
            }
        }
    }

    match rebalance_info {
        Some(RebalanceInfo{remove_idx, insert_idx, page}) => {
            new_manual.remove(remove_idx);
            new_manual.insert(insert_idx, page);
            false
        }
        None => true,
    }
}
