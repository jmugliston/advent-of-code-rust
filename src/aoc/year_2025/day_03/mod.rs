use std::error::Error;
use std::fs;

use crate::utils::parsing;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_03/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_03/input/input.txt";

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

pub fn part_1(input: &str) -> i64 {
    let banks = parsing::read_lines_of_digits(input);

    let mut total: i64 = 0;

    for batteries in banks {
        let left = batteries[..batteries.len() - 1].to_vec();

        let max_left = left.iter().max().unwrap();
        let max_index = left.iter().position(|&x| x == *max_left).unwrap();

        let mut right = batteries[max_index + 1..].to_vec();
        right.push(*batteries.last().unwrap());

        let max_right = right.iter().max().unwrap();

        let joined = format!("{}{}", max_left, max_right);
        let joined_value: i64 = joined.parse().unwrap();

        total += joined_value;
    }

    return total;
}

pub fn part_2(input: &str) -> i64 {
    let banks = parsing::read_lines_of_digits(input);

    let mut total: i64 = 0;

    for batteries in banks {
        let mut current_batteries = batteries.clone();
        let mut final_values = vec![];
        let mut offset = 12; // 12 digits to find

        while final_values.len() < 12 {
            offset -= 1;

            // Sliding window to find the max value
            // Check only batteries up to the offset e.g. if offset is 11 and there are 14 batteries, check first 3 batteries
            let to_check =
                current_batteries[..current_batteries.len().saturating_sub(offset)].to_vec();
            let max_value = to_check.iter().max().unwrap();
            let max_index = to_check.iter().position(|&x| x == *max_value).unwrap();

            // Remove the current batteries that have been checked so far
            current_batteries = current_batteries[max_index + 1..].to_vec();

            final_values.push(*max_value);
        }

        let joined: String = final_values.iter().map(|x| x.to_string()).collect();
        let joined_value: i64 = joined.parse().unwrap();
        total += joined_value;
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 357);
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
                assert_eq!(result, 3121910778619);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
