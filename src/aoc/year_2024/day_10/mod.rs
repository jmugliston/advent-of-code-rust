use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils::grid::{parse_number_grid, Grid, Point};
use crate::Part;
use std::collections::VecDeque;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_10/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_10/input/input.txt";

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

fn find_paths(trail_map: &Grid<u32>, start_position: Point, include_all_paths: bool) -> usize {
    let mut visited: HashMap<Point, bool> = HashMap::new();
    let mut possible_paths: Vec<Vec<Point>> = Vec::new();

    type Path = Vec<Point>;
    let mut queue: VecDeque<(Point, Path)> = VecDeque::new();

    queue.push_back((start_position, vec![start_position]));

    while let Some((current_point, mut current_path)) = queue.pop_front() {
        let Some(&current_point_value) = trail_map.get(&current_point) else {
            continue;
        };

        let neighbours = trail_map.neighbours(&current_point, false);

        for neighbour in neighbours {
            let Some(&neighbour_point_value) = trail_map.get(&neighbour) else {
                continue;
            };

            // Check the next point value is valid for the path
            if neighbour_point_value != (current_point_value + 1) {
                continue;
            }

            if !include_all_paths {
                if visited.contains_key(&neighbour) {
                    continue;
                }
                visited.insert(neighbour, true);
            }

            current_path.push(neighbour.clone());

            // If it's a 9 - then we reach the end of the path
            if neighbour_point_value == 9 {
                let finished_path = current_path.clone();
                possible_paths.push(finished_path);
                continue;
            }

            // Queue the next path to check
            queue.push_back((neighbour, current_path.clone()));
        }
    }

    return possible_paths.len();
}

pub fn part_1(input: &str) -> i32 {
    let trail_map = parse_number_grid(input);

    let start_positions = trail_map.find_all(&0);

    let mut total = 0;

    for start in start_positions {
        let paths = find_paths(&trail_map, start, false);
        total += paths
    }

    return total as i32;
}

pub fn part_2(input: &str) -> i32 {
    let trail_map = parse_number_grid(input);

    let start_positions = trail_map.find_all(&0);

    let mut total = 0;

    for start in start_positions {
        let paths = find_paths(&trail_map, start, true);
        total += paths
    }

    return total as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 36);
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
                assert_eq!(result, 81);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
