use crate::*;
use std::collections::HashSet;

puzzle!("Day 6: Tuning Trouble", Solver, 7, 19);

struct Solver {
    stream: Vec<char>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        Self {
            stream: input.chars().collect(),
        }
    }

    fn part_one(self) -> usize {
        self.packet_start_at(4)
    }

    fn part_two(self) -> usize {
        self.packet_start_at(14)
    }

    pub fn packet_start_at(self, window: usize) -> usize {
        self.stream
            .windows(window)
            .map(|w| HashSet::<char>::from_iter(w.iter().copied()).len())
            .enumerate()
            .find(|(_, l)| *l >= window)
            .map(|(idx, _)| idx + window)
            .unwrap()
    }
}
