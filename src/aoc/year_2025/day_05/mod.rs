use std::error::Error;
use std::fs;

use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_05/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_05/input/input.txt";

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

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let raw_parts = input.split("\n\n").collect::<Vec<&str>>();

    let ranges = raw_parts[0]
        .lines()
        .map(|line| {
            let nums = line
                .split('-')
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (nums[0], nums[1])
        })
        .collect::<Vec<(i64, i64)>>();

    let numbers = raw_parts[1]
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    return (ranges, numbers);
}

fn merge_ranges(ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged: Vec<(i64, i64)> = Vec::new();

    for range in sorted_ranges {
        if merged.is_empty() {
            merged.push(range);
        } else {
            let last = merged.last_mut().unwrap();
            if range.0 <= last.1 {
                last.1 = last.1.max(range.1);
            } else {
                merged.push(range);
            }
        }
    }

    merged
}

pub fn part_1(input: &str) -> i64 {
    let (ranges, numbers) = parse_input(input);

    let mut fresh_count = 0;
    for num in numbers {
        for (start, end) in &ranges {
            if num >= *start && num <= *end {
                fresh_count += 1;
                break;
            }
        }
    }

    return fresh_count;
}

pub fn part_2(input: &str) -> i64 {
    let (ranges, _) = parse_input(input);

    let merged_ranges = merge_ranges(&ranges);

    let mut total = 0;
    for range in &merged_ranges {
        total += range.1 - range.0 + 1;
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
                assert_eq!(result, 14);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
