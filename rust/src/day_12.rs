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
            let region = perform_dfs(map, &Position{ x, y }, map[y][x], &mut seen);
            if !region.is_empty() {
                result.push(region);
            }
        }
    }

    result
}

fn is_same_plant(map: &[Vec<char>], pos: &Position, plant: char) -> bool {
    if pos.y >= map.len() || pos.x >= map[0].len() {
        return false;
    }

    map[pos.y][pos.x] == plant
}

fn calculate_region_perimeter(map: &[Vec<char>], region: &[Position]) -> usize {
    let plant = map[region[0].y][region[0].x];
    
    region.iter().fold(0, |mut acc, plot| {
        if !is_same_plant(map, &Position{ x: plot.x, y: plot.y + 1 }, plant) {
            acc += 1;
        }
        if !is_same_plant(map, &Position{ x: plot.x + 1, y: plot.y }, plant) {
            acc += 1;
        }
        if let Some(x) = plot.x.checked_sub(1) {
            if !is_same_plant(map, &Position{ x, y: plot.y }, plant) {
                acc += 1;
            }
        } else {
            acc += 1;
        }
        if let Some(y) = plot.y.checked_sub(1) {
            if !is_same_plant(map, &Position{ x: plot.x, y }, plant) {
                acc += 1;
            }
        } else {
            acc += 1;
        }

        acc
    })
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct LineSegment {
    p1: Position,
    p2: Position,
}

fn get_region_outside_line_segments(region: &[Position]) -> HashSet<LineSegment> {
    let region_set: HashSet<Position> = region.iter().cloned().collect();
    let mut result = HashSet::new();

    for plot in region {
        // right
        if !region_set.contains(&Position{ x: plot.x + 1, y: plot.y }) {
            result.insert(LineSegment { p1: Position { x: plot.x + 1, y: plot.y }, p2: Position { x: plot.x + 1, y: plot.y + 1 } });
        }

        // bottom
        if !region_set.contains(&Position { x: plot.x, y: plot.y + 1 }) {
            result.insert(LineSegment { p1: Position { x: plot.x, y: plot.y + 1 }, p2: Position { x: plot.x + 1, y: plot.y + 1 } });
        }

        // top
        if let Some(y) = plot.y.checked_sub(1) {
            if !region_set.contains(&Position{ x: plot.x, y }) {
                result.insert(LineSegment { p1: Position { x: plot.x, y: plot.y }, p2: Position { x: plot.x + 1, y: plot.y } });
            }
        } else {
            result.insert(LineSegment { p1: Position { x: plot.x, y: plot.y }, p2: Position { x: plot.x + 1, y: plot.y } });
        }

        // left
        if let Some(x) = plot.x.checked_sub(1) {
            if !region_set.contains(&Position{ x, y: plot.y }) {
                result.insert(LineSegment { p1: Position { x: plot.x, y: plot.y }, p2: Position { x: plot.x, y: plot.y + 1 } });
            }
        } else {
            result.insert(LineSegment { p1: Position { x: plot.x, y: plot.y }, p2: Position { x: plot.x, y: plot.y + 1 } });
        }
    }

    result
}

fn contains_vertical_from_point(pos: &Position, edges: &HashSet<LineSegment>) -> bool {
    if edges.contains(&LineSegment{ p1: pos.clone(), p2: Position{ x: pos.x, y: pos.y + 1 }}) {
        true
    } else if let Some(y) = pos.y.checked_sub(1) {
        edges.contains(&LineSegment{ p1: Position{ x: pos.x, y }, p2: pos.clone() })
    } else {
        false
    }
}

fn contains_horizontal_from_point(pos: &Position, edges: &HashSet<LineSegment>) -> bool {
    if edges.contains(&LineSegment{ p1: pos.clone(), p2: Position{ x: pos.x + 1, y: pos.y }}) {
        true
    } else if let Some(x) = pos.x.checked_sub(1) {
        edges.contains(&LineSegment{ p1: Position{ x, y: pos.y }, p2: pos.clone() })
    } else {
        false
    }
}

fn count_region_sides(region: &[Position]) -> usize {
    let edges = get_region_outside_line_segments(region);
    let mut vertices: HashSet<Position> = HashSet::new();

    for edge in &edges {
        // horizontal
        if edge.p1.x == edge.p2.x {
            if contains_horizontal_from_point(&edge.p1, &edges) {
                vertices.insert(edge.p1.clone());
            }

            if contains_horizontal_from_point(&edge.p2, &edges) {
                vertices.insert(edge.p2.clone());
            }
        } else {
            if contains_vertical_from_point(&edge.p1, &edges) {
                vertices.insert(edge.p1.clone());
            }

            if contains_vertical_from_point(&edge.p2, &edges) {
                vertices.insert(edge.p2.clone());
            }
        }
    }
    // println!("{:?}", edges);
    // println!("{:?}", vertices);

    vertices.len()
}

pub fn run() {
    // let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_12");
    
    let map: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect()
    }).collect();

    let regions = identify_regions(&map);

    let price = regions.iter().fold(0, |acc, region| {
        acc + (calculate_region_perimeter(&map, region) * region.len())
    });

    println!("Price is: {price}");

    let discount_price = regions.iter().fold(0, |acc, region| {
        let x = count_region_sides(region);
        let y = region.len();

        // println!("region of {} plants with price {y} * {x} = {}", map[region[0].y][region[0].x], x * y);

        acc + (x * y)
    });

    println!("Discounted price is: {discount_price}");
}
