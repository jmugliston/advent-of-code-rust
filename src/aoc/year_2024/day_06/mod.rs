use std::error::Error;
use std::fs;

use crate::{
    utils::grid::{self, Direction, Grid, Point, PointWithDirection},
    Part,
};
use std::collections::HashSet;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_06/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_06/input/input.txt";

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

fn simulate_guard(
    guard_map: Grid<char>,
    start: Point,
    obstacle: Option<Point>,
) -> (HashSet<Point>, bool) {
    let mut is_loop = false;
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut visited_points_with_direction: HashSet<PointWithDirection> = HashSet::new();

    visited_points.insert(start);

    let mut current = PointWithDirection {
        direction: Direction::N,
        x: start.x,
        y: start.y,
    };

    loop {
        if visited_points_with_direction.contains(&current) {
            is_loop = true;
            break;
        }

        visited_points_with_direction.insert(current);

        let next = current.next_step();
        let next_point = next.as_point();

        match guard_map.get(&next_point) {
            None => break, // Left the map
            Some('#') => {
                // Hit an obstacle
                current = current.turn_clockwise(90);
            }
            Some(_) if obstacle == Some(next_point) => {
                // Hit the new obstacle
                current = current.turn_clockwise(90);
            }
            _ => {
                current = next;
            }
        }

        visited_points.insert(current.as_point());
    }

    return (visited_points, is_loop);
}

pub fn part_1(input: &str) -> i32 {
    let mut guard_map = grid::parse_string_grid(input);

    let start = guard_map.find_all(&'^')[0];

    guard_map.set(&start, '.');

    let (visited_points, _) = simulate_guard(guard_map, start, None);

    return visited_points.len() as i32;
}

pub fn part_2(input: &str) -> i32 {
    let mut guard_map = grid::parse_string_grid(input);

    let start = guard_map.find_all(&'^')[0];

    guard_map.set(&start, '.');

    let guard_map_clone = guard_map.clone();
    let (mut potential_obstacles, _) = simulate_guard(guard_map, start, None);

    // Obstacle cannot be put on the start position
    potential_obstacles.remove(&start);

    let total = potential_obstacles
        .iter()
        .map(|obstacle| simulate_guard(guard_map_clone.clone(), start, Some(*obstacle)))
        .filter(|(_, is_loop)| *is_loop == true)
        .count();

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
                assert_eq!(result, 41);
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
                assert_eq!(result, 6);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
