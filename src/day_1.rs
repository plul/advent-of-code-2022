use std::collections::BinaryHeap;

/// https://adventofcode.com/2022/day/1#part1
pub fn part_1() -> usize {
    let elves: Vec<Elf> = parse_input();

    // For each elf, sum the calories of all the snacks he/she is carrying.
    let calorie_sums = elves.iter().map(|elf| elf.total_calories());

    // How many total calories is the elf that is carrying the most calories carrying?
    calorie_sums.max().unwrap()
}

/// https://adventofcode.com/2022/day/1#part2
pub fn part_2() -> usize {
    let elves: Vec<Elf> = parse_input();

    // For each elf, put the total number of calories into a max heap.
    let heap = elves
        .iter()
        .map(|elf| elf.total_calories())
        .collect::<BinaryHeap<usize>>();

    // Take out the three max calorie totals and sum them
    heap.into_iter_sorted().take(3).sum()
}

fn parse_input() -> Vec<Elf> {
    let input: String = std::fs::read_to_string("input/day_1.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let split_on_empty_lines = lines.split(|s| s.is_empty());
    split_on_empty_lines
        .map(|lines_for_one_elf| {
            let snacks = lines_for_one_elf
                .iter()
                .map(|s| {
                    let snack: usize = s.parse().unwrap();
                    snack
                })
                .collect();
            Elf { snacks }
        })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert!(3 * part_1() >= part_2());
    }
}
