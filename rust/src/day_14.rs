use std::time::Duration;

use crate::utils;

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    x_velocity: i64,
    y_velocity: i64,
}

fn parse_str(s: &str) -> (i64, i64) {
    let vals_str = s.chars().skip(2).collect::<String>();
    let (x, y) = vals_str.split_once(",").expect(&format!("Failed to split , in {vals_str}"));

    (x.parse().unwrap(), y.parse().unwrap())
}

fn create_robot(line: &String) -> Robot {
    let (pos_str, vel_str) = line.split_once(" ").unwrap();
    let (x, y) = parse_str(pos_str);
    let (x_velocity, y_velocity) = parse_str(vel_str);

    Robot{ position: Position{ x, y }, x_velocity, y_velocity }
}

fn create_robots(lines: &[String]) -> Vec<Robot> {
    lines.iter().map(|line| {
        create_robot(line)
    }).collect()
}

fn calculate_final_position(robot: &Robot, x_max: i64, y_max: i64, seconds: i64) -> Position {
    let mut x = (robot.position.x + robot.x_velocity * seconds) % x_max;
    let mut y = (robot.position.y + robot.y_velocity * seconds) % y_max;

    if x < 0 {
        x += x_max;
    }

    if y < 0 {
        y += y_max;
    }

    // println!("{:?} moved to {:?} after {seconds} seconds with x_max={x_max} y_max={y_max}", robot, Position{ x, y });

    Position{ x, y }
}

fn count_robots_in_quadrants(positions: &[Position], x_max: i64, y_max: i64) -> (i64, i64, i64, i64) {
    let left_x_boundary = x_max / 2;
    let right_x_boundary = if x_max % 2 == 0 { left_x_boundary } else { left_x_boundary + 1 };
    let top_y_boundary = y_max / 2;
    let bottom_y_boundary = if y_max % 2 == 0 { top_y_boundary } else { top_y_boundary + 1 };

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    
    // println!("{:?}", positions);
    for position in positions {
        if position.x < left_x_boundary {
            if position.y < top_y_boundary {
                q1 += 1;
            } else if position.y >= bottom_y_boundary {
                q3 += 1;
            }
        } else if position.x >= right_x_boundary {
            if position.y < top_y_boundary {
                q2 += 1;
            } else if position.y >= bottom_y_boundary {
                q4 += 1;
            }
        }
    }

    (q1, q2, q3, q4)
}

fn get_safety_factor(final_positions: &[Position], x_max: i64, y_max: i64) -> i64 {
    let (q1, q2, q3, q4) = count_robots_in_quadrants(final_positions, x_max, y_max);
    // println!("{q1}, {q2}, {q3}, {q4}");

    //TODO: any special handling for an empty quadrant?
    q1 * q2 * q3 * q4
}

fn print_robots(robots: &[Robot], x_max: i64, y_max: i64) {
    let mut grid: Vec<Vec<char>> = vec![];
    grid.resize_with(y_max as usize, || {
        let mut col = vec![];
        col.resize_with(x_max as usize, || { '.' });
        col
    });

    for robot in robots {
        let x = robot.position.x as usize;
        let y = robot.position.y as usize;

        grid[y][x] = 'X';
    }

    for line in grid {
        println!("{:?}", line);
    }
}

fn show_robot_positions_over_time(mut robots: Vec<Robot>, x_max: i64, y_max: i64) {
    let mut iter = 0;
    loop {
        println!("iteration: {iter}");
        print_robots(&robots, x_max, y_max);

        for robot in &mut robots {
            let position = calculate_final_position(&robot, x_max, y_max, 1);

            robot.position = position;
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
}

pub fn run() {
    // let (lines, x_max, y_max, seconds) = (utils::parse_file("test"), 11, 7, 100);
    let (lines, x_max, y_max, seconds) = (utils::parse_file("day_14"), 101, 103, 100);

    let robots = create_robots(&lines);
    let final_positions: Vec<Position> = robots.iter().map(|robot| {
        calculate_final_position(robot, x_max, y_max, seconds)
    }).collect();

    let safety_factor = get_safety_factor(&final_positions, x_max, y_max);
    println!("Safety factor is: {safety_factor}");

    show_robot_positions_over_time(robots, x_max, y_max);
}
