mod day_1;
mod day_2;
mod utils;

fn main() {
    match std::env::args().nth(1) {
        Some(s) => {
            match s.parse::<i64>().unwrap() {
                1 => crate::day_1::run(),
                2 => crate::day_2::run(),
                day => println!("Unknown day: {day}"),
            }
        }
        None => {
            println!("No day specified");
        }
    }
}

