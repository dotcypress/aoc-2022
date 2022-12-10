use aoc_2022::*;
use std::env;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const PUZZLES: [Puzzle; 10] = [
    day1::PUZZLE,
    day2::PUZZLE,
    day3::PUZZLE,
    day4::PUZZLE,
    day5::PUZZLE,
    day6::PUZZLE,
    day7::PUZZLE,
    day8::PUZZLE,
    day9::PUZZLE,
    day10::PUZZLE,
];

fn main() {
    if let Some(day) = env::args().skip(1).next() {
        let day = day
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or_default();
        if day <= PUZZLES.len() {
            let puzzle = &PUZZLES[day - 1];
            let name = puzzle.name;
            let (part_one, part_two) = (puzzle.solve)();
            println!("{name}");
            println!("\nPart One:\n{part_one}");
            println!("\nPart Two:\n{part_two}");
        } else {
            println!("Day {day} puzzle not found");
        }
    } else {
        println!("╔══════════════════════════════════╦════════════════╦════════════════╗");
        println!("║ 🦀 Advent of Code 2022           ║       Part One ║       Part Two ║");
        println!("╠══════════════════════════════════╬════════════════╬════════════════╣");
        for puzzle in PUZZLES.iter().rev() {
            let name = puzzle.name;
            let (mut part_one, mut part_two) = (puzzle.solve)();
            part_one.truncate(14);
            part_two.truncate(14);
            println!("║ {: <32} ║ {: >14} ║ {: >14} ║", name, part_one, part_two);
        }
        println!("╚══════════════════════════════════╩════════════════╩════════════════╝");
    }
}
