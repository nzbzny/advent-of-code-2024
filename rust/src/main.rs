mod day_1;
mod utils;

fn main() {
    match std::env::args().nth(1) {
        Some(s) => {
            match s.parse::<i64>().unwrap() {
                1 => { crate::day_1::runner::run() }
                day => {
                    println!("Unknown day: {day}");
                }
            }
        }
        None => {
            println!("No day specified");
        }
    }
}

