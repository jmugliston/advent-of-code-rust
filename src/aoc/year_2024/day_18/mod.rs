use std::collections::{HashMap, VecDeque};
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

fn shortest_path(memory_space: &Grid<char>, start: Point, end: Point) -> Vec<Point> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut came_from = HashMap::new();

    queue.push_back(start);
    visited.insert(start, true);

    while let Some(current) = queue.pop_front() {
        if current == end {
            // Reconstruct the path
            let mut path = vec![end];
            let mut curr = end;
            while let Some(&prev) = came_from.get(&curr) {
                path.push(prev);
                curr = prev;
            }
            path.reverse();
            return path;
        }

        for p in current.neighbours(false) {
            if p.x < 0
                || p.y < 0
                || p.x >= memory_space.size().0 as i32
                || p.y >= memory_space.size().1 as i32
            {
                continue;
            }

            if visited.contains_key(&p) {
                continue;
            }

            if memory_space.get(&p) == Some(&'#') {
                continue;
            }

            queue.push_back(p);
            visited.insert(p, true);
            came_from.insert(p, current);
        }
    }
    Vec::new() // No path found
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

    let path = shortest_path(&memory_space, start, end);

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

    let path = shortest_path(&memory_space, start, end);

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
            current_path = shortest_path(&memory_space, start, end);
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
