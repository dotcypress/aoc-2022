use aoc_2022::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const PUZZLES: [Puzzle; 9] = [
    day1::PUZZLE,
    day2::PUZZLE,
    day3::PUZZLE,
    day4::PUZZLE,
    day5::PUZZLE,
    day6::PUZZLE,
    day7::PUZZLE,
    day8::PUZZLE,
    day9::PUZZLE,
];

fn main() {
    println!("╔══════════════════════════════════╦════════════════╦════════════════╗");
    println!("║ 🦀 Advent of Code 2022           ║       Part One ║       Part Two ║");
    println!("╠══════════════════════════════════╬════════════════╬════════════════╣");
    for puzzle in PUZZLES.iter().rev() {
        let name = puzzle.name;
        let (part_one, part_two) = (puzzle.solve)();
        println!("║ {: <32} ║ {: >14} ║ {: >14} ║", name, part_one, part_two);
    }
    println!("╚══════════════════════════════════╩════════════════╩════════════════╝");
}
