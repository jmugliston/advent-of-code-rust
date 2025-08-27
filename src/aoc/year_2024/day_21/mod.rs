use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;

use crate::utils::grid::{self, Direction, Grid, Point};
use crate::utils::parsing;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_21/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_21/input/input.txt";

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

// Find all the shortest paths between start/end keys on the pad
fn shortest_paths(pad: &Grid<char>, start: &char, end: &char) -> Vec<Vec<char>> {
    let start_point = pad.find(start).expect("Start key not found");
    let end_point = pad.find(end).expect("End key not found");

    let mut queue = VecDeque::<(Point, Vec<Point>, Vec<char>)>::new();
    let mut visited = HashSet::<Point>::new();

    queue.push_back((start_point, vec![start_point], vec![]));
    visited.insert(start_point);

    let mut paths = Vec::<Vec<char>>::new();

    while let Some(current) = queue.pop_front() {
        let (current_point, current_path, current_keys) = current;

        if current_point == end_point {
            paths.push(current_keys);
            continue;
        }

        visited.insert(current_point);

        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            let next_point = current_point.next_points_in_direction(dir, 1)[0];

            if !pad.in_bounds(&next_point) {
                continue;
            }

            if visited.contains(&next_point) {
                continue;
            }

            let ch = pad.get(&next_point).expect("Point not found");

            if ch == &'.' {
                continue;
            }

            let mut next_path = current_path.clone();
            next_path.push(next_point);

            let mut next_keys = current_keys.clone();
            match dir {
                Direction::N => next_keys.push('^'),
                Direction::E => next_keys.push('>'),
                Direction::S => next_keys.push('v'),
                Direction::W => next_keys.push('<'),
                _ => {}
            }

            queue.push_back((next_point, next_path, next_keys));
        }
    }

    // Only keep the shortest paths
    if !paths.is_empty() {
        let min_len = paths.iter().map(|p| p.len()).min().unwrap();
        paths.retain(|p| p.len() == min_len);
    }

    paths
}

// Pre-compute all the possible paths from each key on the numpad and dirpad
fn build_key_map() -> HashMap<String, Vec<Vec<char>>> {
    // 7 8 9
    // 4 5 6
    // 1 2 3
    // . 0 A
    let numpad = grid::parse_string_grid("789\n456\n123\n.0A");

    // . ^ A
    // < v >
    let dirpad = grid::parse_string_grid(".^A\n<v>");

    let numbers: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'];
    let directions: Vec<char> = vec!['^', '<', 'v', '>', 'A'];

    let mut key_map = HashMap::<String, Vec<Vec<char>>>::new();

    // Pre-compute all the shortest paths between number keys
    for a in &numbers {
        for b in &numbers {
            if a != b {
                key_map.insert(format!("{a}{b}"), vec![vec!['A']]);
            }
            key_map.insert(format!("{a}{b}"), shortest_paths(&numpad, a, b));
        }
    }

    // Pre-compute all the shortest paths between direction keys
    for a in &directions {
        for b in &directions {
            if a != b {
                key_map.insert(format!("{a}{b}"), vec![vec!['A']]);
            }
            key_map.insert(format!("{a}{b}"), shortest_paths(&dirpad, a, b));
        }
    }

    key_map
}

// Build all possible sequences of keys
fn build_sequences(
    keys: &Vec<char>,
    index: usize,
    prev_key: &char,
    curr_path: Vec<char>,
    key_map: &HashMap<String, Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    if index == keys.len() {
        return vec![curr_path];
    }

    let mut result = Vec::<Vec<char>>::new();

    if let Some(&next_key) = keys.get(index) {
        for path in key_map
            .get(&format!("{}{}", prev_key, next_key))
            .into_iter()
            .flatten()
        {
            let mut new_path = curr_path.clone();

            new_path.extend(path.clone());
            new_path.push('A');

            let sub_result = build_sequences(
                keys,
                index + 1,
                keys.get(index).expect("Could not find key"),
                new_path,
                key_map,
            );

            let res = sub_result.into_iter();

            result.extend(res);
        }
    }

    result
}

// Find the shortest sequence of keys
fn shortest_sequence(
    keys: Vec<char>,
    depth: i32,
    cache: &mut HashMap<String, i64>,
    key_map: &HashMap<String, Vec<Vec<char>>>,
) -> usize {
    if depth == 0 {
        return keys.len();
    }

    let cache_key = format!("{}:{}", keys.iter().collect::<String>(), depth);

    if let Some(cached) = cache.get(&cache_key) {
        return *cached as usize;
    }

    let mut total = 0;

    let joined: String = keys.iter().collect();
    let mut sections: Vec<Vec<char>> = Vec::new();
    let mut current = Vec::new();

    for ch in joined.chars() {
        current.push(ch);
        if ch == 'A' {
            sections.push(current.clone());
            current.clear();
        }
    }
    if !current.is_empty() {
        sections.push(current);
    }

    for sub_key in sections {
        let mut min: usize = usize::MAX;

        let sequences = build_sequences(&sub_key, 0, &'A', vec![], key_map);

        for sequence in sequences {
            let next = shortest_sequence(sequence, depth - 1, cache, key_map);
            if next < min {
                min = next
            }
        }

        total = total + min;
    }

    cache.insert(cache_key, total as i64);

    total
}

pub fn part_1(input: &str) -> i64 {
    let sequences = parsing::read_lines(input);

    let key_map = build_key_map();

    let levels = 2;

    let mut ans = 0;

    for sequence in sequences {
        let keys: Vec<char> = sequence.chars().collect();

        let all_sequences = build_sequences(&keys, 0, &'A', vec![], &key_map);

        let mut shortest_sequences = Vec::<usize>::new();

        for seq in all_sequences {
            let shortest =
                shortest_sequence(seq, levels, &mut HashMap::<String, i64>::new(), &key_map);
            shortest_sequences.push(shortest);
        }

        let num: usize = keys.iter().take(3).collect::<String>().parse().unwrap_or(1);

        if let Some(&min_val) = shortest_sequences.iter().min() {
            ans = ans + (num * min_val);
        }
    }

    return ans as i64;
}

pub fn part_2(input: &str) -> i64 {
    let sequences = parsing::read_lines(input);

    let key_map = build_key_map();

    let levels = 25;

    let mut ans = 0;

    for sequence in sequences {
        let keys: Vec<char> = sequence.chars().collect();

        let all_sequences = build_sequences(&keys, 0, &'A', vec![], &key_map);

        let mut shortest_sequences = Vec::<usize>::new();

        for seq in all_sequences {
            let shortest =
                shortest_sequence(seq, levels, &mut HashMap::<String, i64>::new(), &key_map);
            shortest_sequences.push(shortest);
        }

        let num: usize = keys.iter().take(3).collect::<String>().parse().unwrap_or(1);

        if let Some(&min_val) = shortest_sequences.iter().min() {
            ans = ans + (num * min_val);
        }
    }

    return ans as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 126384);
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
                assert_eq!(result, 154115708116294);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
