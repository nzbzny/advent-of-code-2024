use std::collections::{HashMap, HashSet};

use crate::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

fn gather_antenna_positions(lines: &[String]) -> HashMap<char, Vec<Position>> {
    lines.iter().enumerate().fold(HashMap::new(), |mut acc: HashMap<char, Vec<Position>>, (y, line)| {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let pos = Position{ x, y };
                match acc.get_mut(&c) {
                    Some(vec) => vec.push(pos),
                    None => { 
                        acc.insert(c, vec![pos]);
                    }
                }
            }
        }

        acc
    })
}

fn build_position(x_o: Option<usize>, y_o: Option<usize>, max_x: usize, max_y: usize) -> Option<Position> {
    match (x_o, y_o) {
        (Some(x), Some(y)) => {
            if x > max_x || y > max_y {
                None
            } else {
                Some(Position{ x, y })
            }
        }
        _ => None
    }
}

enum VerticalDirection {
    Up,
    Down
}
enum HorizontalDirection {
    Left,
    Right
}

fn calculate_antinode_positions(pos1: &Position, pos2: &Position, max_x: usize, max_y: usize) -> (Option<Position>, Option<Position>) {
    let (vertical_direction_to_pos2, diff_y) = if pos1.y > pos2.y {
        (VerticalDirection::Up, pos1.y - pos2.y)
    } else {
        (VerticalDirection::Down, pos2.y - pos1.y)
    };

    let (horizontal_direction_to_pos2, diff_x) = if pos1.x > pos2.x {
        (HorizontalDirection::Left, pos1.x - pos2.x)
    } else {
        (HorizontalDirection::Right, pos2.x - pos1.x)
    };

    let (node1_x, node2_x) = match horizontal_direction_to_pos2 {
        HorizontalDirection::Left => (pos1.x.checked_add(diff_x), pos2.x.checked_sub(diff_x)),
        HorizontalDirection::Right => (pos1.x.checked_sub(diff_x), pos2.x.checked_add(diff_x))
    };

    let (node1_y, node2_y) = match vertical_direction_to_pos2 {
        VerticalDirection::Up => (pos1.y.checked_add(diff_y), pos2.y.checked_sub(diff_y)),
        VerticalDirection::Down => (pos1.y.checked_sub(diff_y), pos2.y.checked_add(diff_y))
    };

    (build_position(node1_x, node1_y, max_x, max_y), build_position(node2_x, node2_y, max_x, max_y))
}

pub fn run() {
    // let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_8");
    
    let positions_map = gather_antenna_positions(&lines);
    let max_x = lines[0].len() - 1;
    let max_y = lines.len() - 1;

    let antinode_positions: HashSet<Position> = positions_map.values().fold(HashSet::new(), |mut acc, vec| {
        for i in 0..vec.len() {
            for j in i+1..vec.len() {
                let (pos1, pos2) = calculate_antinode_positions(&vec[i], &vec[j], max_x, max_y);
                if let Some(p) = pos1 {
                    acc.insert(p);
                }
                if let Some(p) = pos2 {
                    acc.insert(p);
                }
            }
        }

        acc
    });

    println!("Count is: {}", antinode_positions.len());
}
