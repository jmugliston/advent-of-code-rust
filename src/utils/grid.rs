use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointWithDirection {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn neighbours(&self, include_diagonal: bool) -> Vec<Point> {
        let directions: Vec<(i32, i32)> = if include_diagonal {
            vec![
                (0, -1),  // N
                (1, 0),   // E
                (0, 1),   // S
                (-1, 0),  // W
                (1, -1),  // NE
                (1, 1),   // SE
                (-1, 1),  // SW
                (-1, -1), // NW
            ]
        } else {
            vec![
                (0, -1), // N
                (1, 0),  // E
                (0, 1),  // S
                (-1, 0), // W
            ]
        };
        return directions
            .iter()
            .map(|(dx, dy)| Point {
                x: self.x + dx,
                y: self.y + dy,
            })
            .collect();
    }

    pub fn with_direction(&self, direction: Direction) -> PointWithDirection {
        PointWithDirection {
            x: self.x,
            y: self.y,
            direction,
        }
    }

    pub fn next_points_in_direction(&self, direction: Direction, num: i32) -> Vec<Point> {
        use Direction::*;
        let (dx, dy) = match direction {
            N => (0, -1),
            NE => (1, -1),
            E => (1, 0),
            SE => (1, 1),
            S => (0, 1),
            SW => (-1, 1),
            W => (-1, 0),
            NW => (-1, -1),
        };
        (1..=num)
            .map(|i| Point {
                x: self.x + dx * i,
                y: self.y + dy * i,
            })
            .collect()
    }

    /// Returns all points within the given Manhattan distance from this point.
    pub fn in_range(&self, distance: i32) -> Vec<Point> {
        if distance <= 0 {
            return Vec::new();
        }
        let mut points = Vec::new();
        for dx in -distance..=distance {
            let max_dy = distance - dx.abs();
            for dy in -max_dy..=max_dy {
                if dx == 0 && dy == 0 {
                    continue;
                }
                points.push(Point {
                    x: self.x + dx,
                    y: self.y + dy,
                });
            }
        }
        points
    }

    /// Returns the Manhattan distance between two points.
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl PointWithDirection {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        PointWithDirection { x, y, direction }
    }

    pub fn as_point(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y,
        };
    }

    pub fn neighbours(&self, include_diagonal: bool) -> Vec<Point> {
        let directions: Vec<(i32, i32)> = if include_diagonal {
            vec![
                (0, -1),  // N
                (1, 0),   // E
                (0, 1),   // S
                (-1, 0),  // W
                (1, -1),  // NE
                (1, 1),   // SE
                (-1, 1),  // SW
                (-1, -1), // NW
            ]
        } else {
            vec![
                (0, -1), // N
                (1, 0),  // E
                (0, 1),  // S
                (-1, 0), // W
            ]
        };

        return directions
            .iter()
            .map(|(dx, dy)| Point {
                x: self.x + dx,
                y: self.y + dy,
            })
            .collect();
    }

    pub fn turn_clockwise(&self, degrees: i32) -> Self {
        let num_steps = ((degrees / 45) % 8 + 8) % 8; // ensure positive modulo
        let new_dir = ((self.direction as i32 + num_steps) % 8) as u8;
        let new_direction = match new_dir {
            0 => Direction::N,
            1 => Direction::NE,
            2 => Direction::E,
            3 => Direction::SE,
            4 => Direction::S,
            5 => Direction::SW,
            6 => Direction::W,
            7 => Direction::NW,
            _ => panic!("Invalid direction value: {}", new_dir),
        };

        return PointWithDirection {
            x: self.x,
            y: self.y,
            direction: new_direction,
        };
    }

    pub fn next_step(&self) -> PointWithDirection {
        use Direction::*;
        let (dx, dy) = match self.direction {
            N => (0, -1),
            NE => (1, -1),
            E => (1, 0),
            SE => (1, 1),
            S => (0, 1),
            SW => (-1, 1),
            W => (-1, 0),
            NW => (-1, -1),
        };
        return PointWithDirection {
            x: self.x + dx,
            y: self.y + dy,
            direction: self.direction,
        };
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a [T];
    type IntoIter = std::iter::Map<std::slice::Iter<'a, Vec<T>>, fn(&'a Vec<T>) -> &'a [T]>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter().map(|row| row.as_slice())
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = std::iter::Flatten<std::slice::IterMut<'a, Vec<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut().flatten()
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    pub fn init(height: usize, width: usize, value: T) -> Grid<T>
    where
        T: Clone,
    {
        Grid {
            data: vec![vec![value.clone(); width]; height],
        }
    }

    pub fn with_size(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone,
    {
        Grid {
            data: vec![vec![default; cols]; rows],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };
        (rows, cols)
    }

    pub fn find(&self, value: &T) -> Option<Point> {
        for (y, row) in self.data.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if item == value {
                    return Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        None
    }

    pub fn find_all(&self, value: &T) -> Vec<Point> {
        let mut points = Vec::new();
        for (y, row) in self.data.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if item == value {
                    points.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        return points;
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.data
            .get(p.y as usize)
            .and_then(|row| row.get(p.x as usize))
    }

    pub fn neighbours(&self, p: &Point, include_diagonal: bool) -> Vec<Point> {
        p.neighbours(include_diagonal)
            .into_iter()
            .filter(|n| self.in_bounds(n))
            .collect()
    }

    pub fn set(&mut self, p: &Point, value: T) {
        if let Some(row) = self.data.get_mut(p.y as usize) {
            if let Some(cell) = row.get_mut(p.x as usize) {
                *cell = value;
            }
        }
    }

    /// Rotates the grid 90 degrees clockwise `times` times.
    /// `times` can be any integer (positive for clockwise, negative for counterclockwise).
    pub fn rotate(&self, times: i32) -> Grid<T>
    where
        T: Clone,
    {
        let mut result = self.clone();
        let mut t = ((times % 4) + 4) % 4; // normalize to [0,3]
        while t > 0 {
            let (rows, cols) = result.size();
            let mut new_data = vec![vec![result.data[0][0].clone(); rows]; cols];
            for y in 0..rows {
                for x in 0..cols {
                    new_data[x][rows - 1 - y] = result.data[y][x].clone();
                }
            }
            result = Grid { data: new_data };
            t -= 1;
        }
        result
    }

    /// Set the value at multiple points.
    pub fn set_many<I>(&mut self, points: I, value: T)
    where
        I: IntoIterator<Item = Point>,
        T: Clone,
    {
        for p in points {
            self.set(&p, value.clone());
        }
    }

    pub fn get_xy(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn compare(&self, other: &Grid<T>) -> bool {
        self.data == other.data
    }

    pub fn in_bounds(&self, p: &Point) -> bool {
        let (rows, cols) = self.size();
        p.x >= 0 && p.y >= 0 && (p.y as usize) < rows && (p.x as usize) < cols
    }

    /// Finds the shortest path from start to end, treating `walls` as impassable.
    pub fn shortest_path(&self, start: Point, end: Point, walls: &char) -> Vec<Point>
    where
        T: std::cmp::PartialEq<char>,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        let mut came_from = HashMap::new();

        queue.push_back(start);
        visited.insert(start, true);

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct the path
                let mut path = vec![end];
                let mut curr = end;
                while let Some(&prev) = came_from.get(&curr) {
                    path.push(prev);
                    curr = prev;
                }
                path.reverse();
                return path;
            }

            for p in current.neighbours(false) {
                if !self.in_bounds(&p) {
                    continue;
                }

                if visited.contains_key(&p) {
                    continue;
                }

                if let Some(cell) = self.get(&p) {
                    if cell == walls {
                        continue;
                    }
                }

                queue.push_back(p);
                visited.insert(p, true);
                came_from.insert(p, current);
            }
        }
        Vec::new() // No path found
    }

    pub fn print_path(&self, path: &Vec<Point>)
    where
        T: From<char> + Copy + std::fmt::Display,
    {
        let mut grid_copy = self.clone();
        for point in path {
            grid_copy.set(point, T::from('0'));
        }
        println!("{}", grid_copy);
    }

    pub fn print_directed_path(&self, path: &Vec<PointWithDirection>)
    where
        T: From<char> + Copy + std::fmt::Display,
    {
        let mut grid_copy = self.clone();
        for point in path {
            let dir_char = match point.direction {
                Direction::E => '>',
                Direction::W => '<',
                Direction::S => 'v',
                Direction::N => '^',
                Direction::NE => '/',
                Direction::NW => '\\',
                Direction::SE => '\\',
                Direction::SW => '/',
            };
            grid_copy.set(&point.as_point(), T::from(dir_char));
        }
        println!("{}", grid_copy);
    }

    pub fn print_points(&self, path: &Vec<Point>)
    where
        T: From<char> + Copy + std::fmt::Display,
    {
        let mut grid_copy = self.clone();
        for point in path {
            grid_copy.set(&point, T::from('0'));
        }
        println!("{}", grid_copy);
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                write!(f, "{}", item)?;
                if j + 1 < row.len() {
                    write!(f, " ")?;
                }
            }
            if i + 1 < self.data.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Direction::N => "N",
            Direction::NE => "NE",
            Direction::E => "E",
            Direction::SE => "SE",
            Direction::S => "S",
            Direction::SW => "SW",
            Direction::W => "W",
            Direction::NW => "NW",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for PointWithDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) {}", self.x, self.y, self.direction)
    }
}

pub fn parse_string_grid(input: &str) -> Grid<char> {
    Grid::new(input.lines().map(|line| line.chars().collect()).collect())
}

pub fn parse_number_grid(input: &str) -> Grid<u32> {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Failed to parse digit"))
                    .collect()
            })
            .collect(),
    )
}
