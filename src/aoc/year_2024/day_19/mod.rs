use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_19/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_19/input/input.txt";

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

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut sections = input.splitn(2, "\n\n");
    let patterns = sections
        .next()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs = sections
        .next()
        .unwrap_or("")
        .lines()
        .map(str::to_string)
        .collect();
    (patterns, designs)
}

fn match_count(design: String, patterns: &Vec<String>, cache: &mut HashMap<String, i64>) -> i64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(&cached) = cache.get(&design) {
        return cached;
    }

    let mut count = 0;
    for p in patterns {
        if design.starts_with(p) {
            let remaining = design[p.len()..].to_string();
            count += match_count(remaining, patterns, cache);
        }
    }

    cache.insert(design.clone(), count);

    return count;
}

pub fn part_1(input: &str) -> i64 {
    let (patterns, designs) = parse_input(input);

    let mut cache = HashMap::<String, i64>::new();

    let mut count = 0;
    for design in designs {
        if match_count(design, &patterns, &mut cache) > 0 {
            count += 1;
        }
    }

    return count;
}

pub fn part_2(input: &str) -> i64 {
    let (patterns, designs) = parse_input(input);

    let mut cache = HashMap::<String, i64>::new();

    let mut count = 0;
    for design in designs {
        count += match_count(design, &patterns, &mut cache)
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 6);
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
                assert_eq!(result, 16);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
