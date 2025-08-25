use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils::grid::{self, Point};
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_20/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_20/input/input.txt";

pub fn main(part: Part, example: bool) -> Result<(), Box<dyn Error>> {
    let input_file = if example { EXAMPLE_FILE } else { INPUT_FILE };

    let contents = fs::read_to_string(input_file)?;

    let res = match part {
        Part::One => part_1(&contents, example),
        Part::Two => part_2(&contents, example),
    };

    println!("{}", res);
    Ok(())
}

/// Find all possible cheats from a specific point in the route.
fn can_cheat(current_point: &Point, visited: &HashMap<Point, i32>, max_range: i32) -> Vec<i32> {
    let mut cheats = Vec::new();

    // Check all points within max_range (excluding diagonals if false)
    for p in current_point.in_range(max_range) {
        if let Some(&potential_visited_time) = visited.get(&p) {
            let current_visited_time = visited[current_point];
            let distance = p.manhattan_distance(current_point);

            let time_saved = (potential_visited_time - current_visited_time) - distance;

            if time_saved > 0 {
                cheats.push(time_saved);
            }
        }
    }

    cheats
}

/// Get the number of cheats that meet the time save threshold.
fn get_num_cheats(route: &Vec<Point>, max_cheat_len: i32, time_saved_threshold: i32) -> i32 {
    let mut visited = HashMap::<Point, i32>::new();

    for (idx, p) in route.iter().enumerate() {
        visited.insert(*p, idx as i32);
    }

    let total = route
        .iter()
        .flat_map(|p| can_cheat(p, &visited, max_cheat_len))
        .filter(|&c| c >= time_saved_threshold)
        .count() as i32;

    total
}

pub fn part_1(input: &str, example: bool) -> i32 {
    let time_saved_threshold = if example { 10 } else { 100 };
    let max_cheat_len = 2;

    let race_map = grid::parse_string_grid(input);

    let start = race_map.find(&'S').expect("Start point not found");
    let end = race_map.find(&'E').expect("End point not found");

    let race_route = race_map.shortest_path(start, end, &'#');

    let total_cheats = get_num_cheats(&race_route, max_cheat_len, time_saved_threshold);

    total_cheats
}

pub fn part_2(input: &str, example: bool) -> i32 {
    let time_saved_threshold = if example { 10 } else { 100 };
    let max_cheat_len = 20;

    let race_map = grid::parse_string_grid(input);

    let start = race_map.find(&'S').expect("Start point not found");
    let end = race_map.find(&'E').expect("End point not found");

    let race_route = race_map.shortest_path(start, end, &'#');

    let total_cheats = get_num_cheats(&race_route, max_cheat_len, time_saved_threshold);

    total_cheats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input, true);
                assert_eq!(result, 10);
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
                let result = part_2(&input, true);
                assert_eq!(result, 2268);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
