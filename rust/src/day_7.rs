use crate::utils;

struct Equation {
    result: i64,
    values: Vec<i64>
}

fn create_equations(lines: Vec<String>) -> Vec<Equation> {
    lines.iter().map(|line| {
        let (res, vals) = line.split_once(':').expect("Could not split line {line}");
        let result = res.parse::<i64>().unwrap();
        let values = vals.split(' ').map(|val| val.parse::<i64>().unwrap()).collect();

        Equation{ result, values }
    }).collect()
}

fn is_equation_valid(eq: &Equation) -> bool {
    let mut sum = 0;

    for val in &eq.values {

    }

    true
}

pub fn run() {
    // let lines = utils::parse_file("day_7");
    let lines = utils::parse_file("test");

    let equations = create_equations(lines);

    let sum = equations.iter().fold(0, |acc, eq| {
        if is_equation_valid(eq) {
            acc + eq.result
        } else {
            acc
        }
    });
    
    println!("Sum is: {sum}");
}
