use std::error::Error;
use std::fs;

use crate::utils::parsing;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_01/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_01/input/input.txt";

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
pub fn part_1(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for line in parsing::read_lines(input) {
        let (command, value) = line.split_at(1);
        let value: i32 = value.trim().parse().unwrap_or(0);

        dial = match command {
            "L" => (dial - value).rem_euclid(100),
            _ => (dial + value).rem_euclid(100),
        };

        if dial == 0 {
            password += 1;
        }
    }

    password
}

pub fn part_2(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for line in parsing::read_lines(input) {
        let (command, value) = line.split_at(1);
        let value: i32 = value.trim().parse().unwrap_or(0);

        match command {
            "L" => {
                // Distance to hit 0 moving Left. If at 0, we need a full 100 steps to return to 0.
                let dist_to_0 = if dial == 0 { 100 } else { dial };

                if value >= dist_to_0 {
                    // 1 pass for the first hit, plus 1 for every full 100 steps after that
                    password += 1 + (value - dist_to_0) / 100;
                }
                dial = (dial - value).rem_euclid(100);
            }
            _ => {
                // Moving Right: The number of times we pass 0 is simply how many
                // 100-boundaries we cross in the total absolute value.
                password += (dial + value) / 100;
                dial = (dial + value) % 100;
            }
        }
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 3);
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
                assert_eq!(result, 6);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
