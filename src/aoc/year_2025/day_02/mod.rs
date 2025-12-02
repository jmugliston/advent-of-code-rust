use std::error::Error;
use std::fs;

use crate::Part;
use std::collections::HashSet;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_02/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_02/input/input.txt";

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

fn parse_ranges(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| {
            x.split('-')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn solve_range(start: i64, end: i64, part_2: bool) -> Vec<i64> {
    let mut candidates: Vec<i64> = Vec::new();

    let start_length = start.to_string().len();
    let end_length = end.to_string().len();

    for length in start_length..=end_length {
        // Try all possible repeat counts (k) that divide the length, k >= 2
        for k in 2..=length {
            if !part_2 && k != 2 {
                // For part 1, only consider k = 2
                continue;
            }

            if length % k != 0 {
                continue;
            }

            let root_length = length / k;
            let root_start = 10_i64.pow((root_length as u32) - 1);
            let root_end = 10_i64.pow(root_length as u32) - 1;

            for root in root_start..=root_end {
                let root_str = root.to_string();
                let candidate_str = root_str.repeat(k);
                let candidate = match candidate_str.parse::<i64>() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                if candidate >= start && candidate <= end {
                    candidates.push(candidate);
                }
            }
        }
    }

    candidates
}

pub fn part_1(input: &str) -> i64 {
    let ranges: Vec<Vec<i64>> = parse_ranges(input);

    let mut all_candidates: Vec<i64> = Vec::new();

    for range in &ranges {
        let mut range_candidates = solve_range(range[0], range[1], false);
        all_candidates.append(&mut range_candidates);
    }

    let unique_candidates: HashSet<i64> = all_candidates.into_iter().collect();
    unique_candidates.iter().sum::<i64>() as i64
}

pub fn part_2(input: &str) -> i64 {
    let ranges: Vec<Vec<i64>> = parse_ranges(input);

    let mut all_candidates: Vec<i64> = Vec::new();

    for range in &ranges {
        let mut range_candidates = solve_range(range[0], range[1], true);
        all_candidates.append(&mut range_candidates);
    }

    let unique_candidates: HashSet<i64> = all_candidates.into_iter().collect();
    unique_candidates.iter().sum::<i64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 1227775554);
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
                assert_eq!(result, 4174379265);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
