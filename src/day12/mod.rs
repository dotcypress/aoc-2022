use std::collections::HashMap;

use crate::*;

puzzle!("Day 12: Hill Climbing Algorithm", Solver, 31, 29);

struct Solver {
    map: HashMap<Point, u8>,
    steps: HashMap<Point, usize>,
    ground: Vec<Point>,
    start: Point,
    end: Point,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let mut ground = Vec::new();
        let mut map = HashMap::new();
        let mut steps = HashMap::new();
        let mut start = Point::zero();
        let mut end = Point::zero();
        for (y, line) in input.lines().enumerate() {
            for (x, height) in line.chars().enumerate() {
                let point = Point::new(x as _, y as _);
                let mut height = height as u8;
                if height == b'S' {
                    height = b'a';
                    start = point;
                } else if height == b'E' {
                    height = b'z';
                    end = point;
                }
                if height == b'a' {
                    ground.push(point);
                }
                map.insert(point, height);
                steps.insert(point, usize::MAX);
            }
        }

        Self {
            map,
            start,
            end,
            steps,
            ground,
        }
    }

    fn part_one(mut self) -> usize {
        self.visit(self.end, 0);
        *self.steps.get(&self.start).unwrap()
    }

    fn part_two(&mut self) -> usize {
        self.visit(self.end, 0);
        *self
            .ground
            .iter()
            .map(|p| self.steps.get(p).unwrap())
            .min()
            .unwrap()
    }

    fn visit(&mut self, point: Point, steps: usize) {
        if steps >= *self.steps.get(&point).unwrap_or(&usize::MAX) {
            return;
        }
        self.steps.insert(point, steps);
        for neighbor in point.close_neighbors() {
            let height = *self.map.get(&point).unwrap();
            let next_height = *self.map.get(&neighbor).unwrap_or(&0);
            if next_height >= height || next_height == (height - 1) {
                self.visit(neighbor, steps + 1);
            }
        }
    }
}
