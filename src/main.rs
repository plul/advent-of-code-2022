#![feature(binary_heap_into_iter_sorted)]

use std::path::Path;

mod day_01;
mod day_02;

fn main() {
    println!("Day 1 Part 1: {}", day_01::part_1(&read_input("day_01.txt")));
    println!("Day 1 Part 2: {}", day_01::part_2(&read_input("day_01.txt")));
    println!("Day 2 Part 1: {}", day_02::part_1(&read_input("day_02.txt")));
    println!("Day 2 Part 2: {}", day_02::part_2(&read_input("day_02.txt")));
}

fn read_input(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(Path::new("input").join(path)).unwrap()
}

#[cfg(test)]
mod examples {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn day_01_part_1_example() {
        let input = read_input("examples/day_01.txt");
        assert_eq!(day_01::part_1(&input), 24000);
    }

    #[test]
    fn day_01_part_2_example() {
        let input = read_input("examples/day_01.txt");
        assert_eq!(day_01::part_2(&input), 45000);
    }

    #[test]
    fn day_02_part_1_example() {
        let input = read_input("examples/day_02.txt");
        assert_eq!(day_02::part_1(&input), 15);
    }

    #[test]
    fn day_02_part_2_example() {
        let input = read_input("examples/day_02.txt");
        assert_eq!(day_02::part_2(&input), 12);
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
