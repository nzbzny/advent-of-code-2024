use std::collections::HashMap;

use crate::utils;

fn create_stones(line: &str) -> Vec<u64> {
    line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let stone_str = stone.to_string();

    if stone_str.len() % 2 == 0 {
        let (first, second) = stone_str.split_at(stone_str.len() / 2);

        vec![first.parse().unwrap(), second.parse().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Memo {
    stone: u64,
    blink_iter: usize,
}

fn blink_with_memo(stones: Vec<u64>, i: usize, blinks: usize, memo: &mut HashMap<Memo, usize>) -> usize {
    println!("handling blink {i}");
    if i == blinks {
        return stones.len();
    }

    let mut result = 0;

    for stone in stones {
        let memo_key = Memo{ stone, blink_iter: i };
        if let Some(val) = memo.get(&memo_key) {
            result += val;
        } else {
            let next_step = blink(stone);
            let val = blink_with_memo(next_step, i + 1, blinks, memo);
            memo.insert(memo_key, val);
            result += val;
        }
    }

    result
}

pub fn run() {
    // let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_11");

    let stones = create_stones(&lines[0]);

    let mut memo = HashMap::new();
    let count = blink_with_memo(stones, 0, 75, &mut memo);

    println!("Stone count is: {count}");
}
