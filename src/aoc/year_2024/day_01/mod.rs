use std::error::Error;
use std::{collections::HashMap, fs};

use crate::{utils, Part};

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_01/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_01/input/input.txt";

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
    let nums = utils::parsing::read_numbers(input);

    let mut list_a: Vec<i32> = nums.iter().step_by(2).copied().collect();
    let mut list_b: Vec<i32> = nums.iter().skip(1).step_by(2).copied().collect();

    list_a.sort();
    list_b.sort();

    let res: i32 = list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    return res;
}

pub fn part_2(input: &str) -> i32 {
    let nums = utils::parsing::read_numbers(input);

    let list_a: Vec<i32> = nums.iter().step_by(2).copied().collect();
    let list_b: Vec<i32> = nums.iter().skip(1).step_by(2).copied().collect();

    let occurrences = list_b.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let res: i32 = list_a
        .iter()
        .map(|x| x * occurrences.get(x).unwrap_or(&0))
        .sum();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 11);
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
                assert_eq!(result, 31);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
