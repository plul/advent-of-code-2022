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

#[cfg(test)]
mod examples {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn day_01_examples() {
        let input = read_input("examples/day_01.txt");
        assert_eq!(day_01::part_1(&input), 24000);
        assert_eq!(day_01::part_2(&input), 45000);
    }

    #[test]
    fn day_02_examples() {
        let input = read_input("examples/day_02.txt");
        assert_eq!(day_02::part_1(&input), 15);
        assert_eq!(day_02::part_2(&input), 12);
    }

    #[test]
    fn day_03_examples() {
        let input = read_input("examples/day_03.txt");
        assert_eq!(day_03::part_1(&input), 157);
        assert_eq!(day_03::part_2(&input), 70);
    }

    #[test]
    fn day_04_examples() {
        let input = read_input("examples/day_04.txt");
        assert_eq!(day_04::part_1(&input), 2);
        assert_eq!(day_04::part_2(&input), 4);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn day_01_sanity_check() {
        let input = read_input("day_01.txt");
        assert!(3 * day_01::part_1(&input) >= day_01::part_2(&input));
    }
}
