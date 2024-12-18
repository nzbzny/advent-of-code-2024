use crate::utils;

#[derive(Clone, PartialEq)]
enum Space {
    Wall,
    Box,
    Robot,
    Empty
}

struct Position {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn build_map(input: &[&String]) -> (Vec<Vec<Space>>, Position) {
    let mut pos = Position{ x: 0, y: 0 };
    (input.iter().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '#' => Space::Wall,
                '@' => {
                    pos = Position{ x, y };
                    Space::Robot
                }
                'O' => Space::Box,
                _ => Space::Empty
            }
        }).collect()
    }).collect(), pos)
}

fn parse_input(input: &[String]) -> (Vec<Vec<Space>>, Position, String){
    let instructions_idx = input.iter().enumerate().find(|(_, line)| !line.starts_with("#")).unwrap().0;

    let map_str: Vec<&String> = input.iter().take(instructions_idx - 1).collect();
    let instructions: Vec<String> = input.iter().skip(instructions_idx).map(&String::to_owned).collect();

    let (map, robot_position) = build_map(&map_str);
    (map, robot_position, instructions.join(""))
}

fn try_move_object_type(map: &mut Vec<Vec<Space>>, new_pos: &Position, old_pos: &Position, direction: &Direction) -> bool {
    match map[new_pos.y][new_pos.x] {
        Space::Empty => {
            map[new_pos.y][new_pos.x] = map[old_pos.y][old_pos.x].clone();
            map[old_pos.y][old_pos.x] = Space::Empty;
            true
        }
        Space::Wall => {
            false
        }
        Space::Box => {
            if try_move_object(map, &Position{ x: new_pos.x, y: new_pos.y }, direction) {
                map[new_pos.y][new_pos.x] = map[old_pos.y][old_pos.x].clone();
                map[old_pos.y][old_pos.x] = Space::Empty; // TODO 
                true
            } else {
                false
            }
        }
        Space::Robot => {
            println!("Should not have gotten here");
            false
        }
    }
}

fn try_move_object(map: &mut Vec<Vec<Space>>, pos: &Position, direction: &Direction) -> bool {
    match direction {
        Direction::Up => {
            if let Some(y) = pos.y.checked_sub(1) {
                try_move_object_type(map, &Position{ x: pos.x, y }, pos, direction)
            } else {
                false
            }
        }
        Direction::Down => {
            try_move_object_type(map, &Position{ x: pos.x, y: pos.y + 1 }, pos, direction)
        }
        Direction::Left => {
            if let Some(x) = pos.x.checked_sub(1) {
                try_move_object_type(map, &Position{ x, y: pos.y }, pos, direction)
            } else {
                false
            }
        }
        Direction::Right => {
            try_move_object_type(map, &Position{ x: pos.x + 1, y: pos.y }, pos, direction)
        }
    }
}

fn handle_instruction(instruction: char, map: &mut Vec<Vec<Space>>, mut robot_position: Position) -> Position {
    let direction: Direction = match instruction {
        'v' => Direction::Down,
        '^' => Direction::Up,
        '>' => Direction::Right,
        '<' => Direction::Left,
        _ => {
            panic!("Could not process instruction {instruction}");
        }
    };

    try_move_object(map, &robot_position, &direction);

    while map[robot_position.y][robot_position.x] != Space::Robot {
        robot_position = match direction {
            Direction::Up => Position{ x: robot_position.x, y: robot_position.y - 1 },
            Direction::Down => Position{ x: robot_position.x, y: robot_position.y + 1 },
            Direction::Left => Position{ x: robot_position.x - 1, y: robot_position.y },
            Direction::Right => Position{ x: robot_position.x + 1, y: robot_position.y }
        }
    }

    robot_position
}

pub fn run() {
    let lines = utils::parse_file("test");
    // let lines = utils::parse_file("day_15");

    let (mut map, mut robot_position, instructions) = parse_input(&lines);

    for instruction in instructions.chars() {
        robot_position = handle_instruction(instruction, &mut map, robot_position);
    }
}
