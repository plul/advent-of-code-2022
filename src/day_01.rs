//! Day 1: Calorie Counting
//!
//! https://adventofcode.com/2022/day/1

use std::collections::BinaryHeap;

pub fn part_1(input: &str) -> usize {
    let elves: Vec<Elf> = parser::parse(input);

    // For each elf, sum the calories of all the snacks he/she is carrying.
    let calorie_sums = elves.iter().map(|elf| elf.total_calories());

    // How many total calories is the elf that is carrying the most calories carrying?
    calorie_sums.max().unwrap()
}

pub fn part_2(input: &str) -> usize {
    let elves: Vec<Elf> = parser::parse(input);

    // For each elf, put the total number of calories into a max heap.
    let heap = elves
        .iter()
        .map(|elf| elf.total_calories())
        .collect::<BinaryHeap<usize>>();

    // Take out the three max calorie totals and sum them
    let solution = heap.into_iter_sorted().take(3).sum();

    debug_assert!(solution <= 3 * part_1(input));

    solution
}

struct Elf {
    /// Snacks carried by this elf.
    snacks: Vec<usize>,
}

impl Elf {
    /// Total number of callories carried by this elf
    fn total_calories(&self) -> usize {
        self.snacks.iter().sum()
    }
}

mod parser {
    use super::Elf;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Elf> {
        all_consuming(parse_elves)(s).unwrap().1
    }

    fn parse_elves(s: &str) -> IResult<&str, Vec<Elf>> {
        separated_list0(line_ending, parse_one_elf)(s)
    }

    fn parse_one_elf(s: &str) -> IResult<&str, Elf> {
        let line_with_snack = terminated(map_res(digit1, str::parse), line_ending);
        map(many1(line_with_snack), |snacks| Elf { snacks })(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 24000);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 45000);
}
