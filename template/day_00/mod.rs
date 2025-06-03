use std::error::Error;
use std::fs;

const EXAMPLE_FILE: &str = "./src/year_0000/day_00/input/example.txt";
const INPUT_FILE: &str = "./src/year_0000/day_00/input/input.txt";

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
    return 0;
}

pub fn part_2(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 0);
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
                assert_eq!(result, 0);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
