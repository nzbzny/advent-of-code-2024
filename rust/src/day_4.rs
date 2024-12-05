use std::str::Chars;

use crate::utils;

fn do_inc(val: usize, inc: i64) -> Option<usize> {
    if inc > 0 {
        val.checked_add(inc.try_into().unwrap())
    } else if inc < 0 {
        val.checked_sub(inc.abs().try_into().unwrap())
    } else {
        Some(val)
    }
}

fn is_xmas_match(
    lines: &Vec<String>, 
    mut line_idx: usize,
    mut char_idx: usize,
    delta_x: i64,
    delta_y: i64,
) -> bool {
    let letters = vec!['X', 'M', 'A', 'S'];
    for i in 1..4 {
        line_idx = match do_inc(line_idx, delta_y) {
            Some(n) => n,
            None => return false
        };
        char_idx = match do_inc(char_idx, delta_x) {
            Some(n) => n,
            None => return false,
        };
    
        let line = match lines.get(line_idx) {
            Some(l) => l,
            None => return false,
        };

        match line.chars().nth(char_idx) {
            Some(c) => {
                if c != letters[i] {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}

fn count_xmas(lines: &Vec<String>) -> u64 {
    let mut count: u64 = 0;

    lines.iter().enumerate().for_each(|(line_idx, line)| {
        line.chars().enumerate().for_each(|(char_idx, c)| {
            if c == 'X' {
                for delta_x in -1..=1 {
                    for delta_y in -1..=1 {
                        if is_xmas_match(lines, line_idx, char_idx, delta_x, delta_y) {
                            count += 1
                        }
                    }
                }
            }
        });
    });

    count
}

fn add_chars_to_strings(
    mut line: Chars<'_>,
    first_idx: usize,
    second_idx: usize,
    first_string: &mut String,
    second_string: &mut String
) -> bool {
    match line.nth(first_idx) {
        Some(c) => first_string.push(c),
        None => return false,
    }
    match line.nth(second_idx - first_idx - 1) {
        Some(c) => second_string.push(c),
        None => return false,
    }

    true
}
fn is_x_mas_match(
    lines: &Vec<String>, 
    line_idx: usize,
    char_idx: usize,
) -> bool {
    let mut forward = String::new();
    let mut backward = String::new();
    let backward_idx = char_idx + 2;

    for i in 0..3 {
        let new_line_idx = match line_idx.checked_add(i) {
            Some(idx) => idx,
            None => return false,
        };

        let mut line = match lines.get(new_line_idx) {
            Some(l) => l.chars(),
            None => return false
        };

        let forward_char_idx = match char_idx.checked_add(i) {
            Some(idx) => idx,
            None => return false
        };
        
        let backward_char_idx = match backward_idx.checked_sub(i) {
            Some(idx) => idx,
            None => return false
        };
        
        if forward_char_idx == backward_char_idx {
            match line.nth(forward_char_idx) {
                Some(c) => {
                    forward.push(c);
                    backward.push(c);
                }
                None => return false
            }
        } else if forward_char_idx < backward_char_idx {
            if !add_chars_to_strings(line, forward_char_idx, backward_char_idx, &mut forward, &mut backward) {
                return false;
            }
        } else {
            if !add_chars_to_strings(line, backward_char_idx, forward_char_idx, &mut backward, &mut forward) {
                return false;
            }
        }
    }

    return (forward == "SAM" || forward == "MAS") && (backward == "SAM" || backward == "MAS") ;
}

fn count_x_mas(lines: &Vec<String>) -> u64 {
    let mut count: u64 = 0;

    lines.iter().enumerate().for_each(|(line_idx, line)| {
        line.chars().enumerate().for_each(|(char_idx, c)| {
            if c == 'M' || c == 'S' {
                if is_x_mas_match(lines, line_idx, char_idx) {
                    count += 1;
                }
            }
        });
    });

    count
}

pub fn run() {
    let lines = utils::parse_file("day_4");

    let xmas_count = count_xmas(&lines);
    println!("xmas count is: {xmas_count}");

    let x_mas_count = count_x_mas(&lines);
    println!("x_mas count is: {x_mas_count}");
}
