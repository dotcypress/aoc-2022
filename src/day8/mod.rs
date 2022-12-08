use crate::*;

puzzle!("Day 8: Treetop Tree House", Solver, 21, 8);

pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}
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

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.map[y][x];
        let init = (false, 0);
        let kernel = |(blocked, cnt), t: &usize| {
            if blocked {
                (blocked, cnt)
            } else if *t >= height {
                (true, cnt + 1)
            } else {
                (false, cnt + 1)
            }
        };

        let u = self.look_from(x, y, Dir::Up).iter().fold(init, kernel);
        let r = self.look_from(x, y, Dir::Right).iter().fold(init, kernel);
        let d = self.look_from(x, y, Dir::Down).iter().fold(init, kernel);
        let l = self.look_from(x, y, Dir::Left).iter().fold(init, kernel);

        u.1 * r.1 * d.1 * l.1
    }

    fn is_visible_from_edges(&self, x: usize, y: usize) -> bool {
        let height = self.map[y][x];
        let pred = |h: &&usize| **h >= height;

        let u = self.look_from(x, y, Dir::Up).iter().filter(pred).count();
        let r = self.look_from(x, y, Dir::Right).iter().filter(pred).count();
        let d = self.look_from(x, y, Dir::Down).iter().filter(pred).count();
        let l = self.look_from(x, y, Dir::Left).iter().filter(pred).count();

        u == 0 || r == 0 || d == 0 || l == 0
    }

    fn points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
    }

    fn look_from(&self, x: usize, y: usize, dir: Dir) -> Vec<usize> {
        match dir {
            Dir::Up => self.map.iter().take(y).map(|c| c[x]).rev().collect(),
            Dir::Right => self.map[y].iter().skip(x + 1).copied().collect(),
            Dir::Down => self.map.iter().skip(y + 1).map(|c| c[x]).collect(),
            Dir::Left => self.map[y].iter().take(x).rev().copied().collect(),
        }
    }
}
