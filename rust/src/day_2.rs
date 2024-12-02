use crate::utils;

pub fn run() {
    let lines = utils::parse_file("day_2");

    let safe_levels = lines.iter().fold(0, |acc, line| {
        let nums: Vec<i64> = line.split_whitespace().map(|val| val.parse().unwrap()).collect();

        let mut safe: bool = true;
        let increasing: bool = nums[1] > nums[0];
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                safe = false;
                break;
            }

            let currently_increasing = nums[i] > nums[i - 1];
            if currently_increasing != increasing {
                safe = false;
                break;
            }

            if (nums[i] - nums[i - 1]).abs() > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            acc + 1
        } else {
            acc
        }
    });

    println!("Safe levels: {safe_levels}");
}
