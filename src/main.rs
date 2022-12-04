#![feature(binary_heap_into_iter_sorted)]

use std::path::Path;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    println!("Day 1 Part 1: {}", day_01::part_1(&read_input("day_01.txt")));
    println!("Day 1 Part 2: {}", day_01::part_2(&read_input("day_01.txt")));
    println!("Day 2 Part 1: {}", day_02::part_1(&read_input("day_02.txt")));
    println!("Day 2 Part 2: {}", day_02::part_2(&read_input("day_02.txt")));
    println!("Day 3 Part 1: {}", day_03::part_1(&read_input("day_03.txt")));
    println!("Day 3 Part 2: {}", day_03::part_2(&read_input("day_03.txt")));
    println!("Day 4 Part 1: {}", day_04::part_1(&read_input("day_04.txt")));
    println!("Day 4 Part 2: {}", day_04::part_2(&read_input("day_04.txt")));
}

fn read_input(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(Path::new("input").join(path)).unwrap()
}
