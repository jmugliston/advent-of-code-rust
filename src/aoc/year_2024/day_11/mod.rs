use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils::parsing;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_11/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_11/input/input.txt";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone {
    value: i64,
    blinks: i64,
}

fn run_rules(stone: Stone, max_blinks: i64, cache: &mut HashMap<Stone, i64>) -> i64 {
    if stone.blinks == max_blinks {
        return 1;
    }

    if let Some(&cached) = cache.get(&stone) {
        return cached;
    }

    let stone_value_string = stone.value.to_string();

    let result;

    if stone.value == 0 {
        // Rule #1 (is number 0)
        result = run_rules(
            Stone {
                value: 1,
                blinks: stone.blinks + 1,
            },
            max_blinks,
            cache,
        );
    } else if stone_value_string.len() % 2 == 0 {
        // Rule #2 (even number of digits)
        let split_stone_1 = Stone {
            value: stone_value_string[..stone_value_string.len() / 2]
                .parse::<i64>()
                .unwrap(),
            blinks: stone.blinks + 1,
        };
        let split_stone_2 = Stone {
            value: stone_value_string[stone_value_string.len() / 2..]
                .parse::<i64>()
                .unwrap(),
            blinks: stone.blinks + 1,
        };
        result = run_rules(split_stone_1, max_blinks, cache)
            + run_rules(split_stone_2, max_blinks, cache);
    } else {
        // Rule #3 (no other rules apply)
        result = run_rules(
            Stone {
                value: stone.value * 2024,
                blinks: stone.blinks + 1,
            },
            max_blinks,
            cache,
        );
    }

    cache.insert(stone, result);

    return result;
}

pub fn part_1(input: &str) -> i64 {
    let stones = parsing::read_lines_of_numbers(input)[0].clone();

    let mut total = 0;

    for stone in stones {
        let mut cache: HashMap<Stone, i64> = HashMap::new();

        total += run_rules(
            Stone {
                value: stone as i64,
                blinks: 0,
            },
            25,
            &mut cache,
        );
    }

    return total;
}

pub fn part_2(input: &str) -> i64 {
    let stones = parsing::read_lines_of_numbers(input)[0].clone();

    let mut total = 0;

    for stone in stones {
        let mut cache: HashMap<Stone, i64> = HashMap::new();

        total += run_rules(
            Stone {
                value: stone as i64,
                blinks: 0,
            },
            75,
            &mut cache,
        );
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
                assert_eq!(result, 55312);
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
                assert_eq!(result, 65601038650482);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
