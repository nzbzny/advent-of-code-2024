use std::collections::HashSet;

use crate::utils;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_next_pos(x_pos: usize, y_pos: usize, direction: &Direction, map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if let Some(y) = y_pos.checked_sub(1) {
                return Some((x_pos, y));
            }
        }
        Direction::Down => {
            if let Some(y) = y_pos.checked_add(1) {
                if y < map.len() {
                    return Some((x_pos, y));
                }
            }
        }
        Direction::Left => {
            if let Some(x) = x_pos.checked_sub(1) {
                return Some((x, y_pos));
            }
        }
        Direction::Right => {
            if let Some(x) = x_pos.checked_add(1) {
                if x < map[y_pos].len() {
                    return Some((x, y_pos));
                }
            } 
        }
    }

    None
}

fn get_next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

pub fn run() {
    let lines = utils::parse_file("day_6");

    let mut direction = Direction::Up;

    let mut x_pos = 0;
    let mut y_pos = 0;
    
    let map: Vec<Vec<char>> = lines.iter().enumerate().map(|(y, line)| {
        if let Some(x) = line.find('^') {
            y_pos = y;
            x_pos = x;
        }

        line.chars().collect()
    }).collect();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((x_pos, y_pos));

    loop {
        let Some((next_x, next_y)) = get_next_pos(x_pos, y_pos, &direction, &map) else { break; };

        if map[next_y][next_x] == '#' {
            direction = get_next_direction(direction);
        } else {
            x_pos = next_x;
            y_pos = next_y;
            visited.insert((x_pos, y_pos));
        }
    }

    println!("Count is: {}", visited.len());
}
