use std::error::Error;
use std::fs;

use crate::utils::grid;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_04/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_04/input/input.txt";

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

fn remove_rolls(warehouse_map: &mut grid::Grid<char>) -> Vec<grid::Point> {
    let mut accessible_roll_points = Vec::new();

    let paper_rolls = warehouse_map.find_all(&'@');

    for p in &paper_rolls {
        let surrounding_points = p.neighbours(true);
        let mut count = 0;
        for sp in &surrounding_points {
            if warehouse_map.get(sp) == Some(&'@') {
                count += 1;
            }
        }
        if count < 4 {
            accessible_roll_points.push(*p);
        }
    }
    return accessible_roll_points;
}

pub fn part_1(input: &str) -> i32 {
    let warehouse_map = grid::parse_string_grid(input);

    let rolls_removed = remove_rolls(&mut warehouse_map.clone());

    let total_removed = rolls_removed.len() as i32;

    return total_removed;
}

pub fn part_2(input: &str) -> i32 {
    let mut warehouse_map = grid::parse_string_grid(input);

    let mut total_removed = 0;

    loop {
        let rolls_removed = remove_rolls(&mut warehouse_map);
        if rolls_removed.is_empty() {
            break;
        }
        for p in &rolls_removed {
            warehouse_map.set(p, '.');
        }
        total_removed += rolls_removed.len() as i32;
    }

    return total_removed;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 13);
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
                assert_eq!(result, 43);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
