use crate::*;
use std::collections::VecDeque;

puzzle!("Day 11: Monkey in the Middle", Solver, 10605, 2713310158);

struct Solver {
    monkeys: Vec<Monkey>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        Self {
            monkeys: input.split("\n\n").map(Monkey::parse).collect(),
        }
    }

    fn part_one(self) -> usize {
        self.monkey_business(20, 3)
    }

    fn part_two(self) -> usize {
        self.monkey_business(10_000, 1)
    }

    fn monkey_business(mut self, rounds: usize, worry_div: usize) -> usize {
        let magic: usize = self.monkeys.iter().map(|m| m.test.0).product();
        let mut monkey_business: Vec<_> = self.monkeys.iter().map(|_| 0_usize).collect();
        for _ in 0..rounds {
            for idx in 0..self.monkeys.len() {
                while let Some(item) = self.monkeys[idx].items.pop_front() {
                    monkey_business[idx] += 1;
                    let worry = (self.monkeys[idx].op.perform(item) % magic) / worry_div;
                    let rec = if (worry % self.monkeys[idx].test.0) == 0 {
                        self.monkeys[idx].test.1
                    } else {
                        self.monkeys[idx].test.2
                    };
                    self.monkeys[rec].items.push_back(worry);
                }
            }
        }
        monkey_business.sort();
        monkey_business.iter().rev().take(2).product()
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

impl Op {
    pub fn perform(&self, x: usize) -> usize {
        match self {
            Op::Add(val) => x + val,
            Op::Mul(val) => x * val,
            Op::Square => x * x,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test: (usize, usize, usize),
}

impl Monkey {
    pub fn parse(data: &str) -> Self {
        // Welcome to my personal, hand-picked `unwrap` collection
        let mut lines = data.lines();
        let _ = lines.next().unwrap();
        let list = lines.next().unwrap().split_at(18).1;
        let items = list.split(", ").map(|x| x.parse().unwrap()).collect();
        let op = match lines.next().unwrap().split_at(19).1 {
            "old * old" => Op::Square,
            other => match other.split_at(4).1.split_once(' ') {
                Some(("+", x)) => Op::Add(x.parse().unwrap()),
                Some(("*", x)) => Op::Mul(x.parse().unwrap()),
                _ => unreachable!(),
            },
        };
        let test = lines.next().unwrap().split_at(21).1.parse().unwrap();
        let pos_action = lines.next().unwrap().split_at(29).1.parse().unwrap();
        let neg_action = lines.next().unwrap().split_at(30).1.parse().unwrap();
        Self {
            items,
            op,
            test: (test, pos_action, neg_action),
        }
    }
}
