#![feature(binary_heap_into_iter_sorted)]
#![feature(array_chunks)]

use clap::Parser;
use std::path::Path;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

#[derive(Parser, Debug)]
struct Cli {
    day: Option<usize>,
    part: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        if let Some(part) = cli.part {
            solve(day, part);
        } else {
            solve(day, 1);
            solve(day, 2);
        }
    } else {
        solve(1, 1);
        solve(1, 2);
        solve(2, 1);
        solve(2, 2);
        solve(3, 1);
        solve(3, 2);
        solve(4, 1);
        solve(4, 2);
        solve(5, 1);
        solve(5, 2);
        solve(6, 1);
        solve(6, 2);
        solve(7, 1);
        solve(7, 2);
        solve(8, 1);
        solve(8, 2);
        solve(9, 1);
        solve(9, 2);
    }
}

fn solve(day: usize, part: usize) {
    let input = read_input(format!("day_{day:02}.txt"));

    let now = Instant::now();
    let solution = match (day, part) {
        (1, 1) => day_01::part_1(&input).to_string(),
        (1, 2) => day_01::part_2(&input).to_string(),
        (2, 1) => day_02::part_1(&input).to_string(),
        (2, 2) => day_02::part_2(&input).to_string(),
        (3, 1) => day_03::part_1(&input).to_string(),
        (3, 2) => day_03::part_2(&input).to_string(),
        (4, 1) => day_04::part_1(&input).to_string(),
        (4, 2) => day_04::part_2(&input).to_string(),
        (5, 1) => day_05::part_1(&input),
        (5, 2) => day_05::part_2(&input),
        (6, 1) => day_06::part_1(&input).to_string(),
        (6, 2) => day_06::part_2(&input).to_string(),
        (7, 1) => day_07::part_1(&input).to_string(),
        (7, 2) => day_07::part_2(&input).to_string(),
        (8, 1) => day_08::part_1(&input).to_string(),
        (8, 2) => day_08::part_2(&input).to_string(),
        (9, 1) => day_09::part_1(&input).to_string(),
        (9, 2) => day_09::part_2(&input).to_string(),
        _ => panic!(),
    };
    let elapsed = now.elapsed();

    println!("{:10}μs   Day {day} Part {part}: {solution}", elapsed.as_micros());
}

fn read_input(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(Path::new("input").join(path)).unwrap()
}

mod nom_complete {
    pub use nom::branch::*;
    pub use nom::bytes::complete::*;
    pub use nom::character::complete::*;
    pub use nom::character::*;
    pub use nom::combinator::*;
    pub use nom::multi::*;
    pub use nom::sequence::*;
    pub use nom::Finish;
    pub use nom::IResult;

    pub fn take_till_whitespace1(s: &str) -> IResult<&str, &str> {
        take_till1(char::is_whitespace)(s)
    }
}
