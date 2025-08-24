use std::collections::HashSet;
use std::error::Error;
use std::fs;

use crate::utils::grid::{parse_string_grid, Direction, Grid, Point, PointWithDirection};
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_16/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_16/input/input.txt";

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

fn turns_between_directions(from: Direction, to: Direction) -> usize {
    use Direction::*;
    // Assign each direction an index in clockwise order
    let dir_order = [N, E, S, W];
    let from_idx = dir_order.iter().position(|&d| d == from).unwrap();
    let to_idx = dir_order.iter().position(|&d| d == to).unwrap();

    let clockwise = (to_idx + 4 - from_idx) % 4;
    let anticlockwise = (from_idx + 4 - to_idx) % 4;

    clockwise.min(anticlockwise)
}

fn find_best_path(
    race_map: Grid<char>,
    start: PointWithDirection,
    finish: Point,
) -> (Vec<Vec<PointWithDirection>>, usize) {
    let mut queue: std::collections::VecDeque<(Vec<PointWithDirection>, usize)> =
        std::collections::VecDeque::new();

    queue.push_back((vec![start], 0));

    let mut visited: std::collections::HashMap<PointWithDirection, i32> =
        std::collections::HashMap::new();

    visited.insert(start, 0);

    let mut current_best_paths: Vec<Vec<PointWithDirection>> = Vec::new();
    let mut current_best_score: usize = usize::MAX;

    while let Some(current) = queue.pop_front() {
        let (current_path, current_score) = current;

        let current_point = current_path[current_path.len() - 1];

        if current_score > current_best_score {
            // Path score is too high
            continue;
        }

        if current_point.as_point() == finish {
            if current_score < current_best_score {
                // New best score
                current_best_score = current_score;
                // Reset paths
                current_best_paths = Vec::new();
            }
            // Reached the end
            current_best_paths.push(current_path);
            continue;
        }

        for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
            let mut neighbour_point = PointWithDirection {
                direction: dir,
                x: current_point.x,
                y: current_point.y,
            };
            neighbour_point = neighbour_point.next_step();

            if race_map.get(&neighbour_point.as_point()) == Some(&'#') {
                // Hit a wall
                continue;
            }

            let new_score =
                current_score + 1 + (1000 * turns_between_directions(current_point.direction, dir));

            if !visited.contains_key(&neighbour_point)
                || visited.get(&neighbour_point).unwrap() >= &(new_score as i32)
            {
                visited.insert(neighbour_point, new_score as i32);
                let mut new_path = current_path.clone();

                let neighbour = PointWithDirection::new(neighbour_point.x, neighbour_point.y, dir);

                new_path.push(neighbour);

                queue.push_back((new_path, new_score));
            }
        }
    }

    return (current_best_paths, current_best_score);
}

pub fn part_1(input: &str) -> i32 {
    let race_map = parse_string_grid(input);

    let start = race_map.find_all(&'S')[0];
    let finish = race_map.find_all(&'E')[0];

    let (_, best_score) =
        find_best_path(race_map.clone(), start.with_direction(Direction::E), finish);

    return best_score as i32;
}

pub fn part_2(input: &str) -> i32 {
    let race_map = parse_string_grid(input);

    let start = race_map.find_all(&'S')[0];
    let finish = race_map.find_all(&'E')[0];

    let (best_paths, _) =
        find_best_path(race_map.clone(), start.with_direction(Direction::E), finish);

    let mut unique_points: HashSet<Point> = HashSet::new();

    for path in best_paths {
        for next in path {
            unique_points.insert(next.as_point());
        }
    }

    return unique_points.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 11048);
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
                assert_eq!(result, 64);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
