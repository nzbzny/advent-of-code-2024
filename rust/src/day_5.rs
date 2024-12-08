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

    // part 1
    let mut sum = 0;

    let mut invalids: Vec<String> = vec![];
    for manual in printed {
        let mut pages = manual.split(',');
        let mut seen: HashSet<String> = HashSet::new();
        let mut valid = true;
        let mut count = 0;

        pages.clone().for_each(|page| {
            if let Some(rules) = ruleset.get(page) {
                if !rules.is_disjoint(&seen) {
                    valid = false;
                    return;
                }
            }

            seen.insert(page.to_owned());
            count += 1;
        });

        if valid {
            sum += pages.nth(count / 2).unwrap().parse::<i64>().unwrap();
        } else {
            invalids.push(manual.to_string());
        }
    }

    println!("Sum is: {sum}");

    // part 2
    let mut corrected_sum = 0;

    for manual in invalids {
        let mut pages = manual.split(',');
        let mut seen: HashSet<String> = HashSet::new();
        let mut count = 0;

        let mut new_manual: Vec<String> = vec![];

        pages.clone().for_each(|page| {
            if let Some(rules) = ruleset.get(page) {
                let mut intersection_peekable = rules.intersection(&seen).peekable();
                if let Some(p) = intersection_peekable.next() {
                    if let Some((idx, _)) = new_manual.iter().enumerate().find(|(_, s)| *s == p) {
                        new_manual.insert(idx, page.to_string());
                    }
                } else {
                    new_manual.push(page.to_string());
                }
            }

            seen.insert(page.to_owned());
            count += 1;
        });

        check_valid(new_manual.clone(), &ruleset);

        corrected_sum += pages.nth(count / 2).unwrap().parse::<i64>().unwrap();
    }

    println!("Corrected sum is: {corrected_sum}");
}

fn check_valid(pages: Vec<String>, ruleset: &HashMap<String, HashSet<String>>) {
        let mut seen: HashSet<String> = HashSet::new();

        for page in pages.clone() {
            if let Some(rules) = ruleset.get(&page) {
                if !rules.is_disjoint(&seen) {
                    println!("invalid: {:?}", pages);
                    // let int = rules.intersection(&seen).fold("".to_owned(), |acc, v| {
                    //     acc.to_owned() + v
                    // });
                    // println!()
                    return;
                }
            }

            seen.insert(page.to_owned());
        }
}
