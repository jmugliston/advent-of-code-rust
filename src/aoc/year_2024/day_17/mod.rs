use std::error::Error;
use std::fs;

use crate::utils::parsing::read_lines;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_17/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_17/input/input.txt";

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

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let raw_lines = read_lines(input);
    let lines: Vec<&String> = raw_lines.iter().filter(|x| !x.is_empty()).collect();

    let registers = (0..3)
        .map(|i| {
            lines
                .get(i)
                .and_then(|line| line.split(": ").nth(1))
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();

    let program: Vec<i64> = lines[3]
        .split(": ")
        .nth(1)
        .unwrap_or("")
        .split(',')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    return (registers, program);
}

fn get_combo_operand(registers: &Vec<i64>, operand: i64) -> i64 {
    if operand < 4 {
        return operand;
    }

    if operand == 4 {
        return registers[0];
    }

    if operand == 5 {
        return registers[1];
    }

    if operand == 6 {
        return registers[2];
    }

    return -1;
}

fn adv(registers: &mut Vec<i64>, operand: i64) -> i64 {
    let val = registers[0] / (1 << get_combo_operand(registers, operand));
    registers[0] = val;
    return val;
}

fn bxl(registers: &mut Vec<i64>, operand: i64) -> i64 {
    let val = registers[1] ^ operand;
    registers[1] = val;
    return val;
}

fn bst(registers: &mut Vec<i64>, operand: i64) -> i64 {
    let val = get_combo_operand(registers, operand) % 8;
    registers[1] = val;
    return val;
}

fn bxc(registers: &mut Vec<i64>) -> i64 {
    let val = registers[1] ^ registers[2];
    registers[1] = val;
    return val;
}

fn jnz(instruction_pointer: &mut i64, registers: &mut Vec<i64>, operand: i64) -> bool {
    if registers[0] == 0 {
        return false;
    }
    *instruction_pointer = operand;
    return true;
}

fn out(registers: &mut Vec<i64>, operand: i64) -> i64 {
    return get_combo_operand(registers, operand) % 8;
}

fn bdv(registers: &mut Vec<i64>, operand: i64) -> i64 {
    let val = registers[0] / (1 << get_combo_operand(registers, operand));
    registers[1] = val;
    return val;
}

fn cdv(registers: &mut Vec<i64>, operand: i64) -> i64 {
    let val = registers[0] / (1 << get_combo_operand(registers, operand));
    registers[2] = val;
    return val;
}

fn run_program(registers: &mut Vec<i64>, program: Vec<i64>) -> Vec<i64> {
    let mut outputs: Vec<i64> = Vec::new();
    let mut instruction_pointer: i64 = 0;

    while instruction_pointer < (program.len() as i64 - 1) {
        let op_code = program[instruction_pointer as usize];
        let operand = program[(instruction_pointer + 1) as usize];

        let mut jumped = false;

        match op_code {
            0 => {
                adv(registers, operand);
            }
            1 => {
                bxl(registers, operand);
            }
            2 => {
                bst(registers, operand);
            }
            3 => {
                jumped = jnz(&mut instruction_pointer, registers, operand);
            }
            4 => {
                bxc(registers);
            }
            5 => {
                outputs.push(out(registers, operand));
            }
            6 => {
                bdv(registers, operand);
            }
            7 => {
                cdv(registers, operand);
            }
            _ => {}
        }

        if !jumped {
            instruction_pointer += 2;
        }
    }

    outputs
}

fn solve(n: i64, d: i64, program: &Vec<i64>) -> i64 {
    let mut res: Vec<i64> = vec![i64::MAX];

    if d == -1 {
        return n;
    }

    // Work backwards through the digits to solve

    // Check each 3 bit value (numbers 0-7) because the program output uses mod 8
    for i in 0..8 {
        let nn = n + (i * 8_i64.pow(d as u32));

        let mut registers: Vec<i64> = [nn, 0, 0].to_vec();

        let outputs = run_program(&mut registers, program.clone());

        if outputs.len() != program.len() {
            continue;
        }

        if outputs[d as usize] == program[d as usize] {
            // Partial match - continue to next digit
            res.push(solve(nn, d - 1, program));
        }
    }

    return res.iter().min().copied().unwrap_or(0);
}

pub fn part_1(input: &str) -> String {
    let (mut registers, program) = parse_input(input);

    let outputs = run_program(&mut registers, program);

    outputs
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part_2(input: &str) -> String {
    let (_, program) = parse_input(input);

    let ans = solve(0, (program.len() - 1).try_into().unwrap(), &program);

    return ans.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE_2: &str = "./src/aoc/year_2024/day_17/input/example2.txt";

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
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
                assert_eq!(result, "117440");
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
