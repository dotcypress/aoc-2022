use crate::*;

puzzle!("Day 8: Treetop Tree House", Solver, 21, 8);

struct Solver {
    map: Vec<Vec<usize>>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let map = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as _).collect())
            .collect();
        Self { map }
    }

    fn part_one(self) -> usize {
        self.points()
            .filter(|(x, y)| self.is_visible_from_edges(*x, *y))
            .count()
    }

    fn part_two(self) -> usize {
        self.points()
            .map(|(x, y)| self.scenic_score(x, y))
            .max()
            .unwrap_or_default()
    }

    fn is_visible_from_edges(&self, x: usize, y: usize) -> bool {
        let height = self.map[y][x];
        self.look_around(x, y)
            .iter()
            .map(|view| view.iter().filter(|h: &&usize| **h >= height).count())
            .any(|blockers| blockers == 0)
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.map[y][x];
        self.look_around(x, y)
            .iter()
            .map(|view| {
                view.iter()
                    .fold((false, 0), |(blocked, cnt), t: &usize| {
                        if blocked {
                            (blocked, cnt)
                        } else if *t >= height {
                            (true, cnt + 1)
                        } else {
                            (false, cnt + 1)
                        }
                    })
                    .1
            })
            .product()
    }

    fn points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
    }

    fn look_around(&self, x: usize, y: usize) -> Vec<Vec<usize>> {
        vec![
            self.map.iter().take(y).map(|c| c[x]).rev().collect(),
            self.map[y].iter().skip(x + 1).copied().collect(),
            self.map.iter().skip(y + 1).map(|c| c[x]).collect(),
            self.map[y].iter().take(x).rev().copied().collect(),
        ]
    }
}
