#![feature(binary_heap_into_iter_sorted)]
#![feature(array_chunks)]

use std::path::Path;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

fn main() {
    println!("Day 1 Part 1: {}", day_01::part_1(&read_input("day_01.txt")));
    println!("Day 1 Part 2: {}", day_01::part_2(&read_input("day_01.txt")));
    println!("Day 2 Part 1: {}", day_02::part_1(&read_input("day_02.txt")));
    println!("Day 2 Part 2: {}", day_02::part_2(&read_input("day_02.txt")));
    println!("Day 3 Part 1: {}", day_03::part_1(&read_input("day_03.txt")));
    println!("Day 3 Part 2: {}", day_03::part_2(&read_input("day_03.txt")));
    println!("Day 4 Part 1: {}", day_04::part_1(&read_input("day_04.txt")));
    println!("Day 4 Part 2: {}", day_04::part_2(&read_input("day_04.txt")));
    println!("Day 5 Part 1: {}", day_05::part_1(&read_input("day_05.txt")));
    println!("Day 5 Part 2: {}", day_05::part_2(&read_input("day_05.txt")));
    println!("Day 6 Part 1: {}", day_06::part_1(&read_input("day_06.txt")));
    println!("Day 6 Part 2: {}", day_06::part_2(&read_input("day_06.txt")));
    println!("Day 7 Part 1: {}", day_07::part_1(&read_input("day_07.txt")));
    println!("Day 7 Part 2: {}", day_07::part_2(&read_input("day_07.txt")));
    println!("Day 8 Part 1: {}", day_08::part_1(&read_input("day_08.txt")));
    println!("Day 8 Part 2: {}", day_08::part_2(&read_input("day_08.txt")));
    println!("Day 9 Part 1: {}", day_09::part_1(&read_input("day_09.txt")));
    println!("Day 9 Part 2: {}", day_09::part_2(&read_input("day_09.txt")));
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
