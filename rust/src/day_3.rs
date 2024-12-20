use crate::utils;

fn parse_substring(s: &str) -> Option<u64> {
    // valid substring can be at most length 7: 3 digits + , + 3 digits
    if s.len() > 7 {
        return None;
    }

    let (n1, n2) = match s.split_once(',') {
        Some((s1, s2)) => {
            let Ok(n1) = s1.parse::<u64>() else {
                return None;
            };
            let Ok(n2) = s2.parse::<u64>() else {
                return None;
            };

            (n1, n2)
        }
        None => return None,
    };

    Some(n1 * n2)
}

pub fn run() {
    let mut line = utils::parse_file("day_3").join("");

    // part 1
    let mut sum: u64 = 0;
    while let Some((_, suffix)) = line.split_once("mul(") {
        if let Some(idx) = suffix.find(')') {
            if let Some(val) =
                parse_substring(suffix.chars().take(idx).collect::<String>().as_str())
            {
                sum += val;
                line = suffix.chars().skip(idx).collect();
            } else {
                line = suffix.to_string();
            }
        } else {
            break;
        }
    }

    println!("sum is: {sum}");

    // part 2
    line = utils::parse_file("day_3").join("");
    let mut do_sum: u64 = 0;
    let mut doing = true;

    while let Some((prefix, suffix)) = line.split_once("mul(") {
        match (prefix.rfind("do()"), prefix.rfind("don't()")) {
            (Some(do_idx), Some(dont_idx)) => {
                doing = do_idx > dont_idx;
            }
            (Some(_), None) => doing = true,
            (None, Some(_)) => doing = false,
            (None, None) => { /* do nothing */ }
        }

        if let Some(idx) = suffix.find(')') {
            if let Some(val) =
                parse_substring(suffix.chars().take(idx).collect::<String>().as_str())
            {
                if doing {
                    do_sum += val;
                }
                line = suffix.chars().skip(idx).collect();
            } else {
                line = suffix.to_string();
            }
        } else {
            break;
        }
    }

    println!("do_sum is: {do_sum}");
}
