use crate::utils;

use std::collections::HashMap;

pub fn run() {
    let lines = utils::parse_file(1);

    let (mut left_list, mut right_list) = lines.iter().fold((vec![], vec![]), |mut acc, line| {
        let nums: Vec<i64> = line.split_whitespace().map(|val| val.parse().unwrap()).collect();

        acc.0.push(nums[0]);
        acc.1.push(nums[1]);

        acc
    });

    left_list.sort();
    right_list.sort();

    // part 1
    let mut sum: i64 = 0;
    for pair in left_list.iter().zip(right_list.iter()) {
        sum += (pair.0 - pair.1).abs();
    }

    println!("Sum is: {sum}");

    // part 2
    let mut similarity_score = 0;
    let right_counts: HashMap<i64, i64> = right_list.iter().fold(HashMap::new(), |mut acc, val| {
        match acc.get_mut(val) {
            Some(count) => *count += 1,
            None => {
                acc.insert(*val, 1);
            }
        }

        acc
    });

    left_list.iter().for_each(|val| {
        match right_counts.get(val) {
            Some(count) => similarity_score += val * count,
            None => { /* nothing */ }
        }
    });

    println!("Similarity score is: {similarity_score}");

}
