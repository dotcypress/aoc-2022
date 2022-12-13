use crate::*;

puzzle!("Day 12: Hill Climbing Algorithm", Solver, 31, 29);

struct Solver {
    start: Point,
    end: Point,
    map: Field<Square>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let mut start = Point::zero();
        let mut end = Point::zero();
        let mut map = Field::parse(input, |row| {
            row.chars().map(|e| Square::new(e as _)).collect()
        });
        for (point, s) in map.iter_mut() {
            match s.elevation {
                b'S' => {
                    start = point;
                    s.elevation = b'a'
                }
                b'E' => {
                    end = point;
                    s.elevation = b'z'
                }
                _ => {}
            }
        }
        Self { map, start, end }
    }

    fn part_one(mut self) -> usize {
        self.visit(self.end, 0);
        self.map.get(self.start).unwrap().steps
    }

    fn part_two(&mut self) -> usize {
        self.visit(self.end, 0);
        self.map
            .iter()
            .filter_map(|(_, s)| {
                if s.elevation == b'a' {
                    Some(s.steps)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    fn visit(&mut self, point: Point, cnt: usize) {
        if let Some(s) = self.map.get_mut(point) {
            if cnt >= s.steps {
                return;
            }
            s.steps = cnt;
        } else {
            return;
        }
        for neighbor in point.close_neighbors() {
            let elevation = self.map.get(point).map(|s| s.elevation).unwrap();
            let next_elevation = self.map.get(neighbor).map(|s| s.elevation).unwrap_or(0);
            if next_elevation >= elevation || next_elevation == (elevation - 1) {
                self.visit(neighbor, cnt + 1);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Square {
    elevation: u8,
    steps: usize,
}

impl Square {
    fn new(elevation: u8) -> Self {
        Self {
            elevation,
            steps: usize::MAX,
        }
    }
}
