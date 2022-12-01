use std::collections::BinaryHeap;

/// https://adventofcode.com/2022/day/1#part1
pub fn part_1() -> usize {
    let elves: Vec<Elf> = parser::parse_input(&read_input());

    // For each elf, sum the calories of all the snacks he/she is carrying.
    let calorie_sums = elves.iter().map(|elf| elf.total_calories());

    // How many total calories is the elf that is carrying the most calories carrying?
    calorie_sums.max().unwrap()
}

/// https://adventofcode.com/2022/day/1#part2
pub fn part_2() -> usize {
    let elves: Vec<Elf> = parser::parse_input(&read_input());

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

fn read_input() -> String {
    std::fs::read_to_string("input/day_1.txt").unwrap()
}

mod parser {
    use super::Elf;
    use nom::character::complete::digit1;
    use nom::character::complete::newline;
    use nom::combinator::all_consuming;
    use nom::combinator::map;
    use nom::combinator::map_res;
    use nom::multi::many1;
    use nom::multi::separated_list0;
    use nom::sequence::terminated;
    use nom::IResult;

    pub(super) fn parse_input(s: &str) -> Vec<Elf> {
        all_consuming(separated_list0(newline, parse_one_elf))(s)
            .unwrap()
            .1
    }

    fn parse_one_elf(s: &str) -> IResult<&str, Elf> {
        let line_with_snack = terminated(map_res(digit1, str::parse), newline);
        map(many1(line_with_snack), |snacks| Elf { snacks })(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert!(3 * part_1() >= part_2());
    }
}
