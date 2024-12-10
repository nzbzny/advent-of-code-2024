use crate::utils;

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>
}

fn create_equations(lines: Vec<String>) -> Vec<Equation> {
    lines.iter().map(|line| {
        let (res, vals) = line.split_once(':').expect("Could not split line {line}");
        let result = res.parse::<u64>().unwrap();
        let values = vals.trim().split(' ').map(|val| val.parse::<u64>().unwrap()).collect();

        Equation{ result, values }
    }).collect()
}

fn is_equation_valid(eq: &Equation, param_idx: usize, curr_val: u64) -> bool {
    if param_idx == eq.values.len() {
        return curr_val == eq.result;
    }

    let add = curr_val + eq.values[param_idx];
    if is_equation_valid(eq, param_idx + 1, add) {
        return true;
    }

    let mult = if param_idx == 0 { eq.values[param_idx] } else { curr_val * eq.values[param_idx] };
    if is_equation_valid(eq, param_idx + 1, mult) {
        return true;
    }
    
    false
}

pub fn run() {
    let lines = utils::parse_file("day_7");
    // let lines = utils::parse_file("test");

    let equations = create_equations(lines);

    let sum = equations.iter().fold(0, |acc, eq| {
        if is_equation_valid(eq, 0, 0) {
            acc + eq.result
        } else {
            acc
        }
    });
    
    println!("Sum is: {sum}");
}
