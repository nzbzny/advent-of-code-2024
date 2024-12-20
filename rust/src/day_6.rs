use std::collections::HashSet;

use crate::utils;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_next_pos(x_pos: usize, y_pos: usize, direction: Direction, map: &[Vec<char>]) -> Option<(usize, usize)> {
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

fn part_1(start_x: usize, start_y: usize, map: &[Vec<char>]) {
    let mut x_pos = start_x;
    let mut y_pos = start_y;
    let mut direction = Direction::Up;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((x_pos, y_pos));

    loop {
        let Some((next_x, next_y)) = get_next_pos(x_pos, y_pos, direction, map) else { break; };

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

#[derive(Eq, Hash, PartialEq)]
struct RouteComponent {
    x_pos: usize,
    y_pos: usize,
    direction: Direction
}

fn obstruction_would_loop(
    mut x: usize,
    mut y: usize,
    mut direction: Direction,
    map: &[Vec<char>],
    visited: &HashSet<RouteComponent>
) -> bool {
    if visited.contains(&RouteComponent{ x_pos: x, y_pos: y, direction }) {
        return true;
    }

    let mut new_visited: HashSet<RouteComponent> = HashSet::new();
    new_visited.insert(RouteComponent{ x_pos: x, y_pos: y, direction });

    while let Some((x_pos, y_pos)) = get_next_pos(x, y, direction, map) {
        if map[y_pos][x_pos] == '#' {
            direction = get_next_direction(direction);
        } else {
            x = x_pos;
            y = y_pos;
        }

        let rc = RouteComponent{ x_pos, y_pos, direction };
        if visited.contains(&rc) || new_visited.contains(&rc) {
            return true;
        }

        new_visited.insert(RouteComponent{x_pos, y_pos, direction});
    }

    false
}

fn part_2_improved(mut x_pos: usize, mut y_pos: usize, map: &[Vec<char>]) {
    let mut direction = Direction::Up;
    let mut visited: HashSet<RouteComponent> = HashSet::new();
    visited.insert(RouteComponent{ x_pos, y_pos, direction });
    let mut obstructions: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let Some((next_x, next_y)) = get_next_pos(x_pos, y_pos, direction, map) else { break; };

        if map[next_y][next_x] == '#' {
            direction = get_next_direction(direction);
        } else {
            x_pos = next_x;
            y_pos = next_y;
        }

        println!("{x_pos},{y_pos}");
        if obstruction_would_loop(x_pos, y_pos, get_next_direction(direction), map, &visited) {
            if let Some((_, _)) = get_next_pos(x_pos, y_pos, direction, map) {
                obstructions.insert((x_pos, y_pos));
            }
        }

        visited.insert(RouteComponent{ x_pos, y_pos, direction});
    }

    println!("Obstruction count is: {}", obstructions.len());
}

// fn would_object_loop(start_x: usize, start_y: usize, map: &[Vec<char>]) -> bool{
//     let mut x_pos = start_x;
//     let mut y_pos = start_y;
//     let mut direction = Direction::Up;
//     let mut visited: HashSet<RouteComponent> = HashSet::new();
//     visited.insert(RouteComponent{ x_pos, y_pos, direction });

//     loop {
//         let Some((next_x, next_y)) = get_next_pos(x_pos, y_pos, direction, map) else { return false; };

//         if map[next_y][next_x] == '#' {
//             direction = get_next_direction(direction);
//         } else {
//             x_pos = next_x;
//             y_pos = next_y;
//             let rc = RouteComponent{ x_pos, y_pos, direction };

//             if visited.contains(&rc) {
//                 return true;
//             }

//             visited.insert(rc);
//         }
//     }
// }

// fn part_2(start_x: usize, start_y: usize, mut map: Vec<Vec<char>>) {
//     let mut count = 0;

//     for y in 0..map.len() {
//         for x in 0..map[y].len() {
//             if map[y][x] != '#' {
//                 map[y][x] = '#';
//                 if would_object_loop(start_x, start_y, &map) {
//                     count += 1;
//                 }

//                 map[y][x] = '.';
//             }
//         }
//     }

//     println!("Obstruction count is: {count}");
// }

pub fn run() {
    let lines = utils::parse_file("day_6");

    let mut x_pos = 0;
    let mut y_pos = 0;
    
    let map: Vec<Vec<char>> = lines.iter().enumerate().map(|(y, line)| {
        if let Some(x) = line.find('^') {
            y_pos = y;
            x_pos = x;
        }

        line.chars().collect()
    }).collect();

    part_1(x_pos, y_pos, &map);
    part_2_improved(x_pos, y_pos, &map);
    // part_2(x_pos, y_pos, map);
}
