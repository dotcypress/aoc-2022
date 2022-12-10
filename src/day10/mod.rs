use crate::*;

puzzle!("Day 10: Cathode-Ray Tube", Solver, 13140, "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....");

struct Solver {
    prog: Vec<isize>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let prog = input
            .lines()
            .flat_map(|line| match line {
                "noop" => vec![0],
                _ => {
                    let (_, val) = line.split_at(5);
                    vec![0, val.parse().unwrap()]
                }
            })
            .collect();
        Self { prog }
    }

    fn part_one(self) -> usize {
        let (_, _, ret) = self.prog.iter().fold((1, 1, 0), |(pc, x, ret), val| {
            if pc == 20 || (pc - 20) % 40 == 0 {
                (pc + 1, x + val, ret + x * pc as isize)
            } else {
                (pc + 1, x + val, ret)
            }
        });
        ret as _
    }

    fn part_two(self) -> String {
        let mut x = 1;
        let mut crt = Vec::new();
        for (cycle, op) in self.prog.iter().enumerate() {
            let ray = cycle as isize % 40;
            if x >= ray - 1 && x <= ray + 1 {
                crt.push('#');
            } else {
                crt.push('.');
            }
            x += op;
        }
        crt.iter().collect()
    }
}
