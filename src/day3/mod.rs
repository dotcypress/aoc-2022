use std::collections::HashSet;

use crate::*;

puzzle!("Day 3: Rucksack Reorganization", Solver, 157, 70);

struct Solver {
    rucksacks: Vec<Rucksac>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let rucksacks = input.lines().map(Rucksac::parse).collect();
        Self { rucksacks }
    }

    fn part_one(self) -> usize {
        self.rucksacks
            .iter()
            .map(|r| r.left.intersection(&r.right).copied().sum::<usize>())
            .sum()
    }

    fn part_two(self) -> usize {
        self.rucksacks
            .chunks_exact(3)
            .map(|group| {
                group
                    .iter()
                    .map(|r| r.left.union(&r.right).copied().collect::<HashSet<_>>())
                    .reduce(|acc, next| acc.intersection(&next).copied().collect::<HashSet<_>>())
                    .unwrap()
                    .iter()
                    .sum::<usize>()
            })
            .sum()
    }
}

pub struct Rucksac {
    left: HashSet<usize>,
    right: HashSet<usize>,
}

impl Rucksac {
    pub fn parse(line: &str) -> Self {
        let snacks = line
            .chars()
            .map(|snack| match snack {
                l @ 'a'..='z' => (l as u8 - b'a' + 1) as _,
                u @ 'A'..='Z' => (u as u8 - b'A' + 27) as _,
                _ => unreachable!(),
            })
            .collect::<Vec<usize>>();
        let mid = snacks.len() / 2;
        Self {
            left: HashSet::from_iter(snacks.iter().copied().take(mid)),
            right: HashSet::from_iter(snacks.iter().copied().skip(mid).take(mid)),
        }
    }
}
