use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_09/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_09/input/input.txt";

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

// Decode the digits into hard drive format
fn decode(digits: Vec<i32>) -> Vec<Option<i32>> {
    let mut decoded: Vec<Option<i32>> = Vec::new();

    for (idx, digit) in digits.iter().enumerate() {
        if idx % 2 == 0 {
            // Add files
            for _ in 0..*digit {
                decoded.push(Some((idx / 2) as i32));
            }
        } else {
            // Add free space
            for _ in 0..*digit {
                decoded.push(None);
            }
        }
    }

    return decoded;
}

// Defrag the hard drive according to the rules
fn defrag(decoded: Vec<Option<i32>>) -> Vec<Option<i32>> {
    let mut defragged = decoded.clone();

    let mut left = 0;
    let mut right = defragged.len() - 1;

    while left < right {
        if defragged[left].is_some() {
            left += 1;
            continue;
        }
        if defragged[right].is_none() {
            right -= 1;
        }
        defragged[left] = defragged[right];
        defragged[right] = None;
    }

    return defragged;
}

// Defrag the hard drive with the updated algorithm
fn defrag_updated(decoded: Vec<Option<i32>>) -> Vec<Option<i32>> {
    let mut defragged = decoded.clone();

    let mut checked_files: HashMap<Option<i32>, bool> = HashMap::new();

    loop {
        let mut file_id: Option<i32> = Some(0);
        let mut file_idx: usize = 0;
        let mut file_size = 0;

        // Scan from the right and get the next file and size
        for i in (0..defragged.len()).rev() {
            if defragged[i].is_none() {
                continue;
            }

            if checked_files.contains_key(&defragged[i]) {
                continue;
            }

            file_id = defragged[i];

            for j in (0..=i).rev() {
                if defragged[j] != file_id {
                    break;
                }
                file_idx = j;
                file_size += 1;
            }
            break;
        }

        // Scan from the left and see if there is a gap big enough
        for i in 0..defragged.len() {
            if defragged[i].is_some() {
                continue;
            }
            let mut gap_size = 0;
            for j in i..file_idx {
                if defragged[j].is_some() {
                    break;
                }
                gap_size += 1;
            }
            if gap_size >= file_size {
                for j in i..(i + file_size) {
                    defragged[j] = file_id;
                    defragged[file_idx + (j - i)] = None
                }
                break;
            }
        }

        checked_files.insert(file_id, true);

        if file_id == Some(0) {
            break;
        }
    }

    return defragged;
}

// Calculate the final checksum of the hard drive
fn calculate_checksum(defragged: Vec<Option<i32>>) -> i64 {
    let mut checksum: i64 = 0;

    for (x, val) in defragged.iter().enumerate() {
        checksum += (x as i64) * (val.unwrap_or(0) as i64);
    }

    return checksum;
}

pub fn part_1(input: &str) -> i64 {
    let digits = utils::parsing::read_lines_of_digits(input)[0].clone();

    let decoded = decode(digits);

    let defragged = defrag(decoded);

    let checksum = calculate_checksum(defragged);

    return checksum;
}

pub fn part_2(input: &str) -> i64 {
    let digits = utils::parsing::read_lines_of_digits(input)[0].clone();

    let decoded = decode(digits);

    // Note: badly optimised algorithm - should be improved!
    let defragged = defrag_updated(decoded);

    let checksum = calculate_checksum(defragged);

    return checksum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 1928);
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
                assert_eq!(result, 2858);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
