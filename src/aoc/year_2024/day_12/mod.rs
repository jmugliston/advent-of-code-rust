use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;

use crate::utils::grid::{self, Point};
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_12/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_12/input/input.txt";

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

#[derive(Debug)]
struct Region {
    name: char,
    points: Vec<Point>,
    edges: usize,
    corners: usize,
}

fn get_tile_edges(p: Point, farm_map: &grid::Grid<char>) -> usize {
    let region_name = *farm_map.get(&p).unwrap_or(&'_');

    let neighbours: Vec<_> = p.neighbours(false);

    let mut edges = 0;
    for neighbour in &neighbours {
        if farm_map.get(&neighbour) == None {
            // Not in the grid - must be an edge
            edges += 1;
        } else if region_name != *farm_map.get(&neighbour).unwrap_or(&'_') {
            edges += 1
        }
    }

    return edges;
}

fn get_tile_corners(p: Point, farm_map: &grid::Grid<char>) -> usize {
    let region_name: char = *farm_map.get(&p).unwrap_or(&'_');

    let neighbours: Vec<_> = p.neighbours(false);

    // 4 types of corner cases (can be inside and outside corners)

    // Top left
    // xx.
    // x..

    // Top right
    // .xx
    // ..x

    // Bottom left
    // x..
    // xx.

    // Bottom right
    // ..x
    // .xx

    let north = p.next_points_in_direction(grid::Direction::N, 1)[0];
    let north_east = p.next_points_in_direction(grid::Direction::NE, 1)[0];
    let east = p.next_points_in_direction(grid::Direction::E, 1)[0];
    let south_east = p.next_points_in_direction(grid::Direction::SE, 1)[0];
    let sout = p.next_points_in_direction(grid::Direction::S, 1)[0];
    let south_west = p.next_points_in_direction(grid::Direction::SW, 1)[0];
    let west = p.next_points_in_direction(grid::Direction::W, 1)[0];
    let north_west = p.next_points_in_direction(grid::Direction::NW, 1)[0];

    let n: char = *farm_map.get(&north).unwrap_or(&'_');
    let ne: char = *farm_map.get(&north_east).unwrap_or(&'_');
    let e: char = *farm_map.get(&east).unwrap_or(&'_');
    let se: char = *farm_map.get(&south_east).unwrap_or(&'_');
    let s: char = *farm_map.get(&sout).unwrap_or(&'_');
    let sw: char = *farm_map.get(&south_west).unwrap_or(&'_');
    let w: char = *farm_map.get(&west).unwrap_or(&'_');
    let nw: char = *farm_map.get(&north_west).unwrap_or(&'_');

    let mut matches = 0;
    for neighbour in neighbours {
        if region_name == *farm_map.get(&neighbour).unwrap_or(&'_') {
            matches += 1;
        }
    }

    if matches == 1 {
        // Special case for end points (only 1 adjacent tile)
        return 2;
    }

    let mut corners = 0;

    // Top left
    if e == region_name && s == region_name {
        if w != region_name && n != region_name {
            // Outside
            corners += 1;
        }
        if se != region_name {
            corners += 1;
        }
    }

    // Top right
    if w == region_name && s == region_name {
        if e != region_name && n != region_name {
            corners += 1;
        }
        if sw != region_name {
            corners += 1;
        }
    }

    // Bottom right
    if w == region_name && n == region_name {
        if s != region_name && e != region_name {
            corners += 1;
        }
        if nw != region_name {
            corners += 1;
        }
    }

    // Bottom left
    if e == region_name && n == region_name {
        if s != region_name && w != region_name {
            corners += 1;
        }
        if ne != region_name {
            corners += 1;
        }
    }

    return corners;
}

fn get_region(p: Point, farm_map: &grid::Grid<char>) -> Region {
    let region_name = *farm_map.get(&p).unwrap_or(&'_');

    let mut region = Region {
        name: region_name,
        points: Vec::new(),
        edges: 0,
        corners: 0,
    };

    let mut visited: HashMap<Point, bool> = HashMap::new();

    let mut queue: VecDeque<Point> = VecDeque::new();

    queue.push_front(p);

    while let Some(p) = queue.pop_front() {
        if visited.contains_key(&p) {
            continue;
        }

        visited.insert(p, true);

        region.points.push(p);

        let edges = get_tile_edges(p, farm_map);
        let corners = get_tile_corners(p, farm_map);

        region.edges += edges;
        region.corners += corners;

        let neighbours = p.neighbours(false);

        for neighbour in neighbours {
            if *farm_map.get(&neighbour).unwrap_or(&'_') == region.name
                && !visited.contains_key(&neighbour)
                && !queue.contains(&neighbour)
            {
                queue.push_front(neighbour);
            }
        }
    }

    if region.points.len() == 1 {
        // Special case for single tile regions
        region.corners = 4
    }

    return region;
}

fn get_all_regions(farm_map: &grid::Grid<char>) -> Vec<Region> {
    let mut checked_points: HashMap<grid::Point, bool> = HashMap::new();

    let grid_size: usize = farm_map.data.len();

    let mut regions: Vec<Region> = Vec::new();

    for row in 0..grid_size {
        for col in 0..grid_size {
            let p = Point::new(col as i32, row as i32);
            if checked_points.get_key_value(&p).is_some() {
                continue;
            }
            let region = get_region(p, &farm_map);
            for point in &region.points {
                checked_points.insert(*point, true);
            }
            regions.push(region);
        }
    }

    return regions;
}

pub fn part_1(input: &str) -> i32 {
    let farm_map = grid::parse_string_grid(input);

    let regions = get_all_regions(&farm_map);

    // The price of fence = area * perimeter
    return regions
        .iter()
        .map(|r| r.points.len() * r.edges)
        .sum::<usize>() as i32;
}

pub fn part_2(input: &str) -> i32 {
    let farm_map = grid::parse_string_grid(input);

    let regions = get_all_regions(&farm_map);

    // The price of fence = area * sides
    return regions
        .iter()
        .map(|r| r.points.len() * r.corners)
        .sum::<usize>() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 1930);
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
                assert_eq!(result, 1206);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
