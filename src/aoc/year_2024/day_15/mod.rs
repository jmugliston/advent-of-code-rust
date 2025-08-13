use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils::grid::{self, Direction, Grid, Point};
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_15/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_15/input/input.txt";

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

struct Simulation {
    warehouse_map: Grid<char>,
    obstacles: HashMap<Point, bool>,
    boxes: Vec<WarehouseBox>,
    robot_position: Point,
}

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    One,
    Two,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct WarehouseBox {
    id: u32,
    points: Vec<Point>,
}

impl Simulation {
    fn new(
        warehouse_map: Grid<char>,
        obstacles: HashMap<Point, bool>,
        boxes: Vec<WarehouseBox>,
        robot_position: Point,
    ) -> Self {
        Simulation {
            warehouse_map,
            boxes,
            obstacles,
            robot_position,
        }
    }

    fn get_box_chain(&mut self, start: WarehouseBox, direction: Direction) -> Vec<u32> {
        let mut position_to_box: HashMap<Point, &WarehouseBox> = HashMap::new();
        for b in &self.boxes {
            for p in &b.points {
                position_to_box.insert(*p, b);
            }
        }

        let mut chain = Vec::new();
        let mut visited = HashMap::new();

        let mut current_boxes = vec![start.clone()];
        for b in &current_boxes {
            chain.push(b.id);
            visited.insert(b.id, true);
        }

        loop {
            let mut found_next = false;
            let mut next_boxes = Vec::new();

            for current_box in &current_boxes {
                for p in &current_box.points {
                    let next_point = p.next_points_in_direction(direction, 1)[0];
                    if let Some(next_box) = position_to_box.get(&next_point) {
                        if !visited.contains_key(&next_box.id) {
                            chain.push(next_box.id);
                            visited.insert(next_box.id, true);
                            next_boxes.push((*next_box).clone());
                            found_next = true;
                        }
                    }
                }
            }

            if !found_next {
                break;
            }
            current_boxes = next_boxes;
        }

        chain
    }

    fn is_chain_blocked(&mut self, box_chain: Vec<u32>, direction: Direction) -> bool {
        for bx in self.boxes.iter_mut().filter(|b| box_chain.contains(&b.id)) {
            for point in &mut bx.points {
                // is there an obstacle in that direction?
                let point_to_check = point.next_points_in_direction(direction, 1)[0];
                if self.obstacles.contains_key(&point_to_check) {
                    return true;
                }
            }
        }
        return false;
    }

    fn move_robot(&mut self, direction: Direction) {
        let next_pos = self.robot_position.next_points_in_direction(direction, 1)[0];

        // If obstacle, can't move
        if self.obstacles.contains_key(&next_pos) {
            return;
        }

        // If next position is a box, get the box chain and move all boxes in the chain
        if let Some(idx) = self.boxes.iter().position(|b| b.points.contains(&next_pos)) {
            let start_box = self.boxes[idx].clone();
            let chain_ids = self.get_box_chain(start_box, direction);

            if self.is_chain_blocked(chain_ids.clone(), direction) {
                return;
            }

            // Move each box in the chain by one step in the given direction
            for bx in self.boxes.iter_mut().filter(|b| chain_ids.contains(&b.id)) {
                for point in &mut bx.points {
                    *point = point.next_points_in_direction(direction, 1)[0];
                }
            }
        }

        // Empty space, move robot
        self.robot_position = next_pos;
    }

    fn sum_gps_coords(&mut self) -> i32 {
        return self
            .boxes
            .iter()
            .map(|b| 100 * b.points[0].y + b.points[0].x)
            .sum();
    }
}

impl std::fmt::Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_grid = self.warehouse_map.clone();

        print_grid.data[self.robot_position.y as usize][self.robot_position.x as usize] = '@';

        for b in &self.boxes {
            let p = b.points[0];
            if b.points.len() == 1 {
                print_grid.data[p.y as usize][p.x as usize] = 'O';
            } else {
                print_grid.data[p.y as usize][p.x as usize] = '[';
                print_grid.data[p.y as usize][(p.x + 1) as usize] = ']';
            }
        }

        for row in &print_grid.data {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_map(input: &str, scale: Scale) -> Grid<char> {
    match scale {
        Scale::One => grid::parse_string_grid(input),
        Scale::Two => {
            let rows: Vec<Vec<char>> = input
                .lines()
                .map(|line| {
                    line.chars()
                        .flat_map(|ch| match ch {
                            'O' => vec!['[', ']'],
                            '@' => vec!['@', '.'],
                            '#' => vec!['#', '#'],
                            '.' => vec!['.', '.'],
                            _ => vec![],
                        })
                        .collect()
                })
                .filter(|row: &Vec<char>| !row.is_empty())
                .collect();
            Grid::new(rows)
        }
    }
}

fn parse_input(
    input: &str,
    scale: Scale,
) -> (
    Grid<char>,
    HashMap<Point, bool>,
    Vec<WarehouseBox>,
    Point,
    Vec<Direction>,
) {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let mut warehouse_map = parse_map(input, scale);

    let robot_position: Point = warehouse_map.find_all(&'@')[0];

    warehouse_map.data[robot_position.y as usize][robot_position.x as usize] = '.';

    let mut obstacles = HashMap::<Point, bool>::new();
    for obstacle_point in warehouse_map.find_all(&'#') {
        obstacles.insert(obstacle_point, true);
    }

    let mut boxes = Vec::<WarehouseBox>::new();

    let box_positions: Vec<Vec<Point>> = match scale {
        Scale::One => warehouse_map
            .find_all(&'O')
            .into_iter()
            .map(|p| vec![p])
            .collect(),
        Scale::Two => warehouse_map
            .find_all(&'[')
            .into_iter()
            .map(|p| vec![p, Point { x: p.x + 1, y: p.y }])
            .collect(),
    };

    let mut id = 0;
    for points in box_positions {
        for p in &points {
            warehouse_map.data[p.y as usize][p.x as usize] = '.';
        }
        boxes.push(WarehouseBox { id, points });
        id += 1;
    }

    let mut directions = Vec::<Direction>::new();
    let direction_chars: Vec<char> = parts[1].chars().collect();
    for dir in direction_chars {
        match dir {
            '<' => directions.push(Direction::W),
            '^' => directions.push(Direction::N),
            '>' => directions.push(Direction::E),
            'v' => directions.push(Direction::S),
            _ => {}
        }
    }

    return (warehouse_map, obstacles, boxes, robot_position, directions);
}

pub fn part_1(input: &str) -> i32 {
    let (warehouse_map, obstacles, boxes, robot_position, directions) =
        parse_input(input, Scale::One);

    let mut sim = Simulation::new(warehouse_map, obstacles, boxes, robot_position);

    for dir in directions {
        sim.move_robot(dir);
    }

    return sim.sum_gps_coords();
}

pub fn part_2(input: &str) -> i32 {
    let (warehouse_map, obstacles, boxes, robot_position, directions) =
        parse_input(input, Scale::Two);

    let mut sim = Simulation::new(warehouse_map, obstacles, boxes, robot_position);

    for dir in directions {
        sim.move_robot(dir);
    }

    return sim.sum_gps_coords();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, 10092);
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
                assert_eq!(result, 9021);
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
