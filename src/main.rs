#[macro_use] extern crate maplit;

mod day16;
mod day17;
mod day18;
mod day19;

use std::env;
use std::fs;

use aoc_helper::{AocDay, Puzzle};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number = &args[1].parse::<u8>().unwrap();
    let part_number = &args[2].parse::<u8>().unwrap();
    let maybe_file = args.get(3);

    run(*day_number, *part_number, maybe_file);
}

fn run(day_number: u8, part_number: u8, maybe_file: Option<&String>) {
    let mut puzzle = create_puzzle(day_number, part_number);
    let mut day = AocDay::new(2015, day_number);

    if let Some(file) = maybe_file {
        let contents = fs::read_to_string(file)
            .expect("Can't read test file!");

        puzzle.examples(&[contents]);
        day.test(&puzzle);
    } else {
        day.run(&puzzle).expect("Failed to run!");
    }
}

fn create_puzzle(day_number: u8, part_number: u8) -> Puzzle<String, i128> {
    match day_number {
        16 => Puzzle::new(part_number, day16::solver(part_number)),
        17 => Puzzle::new(part_number, day17::solver(part_number)),
        18 => Puzzle::new(part_number, day18::solver(part_number)),
        19 => Puzzle::new(part_number, day19::solver(part_number)),
        _ => panic!("Unknown day!"),
    }
}
