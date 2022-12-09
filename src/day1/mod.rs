use crate::*;

puzzle!("Day 1: Calorie Counting", Solver, 24000, 45000);

struct Solver {
    snacks: Vec<usize>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let mut snacks = input.lines().fold(vec![0], |mut acc, line| {
            if line.is_empty() {
                acc.push(0);
            } else if let Some(c) = acc.last_mut() {
                *c += line.parse::<usize>().unwrap();
            }
            acc
        });
        snacks.sort_by(|a, b| b.cmp(a));
        Self { snacks }
    }

    fn part_one(self) -> usize {
        self.snacks[0]
    }

    fn part_two(self) -> usize {
        self.snacks.iter().take(3).sum()
    }
}
