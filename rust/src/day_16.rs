use std::collections::HashSet;

use crate::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq)]
enum Space {
    Empty,
    Box,
}

fn turn_counterclockwise(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    }
}

fn turn_clockwise(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up
    }
}

// grid, start, end
fn create_grid(lines: &[String]) -> (Vec<Vec<Space>>, Position, Position) {
    let mut start = Position{ x: 0, y: 0 };
    let mut end = Position{ x: 0, y: 0 };

    let grid: Vec<Vec<Space>> = lines.iter().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            if c == '#' {
                Space::Box
            } else {
                if c == 'S' {
                    start = Position { x, y };
                }

                if c == 'E' {
                    end = Position { x, y };
                }

                Space::Empty
            }
        }).collect()
    }).collect();

    (grid, start, end)
}

fn try_move(grid: Vec<Vec<Space>>, pos: &Position, direction: Direction) -> Option<Position> {
    let new_pos = match direction {
        Direction::Up => {
            if let Some(y) = pos.y.checked_sub(1) {
                Position{ x: pos.x, y }
            } else {
                return None;
            }
        }
        Direction::Right => {
            let x = pos.x + 1;
            if x < grid[pos.y].len() {
                Position{ x, y: pos.y }
            } else {
                return None;
            }
        }
        Direction::Down => {
            let y = pos.y + 1;
            if y < grid.len() {
                Position{ x: pos.x, y }
            } else {
                return None;
            }
        }
        Direction::Left => {
            if let Some(x) = pos.x.checked_sub(1) {
                Position{ x, y: pos.y }
            } else {
                return None;
            }
        }
    };

    if grid[new_pos.y][new_pos.x] == Space::Empty {
        Some(new_pos)
    } else {
        None
    }
}

fn score_best_path(grid: Vec<Vec<Space>>, pos: &Position, end: &Position, mut seen: HashSet<Position>, direction: Direction, mut current_score: i64) -> i64 {
    if seen.contains(&pos) {
        return -1;
    }

    seen.insert(pos.clone());
    let mut best_score = 0;

    if let Some(new_pos) = try_move(grid, pos, direction) {
        // do stuff
    }
    match try_move(grid, pos, direction) {
        Some(new_pos) => {
            // do stuff

            0
        }
        None => -1
    }
}

pub fn run() {
    let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_16");

    let (grid, start, end) = create_grid(&lines);

}
