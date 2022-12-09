use std::collections::HashSet;

use crate::*;

puzzle!("Day 9: Rope Bridge", Solver, 13, 1);

struct Solver {
    inst: Vec<(Dir, usize)>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let inst = input
            .lines()
            .map(|l| match l.split_once(' ') {
                Some(("U", steps)) => (Dir::North, steps.parse().unwrap()),
                Some(("R", steps)) => (Dir::East, steps.parse().unwrap()),
                Some(("D", steps)) => (Dir::South, steps.parse().unwrap()),
                Some(("L", steps)) => (Dir::West, steps.parse().unwrap()),
                _ => unreachable!(),
            })
            .collect();
        Self { inst }
    }

    fn part_one(self) -> usize {
        self.simulate(2)
    }

    fn part_two(self) -> usize {
        self.simulate(10)
    }

    pub fn simulate(&self, knots: usize) -> usize {
        let mut tail_track = HashSet::new();
        tail_track.insert(Point::zero());

        let mut rope = Vec::with_capacity(knots);
        for _ in 0..knots {
            rope.push(Point::zero());
        }

        for (dir, steps) in &self.inst {
            for _ in 0..*steps {
                rope[0] = rope[0].neighbor_at(*dir);
                for idx in 1..rope.len() {
                    let delta = rope[idx - 1] - rope[idx];
                    if delta.x.abs() > 1 || delta.y.abs() > 1 {
                        rope[idx] = rope[idx] + delta.unit();
                    }
                }
                tail_track.insert(*rope.last().unwrap());
            }
        }
        tail_track.len()
    }
}
