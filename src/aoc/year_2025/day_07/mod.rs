use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use crate::utils::grid::{self, Point};
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_07/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_07/input/input.txt";

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

fn run_tachyon_manifold(diagram: &grid::Grid<char>, start: Point) -> (i64, i64) {
    // Track beam positions as it moves south
    let mut beam_positions: HashMap<Point, usize> = HashMap::new();
    beam_positions.insert(start.clone(), 1);

    let mut total_splits = 0;
    let mut world_count = 0;
    loop {
        // Collect updates to apply after iteration
        let mut to_remove = HashSet::new();
        let mut to_add: HashMap<Point, usize> = HashMap::new();

        for (pos, count) in &beam_positions {
            // Move beam south
            let next_pos = pos.next_points_in_direction(grid::Direction::S, 1)[0];

            if !diagram.in_bounds(&next_pos) {
                // Remove the beam if out of bounds
                to_remove.insert(pos.clone());

                // Count how many worlds this beam represented
                world_count += *count;

                continue;
            }

            // Check if next position is a splitter
            if (diagram.get(&next_pos)) == Some(&'^') {
                total_splits += 1;

                // Mark this beam for removal and add two new beams
                to_remove.insert(pos.clone());

                let left_split = next_pos.next_points_in_direction(grid::Direction::E, 1)[0];
                *to_add.entry(left_split).or_insert(0) += *count;

                let right_split = next_pos.next_points_in_direction(grid::Direction::W, 1)[0];
                *to_add.entry(right_split).or_insert(0) += *count;
            } else {
                // Update beam position
                to_remove.insert(pos.clone());
                *to_add.entry(next_pos).or_insert(0) += *count;
            }
        }

        for p in to_remove {
            beam_positions.remove(&p);
        }

        for (p, c) in to_add {
            // Accumulate counts if multiple beams converge
            *beam_positions.entry(p).or_insert(0) += c;
        }

        if beam_positions.is_empty() {
            // All beams have exited the diagram
            break;
        }
    }
    return (total_splits, world_count as i64);
}

pub fn part_1(input: &str) -> i64 {
    let diagram = grid::parse_string_grid(input);

    let start = diagram.find(&'S').unwrap_or(Point::new(0, 0));

    let (total_splits, _) = run_tachyon_manifold(&diagram, start);

    return total_splits;
}

pub fn part_2(input: &str) -> i64 {
    let diagram = grid::parse_string_grid(input);

    let start = diagram.find(&'S').unwrap_or(Point::new(0, 0));

    let (_, world_count) = run_tachyon_manifold(&diagram, start);

    return world_count as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 21);
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
                assert_eq!(result, 40);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
