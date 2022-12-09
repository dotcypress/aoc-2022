use std::ops::Sub;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const PUZZLES: [Puzzle; 9] = [
    day1::PUZZLE,
    day2::PUZZLE,
    day3::PUZZLE,
    day4::PUZZLE,
    day5::PUZZLE,
    day6::PUZZLE,
    day7::PUZZLE,
    day8::PUZZLE,
    day9::PUZZLE,
];

pub struct Puzzle {
    pub name: &'static str,
    pub solve: fn() -> (String, String),
}

#[macro_export]
macro_rules! puzzle {
    ($name:expr, $solver:ty, $part_one_test:expr, $part_two_test:expr) => {
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
            assert_eq!(<$solver>::ingest(test_puzzle).part_one(), $part_one_test);
        }

        #[cfg(test)]
        #[test]
        fn part_two() {
            let test_puzzle = include_str!("test.txt");
            assert_eq!(<$solver>::ingest(test_puzzle).part_two(), $part_two_test);
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn zero() -> Self {
        Point::default()
    }

    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn parse(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        Self::new(x.parse().unwrap_or_default(), y.parse().unwrap_or_default())
    }

    pub fn offset(&self, dx: isize, dy: isize) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    pub fn neighbor_at(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Self::new(self.x, self.y + 1),
            Dir::Right => Self::new(self.x + 1, self.y),
            Dir::Down => Self::new(self.x, self.y - 1),
            Dir::Left => Self::new(self.x - 1, self.y),
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
