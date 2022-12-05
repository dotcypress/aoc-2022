use crate::*;
use std::ops::RangeInclusive;

puzzle!("Day 4: Camp Cleanup", Solver, 2, 4);

struct Solver {
    assigments: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let assigments = input
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(a, b)| {
                let (start, end) = a.split_once('-').unwrap();
                let a = RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap());
                let (start, end) = b.split_once('-').unwrap();
                let b = RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap());
                (a, b)
            })
            .collect();
        Self { assigments }
    }

    fn part_one(self) -> usize {
        self.assigments
            .iter()
            .filter(|(a, b)| {
                (a.contains(b.start()) && a.contains(b.end()))
                    || (b.contains(a.start()) && b.contains(a.end()))
            })
            .count()
    }

    fn part_two(self) -> usize {
        self.assigments
            .iter()
            .filter(|(a, b)| {
                a.contains(b.start())
                    || a.contains(b.end())
                    || b.contains(a.start())
                    || b.contains(a.end())
            })
            .count()
    }
}
