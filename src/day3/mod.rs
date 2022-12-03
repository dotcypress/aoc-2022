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
                let a = group[0].left.union(&group[0].right).collect::<HashSet<_>>();
                let b = group[1].left.union(&group[1].right).collect::<HashSet<_>>();
                let c = group[2].left.union(&group[2].right).collect::<HashSet<_>>();
                a.intersection(&b)
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&c)
                    .copied()
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
        let snacks = line.chars().map(priority).collect::<Vec<usize>>();
        let mid = snacks.len() / 2;
        Self {
            left: HashSet::from_iter(snacks.iter().copied().take(mid)),
            right: HashSet::from_iter(snacks.iter().copied().skip(mid).take(mid)),
        }
    }
}

pub fn priority(shack: char) -> usize {
    match shack {
        l @ 'a'..='z' => (l as u8 - b'a' + 1) as _,
        u @ 'A'..='Z' => (u as u8 - b'A' + 27) as _,
        _ => unreachable!(),
    }
}
