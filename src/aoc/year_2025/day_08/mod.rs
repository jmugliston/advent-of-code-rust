use std::error::Error;
use std::fs;

use crate::utils::parsing;
use crate::Part;
use std::collections::HashMap;

const EXAMPLE_FILE: &str = "./src/aoc/year_2025/day_08/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2025/day_08/input/input.txt";

pub fn main(part: Part, example: bool) -> Result<(), Box<dyn Error>> {
    let input_file = if example { EXAMPLE_FILE } else { INPUT_FILE };

    let contents = fs::read_to_string(input_file)?;

    let res = match part {
        Part::One => part_1(&contents, example),
        Part::Two => part_2(&contents),
    };

    println!("{}", res);
    Ok(())
}
// 3D point representing a junction box
#[derive(Debug, PartialEq, Eq, Hash)]
struct JunctionPoint {
    x: i32,
    y: i32,
    z: i32,
}

impl JunctionPoint {
    pub fn distance(&self, other: &JunctionPoint) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn parse_junction_points(input: &str) -> Vec<JunctionPoint> {
    let mut junction_points: Vec<JunctionPoint> = Vec::new();

    let lines: Vec<String> = parsing::read_lines(input);

    for line in lines {
        let coords: Vec<i32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        if coords.len() == 3 {
            junction_points.push(JunctionPoint {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            });
        }
    }
    junction_points
}

fn compute_sorted_pairs_by_distance(
    junction_points: &Vec<JunctionPoint>,
) -> Vec<(&JunctionPoint, &JunctionPoint, f64)> {
    let mut pairs: Vec<(&JunctionPoint, &JunctionPoint, f64)> = Vec::new();
    for (i, jp) in junction_points.iter().enumerate() {
        for other_jp in junction_points.iter().skip(i + 1) {
            let distance = jp.distance(other_jp);
            pairs.push((jp, other_jp, distance));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    pairs
}

fn get_circuits_sorted_desc(jp_to_circuit: &HashMap<&JunctionPoint, isize>) -> Vec<(isize, usize)> {
    let mut circuits: HashMap<isize, Vec<&JunctionPoint>> = HashMap::new();

    for (jp, circuit_id) in jp_to_circuit {
        circuits.entry(*circuit_id).or_default().push(*jp);
    }

    let mut circuit_sizes: Vec<(isize, usize)> = circuits
        .iter()
        .map(|(circuit_id, points)| (*circuit_id, points.len()))
        .collect();

    circuit_sizes.sort_by_key(|&(_, size)| std::cmp::Reverse(size));

    circuit_sizes
}

fn assign_or_merge_circuits<'a>(
    jp_to_circuit: &mut HashMap<&'a JunctionPoint, isize>,
    jp1: &'a JunctionPoint,
    jp2: &'a JunctionPoint,
    total_circuits: isize,
) -> isize {
    let mut total_circuits = total_circuits;
    let jp1_circuit_val = *jp_to_circuit.get(jp1).unwrap_or(&-1);
    let jp2_circuit_val = *jp_to_circuit.get(jp2).unwrap_or(&-1);

    if (jp1_circuit_val == -1) && (jp2_circuit_val == -1) {
        // Neither JunctionPoint is assigned to a circuit (add a new one)
        total_circuits += 1;
        jp_to_circuit.insert(jp1, total_circuits);
        jp_to_circuit.insert(jp2, total_circuits);
    } else if (jp1_circuit_val != -1) && (jp2_circuit_val == -1) {
        // Only jp1 is assigned to a circuit
        jp_to_circuit.insert(jp2, jp1_circuit_val);
    } else if (jp1_circuit_val == -1) && (jp2_circuit_val != -1) {
        // Only jp2 is assigned to a circuit
        jp_to_circuit.insert(jp1, jp2_circuit_val);
    } else if jp1_circuit_val != jp2_circuit_val {
        // Both are assigned to different circuits; need to merge
        let circuit_to_keep = jp1_circuit_val;
        let circuit_to_merge = jp2_circuit_val;
        for (_, circuit_id) in jp_to_circuit.iter_mut() {
            if *circuit_id == circuit_to_merge {
                *circuit_id = circuit_to_keep;
            }
        }
        total_circuits -= 1;
    }
    total_circuits
}

pub fn part_1(input: &str, example: bool) -> i64 {
    let junction_points = parse_junction_points(input);

    let num_connections = if example { 10 } else { 1000 };

    let sorted_pairs = compute_sorted_pairs_by_distance(&junction_points);

    let mut total_circuits = 0;
    let mut jp_to_circuit: HashMap<&JunctionPoint, isize> = HashMap::new();

    for i in 0..num_connections {
        let (nearest_jp1, nearest_jp2, _) = sorted_pairs[i];

        total_circuits =
            assign_or_merge_circuits(&mut jp_to_circuit, nearest_jp1, nearest_jp2, total_circuits);
    }

    let circuit_sizes = get_circuits_sorted_desc(&jp_to_circuit);

    let total: i64 = circuit_sizes
        .iter()
        .take(3)
        .map(|&(_, size)| size as i64)
        .product();

    return total;
}

pub fn part_2(input: &str) -> i64 {
    let junction_points = parse_junction_points(input);

    let sorted_pairs = compute_sorted_pairs_by_distance(&junction_points);

    let mut total_circuits = 0;
    let mut jp_to_circuit: HashMap<&JunctionPoint, isize> = HashMap::new();

    for i in 0..sorted_pairs.len() {
        let (nearest_jp1, nearest_jp2, _) = sorted_pairs[i];

        total_circuits =
            assign_or_merge_circuits(&mut jp_to_circuit, nearest_jp1, nearest_jp2, total_circuits);

        let circuit_sizes = get_circuits_sorted_desc(&jp_to_circuit);

        let max_circuit_size = circuit_sizes[0].1;

        if max_circuit_size == junction_points.len() {
            return nearest_jp1.x as i64 * nearest_jp2.x as i64;
        }
    }

    panic!("All junction points could not be connected into a single circuit");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input, true);
                assert_eq!(result, 40);
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
                assert_eq!(result, 25272);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
