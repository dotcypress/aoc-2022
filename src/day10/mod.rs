use crate::*;

puzzle!(
    "Day 10: Cathode-Ray Tube",
    Solver,
    13140,
    "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n"
);

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

    fn part_one(self) -> isize {
        let mut x = 1;
        let mut strength = 0;
        for (cycle, op) in self.prog.iter().enumerate() {
            let pc = cycle as isize + 1;
            if pc % 40 == 20 {
                strength += x * pc as isize;
            }
            x += op;
        }
        strength
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
            if ray == 39 {
                crt.push('\n');
            }
            x += op;
        }
        crt.iter().collect()
    }
}
