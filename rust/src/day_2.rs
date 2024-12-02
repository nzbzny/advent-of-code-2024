use crate::utils;

pub fn is_safe_line(nums: &Vec<i64>) -> bool {
    let increasing: bool = nums[1] > nums[0];
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            return false;
        }

        let currently_increasing = nums[i] > nums[i - 1];
        if currently_increasing != increasing {
            return false;
        }

        if (nums[i] - nums[i - 1]).abs() > 3 {
            return false;
        }
    }

    return true;
}

pub fn run() {
    let lines = utils::parse_file("day_2");

    // part 1
    let safe_levels = lines.iter().fold(0, |acc, line| {
        let nums: Vec<i64> = line.split_whitespace().map(|val| val.parse().unwrap()).collect();

        let safe = is_safe_line(&nums);

        if safe {
            acc + 1
        } else {
            acc
        }
    });

    println!("Safe levels: {safe_levels}");

    // part 2
    let dampened_safe_levels = lines.iter().fold(0, |acc, line| {
        let nums: Vec<i64> = line.split_whitespace().map(|val| val.parse().unwrap()).collect();
        
        let mut safe = is_safe_line(&nums);
        if !safe {
            for i in 0..nums.len() {
                let new_nums = nums.iter().take(i).chain(nums.iter().skip(i + 1)).map(&i64::to_owned).collect();
                

                if is_safe_line(&new_nums) {
                    safe = true;
                    break;
                }
            }
        }

        if safe {
            acc + 1
        } else {
            acc
        }
    });

    println!("Dampened safe levels: {dampened_safe_levels}");

}
