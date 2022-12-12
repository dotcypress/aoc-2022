use std::ops::*;

pub struct Puzzle {
    pub name: &'static str,
    pub solve: fn() -> (String, String),
}

#[macro_export]
macro_rules! puzzle {
    ($name:expr, $solver:ty, $part_one_test_answer:expr, $part_two_test_answer:expr) => {
        pub const PUZZLE: Puzzle = Puzzle {
            name: $name,
            solve: || {
                let puzzle = include_str!("puzzle.txt");
                (
                    <$solver>::ingest(puzzle).part_one().to_string(),
                    <$solver>::ingest(puzzle).part_two().to_string(),
                )
            },
        };

        #[cfg(test)]
        #[test]
        fn part_one() {
            let test_puzzle = include_str!("test.txt");
            assert_eq!(
                <$solver>::ingest(test_puzzle).part_one(),
                $part_one_test_answer
            );
        }

        #[cfg(test)]
        #[test]
        fn part_two() {
            let test_puzzle = include_str!("test.txt");
            assert_eq!(
                <$solver>::ingest(test_puzzle).part_two(),
                $part_two_test_answer
            );
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Dir {
    North,
    NorthEast,
    East,
    SoutEeast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub const fn unit(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    pub const fn direction(&self) -> Option<Dir> {
        match self.unit() {
            Point { x: 0, y: 1 } => Some(Dir::North),
            Point { x: 1, y: 1 } => Some(Dir::NorthEast),
            Point { x: 1, y: 0 } => Some(Dir::East),
            Point { x: 1, y: -1 } => Some(Dir::SoutEeast),
            Point { x: 0, y: -1 } => Some(Dir::South),
            Point { x: -1, y: -1 } => Some(Dir::SouthWest),
            Point { x: -1, y: 0 } => Some(Dir::West),
            Point { x: -1, y: 1 } => Some(Dir::NorthWest),
            _ => None,
        }
    }

    pub const fn neighbor_at(&self, dir: Dir) -> Self {
        match dir {
            Dir::North => Self::new(self.x, self.y + 1),
            Dir::NorthEast => Self::new(self.x + 1, self.y + 1),
            Dir::East => Self::new(self.x + 1, self.y),
            Dir::SoutEeast => Self::new(self.x + 1, self.y - 1),
            Dir::South => Self::new(self.x, self.y - 1),
            Dir::SouthWest => Self::new(self.x - 1, self.y - 1),
            Dir::West => Self::new(self.x - 1, self.y),
            Dir::NorthWest => Self::new(self.x - 1, self.y + 1),
        }
    }

    pub const fn neighbors(&self) -> Neighbors {
        Neighbors::new(*self, false)
    }

    pub const fn close_neighbors(&self) -> Neighbors {
        Neighbors::new(*self, true)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub struct Neighbors {
    origin: Point,
    next: usize,
    close_only: bool,
}

impl Neighbors {
    const NEIGHBORHOOD: [Point; 8] = [
        Point::zero().neighbor_at(Dir::North),
        Point::zero().neighbor_at(Dir::NorthEast),
        Point::zero().neighbor_at(Dir::East),
        Point::zero().neighbor_at(Dir::SoutEeast),
        Point::zero().neighbor_at(Dir::South),
        Point::zero().neighbor_at(Dir::SouthWest),
        Point::zero().neighbor_at(Dir::West),
        Point::zero().neighbor_at(Dir::NorthWest),
    ];

    pub const fn new(origin: Point, close_only: bool) -> Self {
        Self {
            origin,
            next: 0,
            close_only,
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= Self::NEIGHBORHOOD.len() {
            return None;
        }
        let offset = Self::NEIGHBORHOOD[self.next];
        if self.close_only {
            self.next += 2;
        } else {
            self.next += 1;
        }
        Some(self.origin + offset)
    }
}
