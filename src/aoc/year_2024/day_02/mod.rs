use std::error::Error;
use std::fs;

use crate::{utils, Part};

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_02/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_02/input/input.txt";

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

fn is_safe(report: &Vec<i32>) -> bool {
    let diffs = report.windows(2).map(|w| w[1] - w[0]);
    let mut pos = true;
    let mut neg = true;
    let mut safe = true;
    for d in diffs {
        if d <= 0 {
            pos = false;
        }
        if d >= 0 {
            neg = false;
        }
        if d.abs() > 3 {
            safe = false;
        }
    }
    (pos || neg) && safe
}

fn is_tolerant(report: &Vec<i32>) -> bool {
    // Get all combinations when a single value is removed
    let combinations: Vec<Vec<i32>> = (0..report.len())
        .map(|i| {
            let mut v = report.clone();
            v.remove(i);
            return v;
        })
        .collect();

    // Are any of the variations safe?
    return combinations.iter().map(is_safe).any(|x| x == true);
}

pub fn part_1(input: &str) -> i32 {
    let nums = utils::parsing::read_lines_of_numbers(&input);

    let checks: Vec<bool> = nums.iter().map(is_safe).collect();

    let count = checks.iter().filter(|&&b| b).count();

    return count as i32;
}

pub fn part_2(input: &str) -> i32 {
    let nums = utils::parsing::read_lines_of_numbers(&input);

    let checks: Vec<bool> = nums.iter().map(is_tolerant).collect();

    let count = checks.iter().filter(|&&b| b).count();

    return count as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 2);
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
                assert_eq!(result, 4);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
