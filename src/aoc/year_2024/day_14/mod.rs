use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils::grid::Point;
use crate::utils::parsing;
use crate::Part;
use regex::Regex;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_14/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_14/input/input.txt";

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

#[derive(Debug)]
struct Robot {
    point: Point,
    vx: i32,
    vy: i32,
}

impl Robot {
    pub fn move_robot(&mut self, height: i32, width: i32) {
        self.point.x = (self.point.x + self.vx).rem_euclid(width);
        self.point.y = (self.point.y + self.vy).rem_euclid(height);
    }

    pub fn get_quadrant(&mut self, height: i32, width: i32) -> Option<&'static str> {
        let mid_x = width / 2;
        let mid_y = height / 2;

        if self.point.x == mid_x || self.point.y == mid_y {
            return None;
        }

        let quadrant = match (self.point.x < mid_x, self.point.y < mid_y) {
            (true, true) => "NW",
            (false, true) => "NE",
            (true, false) => "SW",
            (false, false) => "SE",
        };

        return Some(quadrant);
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::<Robot>::new();

    let lines = parsing::read_lines(input);

    let re = Regex::new(r"p=\s*(-?\d+),\s*(-?\d+)\s+v=\s*(-?\d+),\s*(-?\d+)").unwrap();
    for line in lines {
        if let Some(caps) = re.captures(&line) {
            let p_x = caps[1].parse::<i32>().unwrap();
            let p_y = caps[2].parse::<i32>().unwrap();
            let v_x = caps[3].parse::<i32>().unwrap();
            let v_y = caps[4].parse::<i32>().unwrap();
            robots.push(Robot {
                point: Point { x: p_x, y: p_y },
                vx: v_x,
                vy: v_y,
            });
        }
    }

    return robots;
}

fn get_map(_robots: &Vec<Robot>, height: i32, width: i32) -> Vec<Vec<char>> {
    let mut robot_map = vec![vec!['.'; width as usize]; height as usize];

    for robot in _robots.iter() {
        robot_map[robot.point.y as usize][robot.point.x as usize] = '#';
    }

    return robot_map;
}

// fn print_map(map: Vec<Vec<char>>) {
//     println!();
//     for row in map.iter() {
//         println!("{}", row.iter().collect::<String>());
//     }
//     println!();
// }

pub fn part_1(input: &str, example: bool) -> i32 {
    let mut robots = parse_robots(input);

    let (height, width) = if example { (7, 11) } else { (103, 101) };

    for _ in 0..=99 {
        for robot in robots.iter_mut() {
            robot.move_robot(height, width);
        }
    }

    let mut quads: HashMap<&str, i32> = HashMap::new();

    for robot in robots.iter_mut() {
        let quad = robot.get_quadrant(height, width);
        if let Some(q) = quad {
            *quads.entry(q).or_insert(0) += 1;
        }
    }

    return quads.get("NW").unwrap_or(&0)
        * quads.get("NE").unwrap_or(&0)
        * quads.get("SE").unwrap_or(&0)
        * quads.get("SW").unwrap_or(&0);
}

pub fn part_2(input: &str, example: bool) -> i32 {
    let mut robots = parse_robots(input);

    let (height, width) = if example { (7, 11) } else { (103, 101) };

    for step in 0..=10000 {
        for robot in robots.iter_mut() {
            robot.move_robot(height, width);
        }

        let mut x_counts = HashMap::new();
        for robot in robots.iter() {
            *x_counts.entry(robot.point.x).or_insert(0) += 1;
        }

        if x_counts.values().any(|&count| count > 20) {
            let robot_map = get_map(&robots, height, width);

            let mut special = false;
            for (_, row) in robot_map.iter().enumerate() {
                let count = row.iter().filter(|&&c| c == '#').count();
                if count > 20 {
                    special = true
                }
            }

            if special {
                // Uncomment to see the image
                // print_map(robot_map);
                return step;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input, true);
                assert_eq!(result, 12);
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
                assert_eq!(result, 0);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
