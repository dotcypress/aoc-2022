use crate::*;

puzzle!("Day 2: Rock Paper Scissors", Solver, 15, 12);

struct Solver {
    strategy: Vec<Round>,
}

impl Solver {
    fn ingest(input: &str) -> Self {
        let strategy = input.lines().map(Round::parse).collect();
        Self { strategy }
    }

    fn part_one(self) -> usize {
        self.strategy.iter().map(|r| r.naive_score()).sum()
    }

    fn part_two(self) -> usize {
        self.strategy.iter().map(|r| r.score()).sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveOutcome {
    Loose,
    Draw,
    Win,
}

impl MoveOutcome {
    pub fn parse(val: &str) -> Self {
        match val {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("AAAAAA"),
        }
    }

    pub fn play(opponent_move: Move, my_move: Move) -> Self {
        match (opponent_move, my_move) {
            (o, m) if o == m => Self::Draw,
            (Move::Scissors, Move::Rock)
            | (Move::Paper, Move::Scissors)
            | (Move::Rock, Move::Paper) => Self::Win,
            (_, _) => Self::Loose,
        }
    }

    pub fn choose_move(&self, opponent_move: Move) -> Move {
        match (self, opponent_move) {
            (MoveOutcome::Draw, m) => m,
            (MoveOutcome::Loose, Move::Rock) => Move::Scissors,
            (MoveOutcome::Loose, Move::Paper) => Move::Rock,
            (MoveOutcome::Loose, Move::Scissors) => Move::Paper,
            (MoveOutcome::Win, Move::Rock) => Move::Paper,
            (MoveOutcome::Win, Move::Paper) => Move::Scissors,
            (MoveOutcome::Win, Move::Scissors) => Move::Rock,
        }
    }

    pub fn score(&self) -> usize {
        match self {
            MoveOutcome::Loose => 0,
            MoveOutcome::Draw => 3,
            MoveOutcome::Win => 6,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn parse(val: &str) -> Self {
        match val {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("AAAAAA"),
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
pub struct Round {
    opponent_move: Move,
    my_move: Move,
    outcome: MoveOutcome,
}

impl Round {
    pub fn parse(line: &str) -> Self {
        let (opp, my) = line.split_once(' ').unwrap();
        Self {
            opponent_move: Move::parse(opp),
            my_move: Move::parse(my),
            outcome: MoveOutcome::parse(my),
        }
    }

    pub fn naive_score(&self) -> usize {
        let opponent_move = self.opponent_move;
        let my_move = self.my_move;
        MoveOutcome::play(opponent_move, my_move).score() + my_move.score()
    }

    pub fn score(&self) -> usize {
        let opponent_move = self.opponent_move;
        let my_move = self.outcome.choose_move(opponent_move);
        MoveOutcome::play(opponent_move, my_move).score() + my_move.score()
    }
}
