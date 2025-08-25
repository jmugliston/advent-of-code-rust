use std::error::Error;
use std::fs;

use crate::utils::grid::{Grid, Point};
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_18/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_18/input/input.txt";

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

fn get_corrupt_points(input: &str) -> Vec<Point> {
    return input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',').map(str::trim);
            Some(Point {
                x: parts.next()?.parse().ok()?,
                y: parts.next()?.parse().ok()?,
            })
        })
        .collect();
}

pub fn part_1(input: &str, example: bool) -> String {
    let (height, width) = if example { (7, 7) } else { (71, 71) };
    let num_falling = if example { 12 } else { 1024 };

    let corrupt_points = get_corrupt_points(input);

    let mut memory_space = Grid::<char>::init(height, width, '.');

    memory_space.set_many(corrupt_points.into_iter().take(num_falling), '#');

    let start = Point { x: 0, y: 0 };

    let end = Point {
        x: (width - 1) as i32,
        y: (height - 1) as i32,
    };

    let path = memory_space.shortest_path(start, end, &'#');

    return (path.len() - 1).to_string();
}

pub fn part_2(input: &str, example: bool) -> String {
    let (height, width) = if example { (7, 7) } else { (71, 71) };
    let num_falling = if example { 12 } else { 1024 };

    let corrupt_points = get_corrupt_points(input);

    let mut memory_space = Grid::<char>::init(height, width, '.');

    memory_space.set_many(corrupt_points.into_iter().take(num_falling), '#');

    let start = Point { x: 0, y: 0 };

    let end = Point {
        x: (width - 1) as i32,
        y: (height - 1) as i32,
    };

    let path = memory_space.shortest_path(start, end, &'#');

    let remaining_corrupt_points: Vec<Point> = get_corrupt_points(input)
        .into_iter()
        .skip(num_falling)
        .collect();

    let mut current_path = path.clone();
    let mut blocking_point = Point { x: 0, y: 0 };

    // Check each falling block to see which one cuts off the exit
    for p in remaining_corrupt_points {
        memory_space.set(&p, '#');
        // Did the next point fall in current path?
        if current_path.contains(&p) {
            // Yes - Check the next shortest path
            current_path = memory_space.shortest_path(start, end, &'#');
            if current_path.len() == 0 {
                // Exit is blocked
                blocking_point = p;
                break;
            }
        }
    }

    return format!("{},{}", blocking_point.x, blocking_point.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input, true);
                assert_eq!(result, "22");
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
                assert_eq!(result, "6,1");
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
