use std::error::Error;
use std::fs;

use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_06/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_06/input/input.txt";

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

pub fn part_1(input: &str) -> i64 {
    let operators = input
        .lines()
        .last()
        .unwrap_or("")
        .split_whitespace()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let numbers = input
        .lines()
        .take(input.lines().count().saturating_sub(1))
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i64>().expect("Invalid number"))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut columns: Vec<Vec<i64>> = Vec::new();
    if let Some(first_row) = numbers.first() {
        for col in 0..first_row.len() {
            let column = numbers.iter().map(|row| row[col]).collect::<Vec<i64>>();
            columns.push(column);
        }
    }

    let mut result = 0;
    for (col, op) in columns.iter().zip(operators.iter()) {
        let col_sum = match op.as_str() {
            "+" => col.iter().sum::<i64>(),
            "-" => col.iter().fold(0, |acc, &x| acc - x),
            "*" => col.iter().product::<i64>(),
            "/" => col
                .iter()
                .fold(col[0], |acc, &x| if acc == col[0] { x } else { acc / x }),
            _ => 0,
        };
        result += col_sum;
    }

    return result;
}

pub fn part_2(input: &str) -> i64 {
    let operators = input
        .lines()
        .last()
        .unwrap_or("")
        .split_whitespace()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    // Remove the last line from the input
    let trimmed_input = input
        .lines()
        .take(input.lines().count().saturating_sub(1))
        .collect::<Vec<_>>()
        .join("\n");

    // Split input into rows of characters
    let rows: Vec<Vec<char>> = trimmed_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let n_cols = rows[0].len();

    // Find columns that are exactly one whitespace wide
    let split_indices: Vec<usize> = (0..n_cols)
        .filter(|&col| {
            rows.iter().all(|row| row[col] == ' ')
                && (col == 0 || rows.iter().any(|row| row[col - 1] != ' '))
                && (col + 1 == n_cols || rows.iter().any(|row| row[col + 1] != ' '))
        })
        .collect();

    // Now, split the grid into subgrids using the split_indices
    let mut subgrids = Vec::new();
    let mut start = 0;
    for &split_col in &split_indices {
        let end = split_col;
        let subgrid: Vec<Vec<char>> = rows.iter().map(|row| row[start..end].to_vec()).collect();
        subgrids.push(subgrid);
        start = split_col + 1;
    }
    // Add the last subgrid
    if start < n_cols {
        let subgrid: Vec<Vec<char>> = rows.iter().map(|row| row[start..].to_vec()).collect();
        subgrids.push(subgrid);
    }

    let mut result = 0;

    for (i, grid) in subgrids.iter().enumerate() {
        // Collect numbers from columns, right to left
        let numbers: Vec<i64> = (0..grid[0].len())
            .rev()
            .map(|col| {
                grid.iter()
                    .filter_map(|row| row[col].to_digit(10))
                    .fold(0, |acc, d| acc * 10 + d as i64)
            })
            .filter(|&num| num != 0)
            .collect();

        let op = &operators[i];

        let col_sum = match op.as_str() {
            "+" => numbers.iter().sum::<i64>(),
            "-" => numbers.iter().fold(0, |acc, &x| acc - x),
            "*" => numbers.iter().product::<i64>(),
            "/" => numbers.iter().fold(
                numbers[0],
                |acc, &x| if acc == numbers[0] { x } else { acc / x },
            ),
            _ => 0,
        };

        result += col_sum;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 4277556);
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
                assert_eq!(result, 3263827);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
