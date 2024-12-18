use std::ops::Neg;

use crate::utils;

#[derive(Clone, Debug)]
struct MatrixRow {
    a: f64,
    b: f64,
    result: f64
}

// aX + bX = result1
// cY + dY = result2
#[derive(Clone, Debug)]
struct Matrix {
    row1: MatrixRow,
    row2: MatrixRow,
}

fn get_value(s: &String, search: &str) -> f64 {
    s.chars()
        .skip(s.find(search).unwrap_or_else(|| panic!("Could not find {search} in {s}")) + search.len())
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse::<f64>()
        .unwrap_or_else(|_| panic!("Failed to parse {s} looking for {search}"))
}

fn create_single_matrix(first: &String, second: &String, result: &String) -> Matrix {
    let mut row1 = MatrixRow{ a: 0.0, b: 0.0, result: 0.0 };
    let mut row2 = MatrixRow{ a: 0.0, b: 0.0, result: 0.0 };
    row1.a = get_value(first, "X+");
    row1.b = get_value(second, "X+");
    row1.result = get_value(result, "X=");

    row2.a = get_value(first, "Y+");
    row2.b = get_value(second, "Y+");
    row2.result = get_value(result, "Y=");

    Matrix{ row1, row2 }
}

fn create_matrices(lines: &[String]) -> Vec<Matrix> {
    let mut idx = 0;
    let mut result: Vec<Matrix> = vec![];

    while idx < lines.len() {
        let first = &lines[idx];
        let second = &lines[idx + 1];
        let results = &lines[idx + 2];
    
        result.push(create_single_matrix(first, second, results));

        idx += 3;
    }

    result
}

fn multiply_row_by_constant(mut row: MatrixRow, c: f64) -> MatrixRow {
    row.a *= c;
    row.b *= c;
    row.result *= c;

    row
}

fn add_row_product_to_row(mut dest_row: MatrixRow, source_row: &MatrixRow, c: f64) -> MatrixRow {
    let multiplied_row = multiply_row_by_constant(source_row.clone(), c);

    dest_row.a += multiplied_row.a;
    dest_row.b += multiplied_row.b;
    dest_row.result += multiplied_row.result;

    dest_row 
}

fn is_within_delta(f: f64, delta: f64) -> bool {
    if f.fract() < delta {
        return true;
    }

    if f.ceil() - f < delta {
        return true;
    }

    false
}

fn gaussian_elimination(matrix: &Matrix) -> Option<(i64, i64)> {
    let mut owned_matrix = matrix.clone();

    let mut c = owned_matrix.row1.a.recip();
    owned_matrix.row1 = multiply_row_by_constant(owned_matrix.row1, c);

    c = owned_matrix.row2.a.neg();
    owned_matrix.row2 = add_row_product_to_row(owned_matrix.row2, &owned_matrix.row1, c);

    c = owned_matrix.row2.b.recip();
    owned_matrix.row2 = multiply_row_by_constant(owned_matrix.row2, c);

    c = owned_matrix.row1.b.neg() / owned_matrix.row2.b;
    owned_matrix.row1 = add_row_product_to_row(owned_matrix.row1, &owned_matrix.row2, c);

    let a_result = owned_matrix.row1.result;
    let b_result = owned_matrix.row2.result;
    
    let delta = 0.01;
    if a_result > 0.0 && b_result > 0.0 && is_within_delta(a_result, delta) && is_within_delta(b_result, delta) {
        #[allow(clippy::cast_possible_truncation)]
        return Some((a_result.round() as i64, b_result.round() as i64));
    }

    None
}

pub fn run() {
    // let lines = utils::parse_file("test");
    let lines = utils::parse_file("day_13");

    let systems = create_matrices(&lines);

    let coins = systems.iter().fold(0, |acc, matrix| {
        if let Some((a, b)) = gaussian_elimination(matrix) {
            acc + (3 * a + b)
        } else {
            acc
        }
    });

    println!("Coin total: {coins}");

    let corrected_systems: Vec<Matrix> = systems.iter().map(|system| {
        let mut new_system = system.clone();
        new_system.row1.result += 10_000_000_000_000.0;
        new_system.row2.result += 10_000_000_000_000.0;

        new_system
    }).collect();

    let corrected_coins = corrected_systems.iter().fold(0, |acc, matrix| {
        if let Some((a, b)) = gaussian_elimination(matrix) {
            acc + (3 * a + b)
        } else {
            acc
        }
    });

    println!("Corrected coin total: {corrected_coins}");
}
