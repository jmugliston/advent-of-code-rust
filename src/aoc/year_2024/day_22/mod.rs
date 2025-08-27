use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use crate::utils::parsing;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_22/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_22/input/input.txt";

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

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

fn process(mut secret: i64) -> i64 {
    secret = prune(mix(secret, secret * 64));

    secret = prune(mix(secret, secret / 32));

    secret = prune(mix(secret, secret * 2048));

    secret
}

pub fn part_1(input: &str) -> i64 {
    let nums: Vec<i64> = parsing::read_lines(input)
        .iter()
        .map(|x| x.parse::<i64>().expect("Failed to parse number"))
        .collect();

    let mut final_nums = Vec::<i64>::new();
    for num in nums {
        let mut next = num;
        for _i in 0..2000 {
            next = process(next);
        }
        final_nums.push(next);
    }

    let res = final_nums.iter().sum();

    return res;
}

pub fn part_2(input: &str) -> i64 {
    let nums: Vec<i64> = parsing::read_lines(input)
        .iter()
        .map(|x| x.parse::<i64>().expect("Failed to parse number"))
        .collect();

    let mut prices = Vec::<Vec<i64>>::new();
    let mut price_changes = Vec::<Vec<i64>>::new();

    for &num in &nums {
        let mut next = num;
        let mut next_price = vec![next % 10];
        let mut next_price_changes = vec![0];
        for i in 0..2000 {
            let processed = process(next);
            next_price.push(processed % 10);
            next_price_changes.push((processed % 10) - next_price[i]);
            next = processed
        }
        prices.push(next_price);
        price_changes.push(next_price_changes);
    }

    let mut price_map = HashMap::<String, i64>::new();
    for num in 0..nums.len() {
        let mut seen = HashSet::<String>::new();
        let changes = price_changes.get(num as usize).expect("num not found");
        for i in 0..(changes.len() - 4) {
            let window = &changes[i..i + 4];
            let key = format!("{},{},{},{}", window[0], window[1], window[2], window[3]);
            if !seen.contains(&key) {
                seen.insert(key.clone());
                *price_map.entry(key.clone()).or_insert(0) += prices[num as usize][i + 3];
            }
        }
    }

    let res = price_map.values().max().expect("Failed to get max value");

    *res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE_2: &str = "./src/aoc/year_2024/day_22/input/example2.txt";

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 37327623);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }

    #[test]
    fn test_part_2() {
        match fs::read_to_string(EXAMPLE_FILE_2) {
            Ok(input) => {
                let result = part_2(&input);
                assert_eq!(result, 23);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
