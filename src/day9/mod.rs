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
                Some(("U", steps)) => (Dir::Up, steps.parse().unwrap()),
                Some(("R", steps)) => (Dir::Right, steps.parse().unwrap()),
                Some(("D", steps)) => (Dir::Down, steps.parse().unwrap()),
                Some(("L", steps)) => (Dir::Left, steps.parse().unwrap()),
                _ => unreachable!(),
            })
            .collect();
        Self { inst }
    }

    fn part_one(self) -> usize {
        Field::new(2).simulate(&self.inst)
    }

    fn part_two(self) -> usize {
        Field::new(10).simulate(&self.inst)
    }
}

pub struct Field {
    rope: Vec<Point>,
}

impl Field {
    pub fn new(knots: usize) -> Self {
        let mut rope = Vec::with_capacity(knots);
        for _ in 0..knots {
            rope.push(Point::zero());
        }
        Self { rope }
    }

    pub fn simulate(mut self, inst: &[(Dir, usize)]) -> usize {
        let mut tail_track = HashSet::new();
        tail_track.insert(Point::zero());
        for (dir, steps) in inst {
            for _ in 0..*steps {
                self.rope[0] = self.rope[0].neighbor_at(*dir);
                for idx in 1..self.rope.len() {
                    let delta = self.rope[idx - 1] - self.rope[idx];
                    if delta.x.abs() > 1 || delta.y.abs() > 1 {
                        self.rope[idx] = self.rope[idx].offset(delta.x.signum(), delta.y.signum());
                    }
                }
                tail_track.insert(*self.rope.last().unwrap());
            }
        }
        tail_track.len()
    }
}
