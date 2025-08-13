use std::error::Error;
use std::fs;

use nalgebra::{DMatrix, DVector};

use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_13/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_13/input/input.txt";

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
struct XYvalues {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Config {
    button_a: XYvalues,
    button_b: XYvalues,
    prize: XYvalues,
}

fn extract_coords(line: &str) -> XYvalues {
    let re = regex::Regex::new(r"X[+=](\d+),\s*Y[+=](\d+)").unwrap();
    re.captures(line)
        .and_then(|caps| {
            let x = caps.get(1)?.as_str().parse().ok()?;
            let y = caps.get(2)?.as_str().parse().ok()?;
            Some(XYvalues { x, y })
        })
        .unwrap_or(XYvalues { x: 0, y: 0 })
}

fn parse_configurations(sections: Vec<&str>) -> Vec<Config> {
    let mut button_configurations: Vec<Config> = Vec::new();

    for section in sections {
        let lines: Vec<&str> = section.split("\n").collect();

        let next_config: Config = Config {
            button_a: extract_coords(lines[0]),
            button_b: extract_coords(lines[1]),
            prize: extract_coords(lines[2]),
        };

        button_configurations.push(next_config);
    }

    return button_configurations;
}

fn solve_linear_equation(x1: f64, x2: f64, y1: f64, y2: f64, res_x: f64, res_y: f64) -> (f64, f64) {
    // Create a matrix A and a vector b
    let a = DMatrix::from_row_slice(2, 2, &[x1, x2, y1, y2]);
    let b = DVector::from_row_slice(&[res_x, res_y]);

    // Solve the linear system
    let x = a.lu().solve(&b).expect("Could not solve the equations");

    // Round to 3 decimal places to avoid precision errors
    let final_x = (x[0] * 1000.0).round() / 1000.0;
    let final_y = (x[1] * 1000.0).round() / 1000.0;

    (final_x, final_y)
}

pub fn part_1(input: &str) -> i64 {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let button_configurations = parse_configurations(sections);

    let mut total = 0;

    for config in button_configurations {
        let (x, y) = solve_linear_equation(
            config.button_a.x as f64,
            config.button_b.x as f64,
            config.button_a.y as f64,
            config.button_b.y as f64,
            config.prize.x as f64,
            config.prize.y as f64,
        );

        if x.fract() == 0.0 && y.fract() == 0.0 {
            total += (x as i64 * 3) + (y as i64);
        }
    }

    return total;
}

pub fn part_2(input: &str) -> i64 {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let button_configurations = parse_configurations(sections);

    let mut total = 0;

    for config in button_configurations {
        let (x, y) = solve_linear_equation(
            config.button_a.x as f64,
            config.button_b.x as f64,
            config.button_a.y as f64,
            config.button_b.y as f64,
            config.prize.x as f64 + 10000000000000.0,
            config.prize.y as f64 + 10000000000000.0,
        );

        if x.fract() == 0.0 && y.fract() == 0.0 {
            total += (x as i64 * 3) + (y as i64);
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 480);
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
                assert_eq!(result, 875318608908);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
