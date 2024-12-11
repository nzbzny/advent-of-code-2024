use std::collections::HashSet;

use crate::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn build_map(lines: &[String]) -> Vec<Vec<u32>> {
    lines.iter().map(|line| {
        line.chars().map(|c| { c.to_digit(10).unwrap()} ).collect()
    }).collect()
}

fn score_dfs(map: &Vec<Vec<u32>>, pos: Position, last_value: u32) -> HashSet<Position> {
    let mut result = HashSet::new();
    if pos.y >= map.len() || pos.x >= map[0].len() {
        return result;
    }

    let val = map[pos.y][pos.x];
    
    if val <= last_value {
        return result;
    }

    if val - last_value > 1 {
        return result;
    }

    if val == 9 {
        result.insert(pos);
        result
    } else {
        run_score_dfs(map, &pos, val)
    }
}

fn run_score_dfs(map: &Vec<Vec<u32>>, pos: &Position, last_value: u32) -> HashSet<Position> {
    let mut result = score_dfs(map, Position{ x: pos.x, y: pos.y + 1}, last_value);
    result.extend(score_dfs(map, Position{ x: pos.x + 1, y: pos.y }, last_value));

    if let Some(y) = pos.y.checked_sub(1) {
        result.extend(score_dfs(map, Position{ x: pos.x, y }, last_value));
    }

    if let Some(x) = pos.x.checked_sub(1) {
        result.extend(score_dfs(map, Position{ x, y: pos.y }, last_value));
    }

    result
}

fn check_and_score_trailhead(map: &Vec<Vec<u32>>, pos: &Position) -> usize {
    if map[pos.y][pos.x] == 0 {
        run_score_dfs(map, pos, 0).len()
    } else {
        0
    }
}

fn rate_dfs(map: &Vec<Vec<u32>>, pos: &Position, last_value: u32) -> u64 {
    if pos.y >= map.len() || pos.x >= map[0].len() {
        return 0;
    }

    let val = map[pos.y][pos.x];
    
    if val <= last_value {
        return 0;
    }

    if val - last_value > 1 {
        return 0;
    }

    if val == 9 {
        1
    } else {
        run_rate_dfs(map, pos, val)
    }
}

fn run_rate_dfs(map: &Vec<Vec<u32>>, pos: &Position, last_value: u32) -> u64 {
    let mut result = rate_dfs(map, &Position{ x: pos.x, y: pos.y + 1}, last_value);
    result += rate_dfs(map, &Position{ x: pos.x + 1, y: pos.y }, last_value);

    if let Some(y) = pos.y.checked_sub(1) {
        result += rate_dfs(map, &Position{ x: pos.x, y }, last_value);
    }

    if let Some(x) = pos.x.checked_sub(1) {
        result += rate_dfs(map, &Position{ x, y: pos.y }, last_value);
    }

    result
}

fn check_and_rate_trailhead(map: &Vec<Vec<u32>>, pos: &Position) -> u64 {
    if map[pos.y][pos.x] == 0 {
        run_rate_dfs(map, pos, 0)
    } else {
        0
    }
}

pub fn run() {
    // let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_10");

    let map = build_map(&lines);

    let score = map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(0, |acc2, (x, _)| {
            acc2 + check_and_score_trailhead(&map, &Position{ x, y })
        })
    });

    println!("Score is: {score}");

    let rating = map.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(0, |acc2, (x, _)| {
            acc2 + check_and_rate_trailhead(&map, &Position{ x, y })
        })
    });

    println!("Rating is: {rating}");
}
