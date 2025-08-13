use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils::grid::{self, parse_string_grid, Grid, Point};
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_08/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_08/input/input.txt";

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

fn get_antenna_locations(antenna_map: Grid<char>) -> HashMap<String, Vec<grid::Point>> {
    let mut locations: HashMap<String, Vec<grid::Point>> = HashMap::new();

    for (i, row) in antenna_map.data.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '.' {
                let key = cell.to_string();
                let point = grid::Point {
                    x: j as i32,
                    y: i as i32,
                };
                locations.entry(key).or_insert_with(Vec::new).push(point);
            }
        }
    }

    return locations;
}

fn get_antenna_pairs(
    locations: HashMap<String, Vec<grid::Point>>,
) -> HashMap<String, Vec<(grid::Point, grid::Point)>> {
    let mut antenna_pairs_map: HashMap<String, Vec<(grid::Point, grid::Point)>> = HashMap::new();

    for (antenna, locations) in locations {
        let mut pairs = Vec::new();
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                pairs.push((locations[i], locations[j]));
            }
        }
        antenna_pairs_map.insert(antenna, pairs);
    }

    return antenna_pairs_map;
}

fn get_antinodes(
    point_a: grid::Point,
    point_b: grid::Point,
    grid_size: i32,
    include_all: bool,
) -> Vec<grid::Point> {
    let mut antinodes: Vec<grid::Point> = Vec::new();

    if include_all {
        // Add the antenna positions
        antinodes.push(point_a);
        antinodes.push(point_b);
    }

    let dx = point_b.x - point_a.x;
    let dy = point_b.y - point_a.y;

    let mut i = 1;
    loop {
        let next_dx = dx * i;
        let next_dy = dy * i;

        let diagonal_point_a = Point::new(point_a.x - next_dx, point_a.y - next_dy);
        let diagonal_point_b = Point::new(point_b.x + next_dx, point_b.y + next_dy);

        let mut in_bounds_count = 0;

        if diagonal_point_a.x >= 0
            && diagonal_point_a.x < grid_size
            && diagonal_point_a.y >= 0
            && diagonal_point_a.y < grid_size
        {
            antinodes.push(diagonal_point_a);
            in_bounds_count += 1
        }

        if diagonal_point_b.x >= 0
            && diagonal_point_b.x < grid_size
            && diagonal_point_b.y >= 0
            && diagonal_point_b.y < grid_size
        {
            antinodes.push(diagonal_point_b);
            in_bounds_count += 1;
        }

        i += 1;

        if !include_all || in_bounds_count == 0 {
            break;
        }
    }

    return antinodes;
}

pub fn part_1(input: &str) -> i32 {
    let antenna_map = parse_string_grid(input);

    let grid_size = antenna_map.data[0].len() as i32;

    let locations = get_antenna_locations(antenna_map);

    let antenna_pairs = get_antenna_pairs(locations);

    let mut antinodes: HashMap<grid::Point, bool> = HashMap::new();

    for (_a, pairs) in antenna_pairs {
        for pair in pairs {
            let next_antinodes = get_antinodes(pair.0, pair.1, grid_size, false);
            for node in next_antinodes {
                antinodes.insert(node, true);
            }
        }
    }

    antinodes.len() as i32
}

pub fn part_2(input: &str) -> i32 {
    let antenna_map = parse_string_grid(input);

    let grid_size = antenna_map.data[0].len() as i32;

    let locations = get_antenna_locations(antenna_map);

    let antenna_pairs = get_antenna_pairs(locations);

    let mut antinodes: HashMap<grid::Point, bool> = HashMap::new();

    for (_a, pairs) in antenna_pairs {
        for pair in pairs {
            let next_antinodes = get_antinodes(pair.0, pair.1, grid_size, true);
            for node in next_antinodes {
                antinodes.insert(node, true);
            }
        }
    }

    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 14);
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
                assert_eq!(result, 34);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
