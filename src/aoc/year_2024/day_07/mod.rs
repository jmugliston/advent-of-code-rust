use itertools::Itertools;
use std::error::Error;
use std::fs;

use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_07/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_07/input/input.txt";

pub fn main(part: Part, example: bool) -> Result<(), Box<dyn Error>> {
    let input_file = if example { EXAMPLE_FILE } else { INPUT_FILE };

    let contents = fs::read_to_string(input_file)?;

    let res = match part {
        Part::One => part_1(&contents),
        Part::Two => part_2(&contents),
    };

    println!("{}", res);
    Ok(())
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .filter_map(|line| line.split_once(":"))
        .map(|(res, nums)| {
            let res: i64 = res.trim().parse::<i64>().unwrap_or(0);
            let nums: Vec<i64> = nums
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap_or(0))
                .collect();
            (res, nums)
        })
        .collect()
}

fn is_equation_valid(target: i64, nums: Vec<i64>, operations: &[&str]) -> bool {
    let n_ops = nums.len() - 1;

    let perms = std::iter::repeat(operations.iter().cloned())
        .take(n_ops)
        .multi_cartesian_product();

    // Check each combination for one that works
    for perm in perms {
        let mut total: i64 = nums[0];

        for (i, op) in perm.iter().enumerate() {
            let num = nums[i + 1];
            match *op {
                "+" => total += num,
                "*" => total *= num,
                "||" => total = format!("{}{}", total, num).parse::<i64>().unwrap_or(0),
                _ => (),
            }
        }

        if total == target {
            return true;
        }
    }
    return false;
}

pub fn part_1(input: &str) -> i64 {
    let equations = parse_input(input);

    let operations = ["+", "*"];

    let mut answer = 0;
    for equation in equations {
        let (target, nums) = equation;
        let is_valid = is_equation_valid(target, nums, &operations);
        if is_valid {
            answer += target;
        }
    }

    return answer;
}

pub fn part_2(input: &str) -> i64 {
    let equations = parse_input(input);

    let operations = ["+", "*", "||"];

    let mut answer = 0;
    for equation in equations {
        let (target, nums) = equation;
        let is_valid = is_equation_valid(target, nums, &operations);
        if is_valid {
            answer += target;
        }
    }

    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 3749);
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
                assert_eq!(result, 11387);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
