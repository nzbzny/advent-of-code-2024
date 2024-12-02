use std::fs;

fn main() {
    let lines = parse_file();

    let (mut list1, mut list2) = lines.iter().fold((vec![], vec![]), |mut acc, line| {
        let nums: Vec<i64> = line.split_whitespace().map(|val| val.parse().unwrap()).collect();

        acc.0.push(nums[0]);
        acc.1.push(nums[1]);

        acc
    });

    list1.sort();
    list2.sort();

    let mut sum: i64 = 0;
    for pair in list1.iter().zip(list2.iter()) {
        sum += (pair.0 - pair.1).abs();
    }

    println!("Sum is: {sum}");
}

fn parse_file() -> Vec<String> {
    let contents = fs::read_to_string("./src/input.txt").expect("Failed to read file");

    contents.split("\n").map(str::to_string).filter(|line| !line.is_empty()).collect()
}
