use crate::*;

puzzle!("Day 5: Supply Stacks", Solver, "CMZ", "MCD");

struct Solver {
    crates: Crates,
    ops: Vec<Op>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let (crates, ops) = input.split_once("\n\n").unwrap();
        let ops = ops.lines().map(Op::parse).collect();
        let crates = crates
            .lines()
            .rev()
            .skip(1)
            .fold(Crates::default(), |mut acc, line| {
                let crates: Vec<char> = line.chars().collect();
                for (idx, chunk) in crates.chunks(4).enumerate() {
                    if chunk[1].is_alphabetic() {
                        acc.push(idx, chunk[1]);
                    }
                }
                acc
            });
        Self { crates, ops }
    }

    fn part_one(mut self) -> String {
        for op in self.ops {
            self.crates.single_move(op);
        }
        self.crates.top()
    }

    fn part_two(mut self) -> String {
        for op in self.ops {
            self.crates.batch_move(op);
        }
        self.crates.top()
    }
}

#[derive(Debug, Default)]
pub struct Op {
    amount: usize,
    from: usize,
    to: usize,
}

impl Op {
    pub fn parse(line: &str) -> Self {
        let mut inst = line.split(' ');
        inst.next();
        let amount = inst.next().unwrap().parse::<usize>().unwrap();
        inst.next();
        let from = inst.next().unwrap().parse::<usize>().unwrap() - 1;
        inst.next();
        let to = inst.next().unwrap().parse::<usize>().unwrap() - 1;
        Self { amount, from, to }
    }
}

#[derive(Debug, Default)]
pub struct Crates {
    layout: Vec<Vec<char>>,
}

impl Crates {
    pub fn push(&mut self, idx: usize, value: char) {
        while self.layout.len() <= idx {
            self.layout.push(vec![])
        }
        self.layout[idx].push(value)
    }

    fn top(self) -> String {
        self.layout.iter().map(|v| v.last().unwrap()).collect()
    }

    pub fn single_move(&mut self, op: Op) {
        for _ in 0..op.amount {
            let c = self.layout[op.from].pop().unwrap();
            self.layout[op.to].push(c)
        }
    }

    pub fn batch_move(&mut self, op: Op) {
        let src = &mut self.layout[op.from];
        let chunk = src.split_off(src.len() - op.amount);
        self.layout[op.to].extend(chunk);
    }
}
