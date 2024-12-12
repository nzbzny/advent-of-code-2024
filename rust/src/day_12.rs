use std::collections::HashSet;

use crate::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn perform_dfs(map: &Vec<Vec<char>>, pos: &Position, prev: char, seen: &mut HashSet<Position>) -> Vec<Position> {
    if seen.contains(pos) {
        return vec![];
    }

    if pos.y >= map.len() || pos.x >= map[0].len() {
        return vec![];
    }

    if map[pos.y][pos.x] != prev {
        return vec![];
    }

    let mut result = vec![pos.clone()];
    seen.insert(pos.clone());

    result.extend(perform_dfs(map, &Position{ x: pos.x + 1, y: pos.y }, prev, seen));
    result.extend(perform_dfs(map, &Position{ x: pos.x, y: pos.y + 1 }, prev, seen));
    
    if let Some(x) = pos.x.checked_sub(1) {
        result.extend(perform_dfs(map, &Position{ x, y: pos.y }, prev, seen));
    }

    if let Some(y) = pos.y.checked_sub(1) {
        result.extend(perform_dfs(map, &Position{ x: pos.x, y }, prev, seen));
    }

    result
}

fn identify_regions(map: &Vec<Vec<char>>) -> Vec<Vec<Position>>{
    let mut seen: HashSet<Position> = HashSet::new();

    let mut result = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let region = perform_dfs(&map, &Position{ x, y }, map[y][x], &mut seen);
            if region.len() > 0 {
                result.push(region);
            }
        }
    }

    result
}

fn is_same_plant(map: &Vec<Vec<char>>, pos: &Position, plant: char) -> bool {
    if pos.y >= map.len() || pos.x >= map[0].len() {
        return false;
    }

    map[pos.y][pos.x] == plant
}

fn calculate_region_perimeter(map: &Vec<Vec<char>>, regions: &[Position]) -> usize {
    let plant = map[regions[0].y][regions[0].x];
    
    regions.iter().fold(0, |mut acc, region| {
        if !is_same_plant(map, &Position{ x: region.x, y: region.y + 1 }, plant) {
            acc += 1;
        }
        if !is_same_plant(map, &Position{ x: region.x + 1, y: region.y }, plant) {
            acc += 1;
        }
        if let Some(x) = region.x.checked_sub(1) {
            if !is_same_plant(map, &Position{ x, y: region.y }, plant) {
                acc += 1;
            }
        } else {
            acc += 1;
        }
        if let Some(y) = region.y.checked_sub(1) {
            if !is_same_plant(map, &Position{ x: region.x, y }, plant) {
                acc += 1;
            }
        } else {
            acc += 1;
        }

        acc
    })
}

pub fn run() {
    // let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_12");
    
    let map: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect()
    }).collect();

    let regions = identify_regions(&map);

    let price = regions.iter().fold(0, |acc, region| {
        acc + (calculate_region_perimeter(&map, &region) * region.len())
    });

    println!("Price is: {price}");
}
