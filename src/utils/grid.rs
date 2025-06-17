use std::fmt;

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

    pub fn neighbours(&self) -> Vec<Point> {
        use Direction::*;
        let directions = [
            (N, (0, -1)),
            (NE, (1, -1)),
            (E, (1, 0)),
            (SE, (1, 1)),
            (S, (0, 1)),
            (SW, (-1, 1)),
            (W, (-1, 0)),
            (NW, (-1, -1)),
        ];
        directions
            .iter()
            .map(|&(_, (dx, dy))| Point {
                x: self.x + dx,
                y: self.y + dy,
            })
            .collect()
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

    pub fn neighbours(&self) -> Vec<PointWithDirection> {
        use Direction::*;
        let directions = [
            (N, (0, -1)),
            (NE, (1, -1)),
            (E, (1, 0)),
            (SE, (1, 1)),
            (S, (0, 1)),
            (SW, (-1, 1)),
            (W, (-1, 0)),
            (NW, (-1, -1)),
        ];
        directions
            .iter()
            .map(|&(dir, (dx, dy))| PointWithDirection {
                x: self.x + dx,
                y: self.y + dy,
                direction: dir,
            })
            .collect()
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
}

impl<T: PartialEq> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    pub fn size(&self) -> (usize, usize) {
        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };
        (rows, cols)
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.data
            .get(p.y as usize)
            .and_then(|row| row.get(p.x as usize))
    }

    pub fn set(&mut self, p: &Point, value: T) {
        if let Some(row) = self.data.get_mut(p.y as usize) {
            if let Some(cell) = row.get_mut(p.x as usize) {
                *cell = value;
            }
        }
    }

    pub fn get_xy(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn compare(&self, other: &Grid<T>) -> bool {
        self.data == other.data
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for item in row {
                write!(f, "{} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().expect("Failed to parse number"))
                    .collect()
            })
            .collect(),
    )
}
