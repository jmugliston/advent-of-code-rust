use std::error::Error;
use std::fs;

const EXAMPLE_FILE: &str = "./src/year_2024/day_05/input/example.txt";
const INPUT_FILE: &str = "./src/year_2024/day_05/input/input.txt";

pub fn main(part: Option<i32>, example: Option<bool>) -> Result<(), Box<dyn Error>> {
    let input_file = if example.unwrap_or(true) {
        EXAMPLE_FILE
    } else {
        INPUT_FILE
    };

    let contents = fs::read_to_string(input_file)?;

    let res = match part.unwrap_or(1) {
        1 => part_1(&contents),
        _ => part_2(&contents),
    };

    println!("{}", res);
    Ok(())
}

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<(i32, i32)> = sections[0]
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split('|')
                .map(|s| s.trim().parse::<i32>().expect("Invalid number"))
                .collect();
            (nums[0], nums[1])
        })
        .collect();

    let pages = sections[1]
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split(",")
                .map(|s| s.trim().parse::<i32>().expect("Invalid number"))
                .collect();
            return nums;
        })
        .collect();

    return (rules, pages);
}

fn is_sorted(page: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    rules.iter().all(|&(a, b)| {
        let idx_a = page.iter().position(|&x| x == a);
        let idx_b = page.iter().position(|&x| x == b);
        if let (Some(idx_a), Some(idx_b)) = (idx_a, idx_b) {
            idx_a <= idx_b
        } else {
            true
        }
    })
}

fn custom_sort(a: &i32, b: &i32, rules: &Vec<(i32, i32)>) -> std::cmp::Ordering {
    for (rule_a, rule_b) in rules {
        if *a == *rule_a && *b == *rule_b {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Equal;
}

pub fn part_1(input: &str) -> i32 {
    let (rules, pages) = parse(input);

    let correct_count: i32 = pages
        .iter()
        .filter(|p| is_sorted(p, &rules))
        .map(|x| x[x.len() / 2])
        .sum();

    return correct_count as i32;
}

pub fn part_2(input: &str) -> i32 {
    let (rules, pages) = parse(input);

    return pages
        .iter()
        .map(|p| {
            if is_sorted(p, &rules) {
                return 0;
            }
            let mut sorted = p.clone();
            sorted.sort_by(|a, b| custom_sort(a, b, &rules));
            return sorted[sorted.len() / 2];
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 143);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }

    #[test]
    fn test_part_2() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_2(&input);
                assert_eq!(result, 123);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
