use std::error::Error;
use std::fs;

use crate::utils::grid;
use grid::Direction;
use Direction::*;

const EXAMPLE_FILE: &str = "./src/year_2024/day_04/input/example.txt";
const INPUT_FILE: &str = "./src/year_2024/day_04/input/input.txt";

pub fn main(part: Option<i32>, example: Option<bool>) -> Result<(), Box<dyn Error>> {
    let input_file = if example.unwrap_or(true) {
        EXAMPLE_FILE
    } else {
        INPUT_FILE
    };

    let contents = fs::read_to_string(input_file)?;

    let res = match part.unwrap_or(1) {
        1 => part_1(&contents),
        _ => part_2(&contents),
    };

    println!("{}", res);
    Ok(())
}

pub fn part_1(input: &str) -> i32 {
    let wordsearch = grid::parse_string_grid(input);

    let x_points = wordsearch.find_all(&'X');

    let mut count = 0;

    let directions = [N, NE, E, SE, S, SW, W, NW];

    for point in &x_points {
        for &dir in &directions {
            let word: String = point
                .next_points_in_direction(dir, 3)
                .iter()
                .filter_map(|p| wordsearch.get(p).copied())
                .collect();
            if word == "MAS" {
                count += 1;
            }
        }
    }

    return count;
}

pub fn part_2(input: &str) -> i32 {
    let wordsearch = grid::parse_string_grid(input);

    wordsearch
        .find_all(&'A')
        .iter()
        .filter(|point| {
            let get_char = |dir| {
                point
                    .next_points_in_direction(dir, 1)
                    .get(0)
                    .and_then(|p| wordsearch.get(p))
                    .copied()
                    .unwrap_or('_')
            };

            let nw = get_char(NW);
            let ne = get_char(NE);
            let sw = get_char(SW);
            let se = get_char(SE);

            let is_pair = |a, b| (a == 'M' && b == 'S') || (a == 'S' && b == 'M');

            is_pair(nw, se) && is_pair(ne, sw)
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 18);
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
                assert_eq!(result, 9);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
