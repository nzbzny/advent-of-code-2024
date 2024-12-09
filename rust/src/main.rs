mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod utils;

fn main() {
    match std::env::args().nth(1) {
        Some(s) => match s.parse::<i64>().unwrap() {
            1 => crate::day_1::run(),
            2 => crate::day_2::run(),
            3 => crate::day_3::run(),
            4 => crate::day_4::run(),
            5 => crate::day_5::run(),
            6 => crate::day_6::run(),
            day => println!("Unknown day: {day}"),
        },
        None => {
            println!("No day specified");
        }
    }
}
