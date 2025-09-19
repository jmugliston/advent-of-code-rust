use std::error::Error;
use std::fs;

use crate::utils::parsing;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_25/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_25/input/input.txt";

pub fn main(part: Part, example: bool) -> Result<(), Box<dyn Error>> {
    let input_file = if example { EXAMPLE_FILE } else { INPUT_FILE };

    let contents = fs::read_to_string(input_file)?;

    let res = match part {
        Part::One => part_1(&contents),
        _ => 0,
    };

    println!("{}", res);
    Ok(())
}

fn get_locks_and_keys(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let groups = parsing::read_groups_of_lines(input);

    let mut locks = vec![];
    let mut keys = vec![];

    for group in groups {
        let mut next = group.clone();

        let is_lock = group[0].to_string() == "#####";

        if !is_lock {
            next.reverse();
        }

        let mut counts: Vec<i32> = vec![0; 5];
        for row in next.iter().skip(1) {
            for (idx, char) in row.chars().enumerate() {
                if char == '#' {
                    counts[idx] += 1;
                }
            }
        }

        if is_lock {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    }

    (locks, keys)
}

pub fn part_1(input: &str) -> i32 {
    let (locks, keys) = get_locks_and_keys(input);

    let res = locks
        .iter()
        .flat_map(|lock| {
            keys.iter()
                .filter(move |key| (0..5).all(|i| key[i] + lock[i] < 6))
        })
        .count() as i32;

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
                assert_eq!(result, 3);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
