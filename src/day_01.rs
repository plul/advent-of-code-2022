//! Day 1: Calorie Counting
//!
//! https://adventofcode.com/2022/day/1

use std::collections::BinaryHeap;

/// Part 1
pub fn part_1(input: &str) -> usize {
    let elves: Vec<Elf> = parser::parse(input);

    // For each elf, sum the calories of all the snacks he/she is carrying.
    let calorie_sums = elves.iter().map(|elf| elf.total_calories());

    // How many total calories is the elf that is carrying the most calories carrying?
    calorie_sums.max().unwrap()
}

/// Part 2
pub fn part_2(input: &str) -> usize {
    let elves: Vec<Elf> = parser::parse(input);

    // For each elf, put the total number of calories into a max heap.
    let heap = elves
        .iter()
        .map(|elf| elf.total_calories())
        .collect::<BinaryHeap<usize>>();

    // Take out the three max calorie totals and sum them
    heap.into_iter_sorted().take(3).sum()
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
    use nom::character::complete::digit1;
    use nom::character::complete::line_ending;
    use nom::combinator::all_consuming;
    use nom::combinator::map;
    use nom::combinator::map_res;
    use nom::multi::many1;
    use nom::multi::separated_list0;
    use nom::sequence::terminated;
    use nom::IResult;

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