use regex::Regex;
use std::error::Error;
use std::fs;

const EXAMPLE_FILE: &str = "./src/year_2024/day_03/input/example.txt";
const INPUT_FILE: &str = "./src/year_2024/day_03/input/input.txt";

pub fn main(part: Option<i32>, example: Option<bool>) -> Result<(), Box<dyn Error>> {
    let input_file = if example.unwrap_or(true) {
        EXAMPLE_FILE
    } else {
        INPUT_FILE
    };

    let contents = fs::read_to_string(input_file)?;

    let res = match part.unwrap_or(1) {
        1 => part_1(&contents),
        _ => part_2(&contents),
    };

    println!("{}", res);
    Ok(())
}

pub fn part_1(input: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res: Vec<i32> = re
        .captures_iter(input)
        .map(|cap| {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            a * b
        })
        .collect();

    return res.iter().sum();
}

pub fn part_2(input: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut enable = true;

    let res: Vec<i32> = re
        .captures_iter(input)
        .map(|cap| {
            if let Some(m) = cap.get(0) {
                match m.as_str() {
                    "do()" => {
                        enable = true;
                        0
                    }
                    "don't()" => {
                        enable = false;
                        0
                    }
                    _ => {
                        println!("enabled: {:?}", enable);
                        let a: i32 = cap[1].parse().unwrap();
                        let b: i32 = cap[2].parse().unwrap();
                        if enable {
                            a * b
                        } else {
                            0
                        }
                    }
                }
            } else {
                0
            }
        })
        .collect();

    println!("{:?}", res);

    return res.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE_2: &str = "./src/year_2024/day_03/input/example2.txt";

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 161);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }

    #[test]
    fn test_part_2() {
        match fs::read_to_string(EXAMPLE_FILE_2) {
            Ok(input) => {
                let result = part_2(&input);
                assert_eq!(result, 48);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
