//! Day 20: Grove Positioning System
//!
//! https://adventofcode.com/2022/day/20

use tap::Pipe;

pub fn part_1(input: &str) -> i64 {
    let numbers = parser::parse(input);
    let mixed = mix_numbers(&numbers, 1);
    grove_coordinates(&mixed).into_iter().sum()
}

pub fn part_2(input: &str) -> i64 {
    let mut numbers = parser::parse(input);
    let decryption_key = 811589153;
    numbers.iter_mut().for_each(|n| *n *= decryption_key);
    log::debug!("Decrypted numbers: {numbers:?}");
    let mixed = mix_numbers(&numbers, 10);
    grove_coordinates(&mixed).into_iter().sum()
}

fn grove_coordinates(mixed: &[i64]) -> [i64; 3] {
    let mixed_index_of_zero = mixed
        .iter()
        .enumerate()
        .find(|&(_, &n)| n == 0)
        .map(|(idx, _)| idx)
        .unwrap();

    let x = (mixed_index_of_zero + 1000)
        .rem_euclid(mixed.len())
        .pipe(|i| mixed[i]);

    let y = (mixed_index_of_zero + 2000)
        .rem_euclid(mixed.len())
        .pipe(|i| mixed[i]);

    let z = (mixed_index_of_zero + 3000)
        .rem_euclid(mixed.len())
        .pipe(|i| mixed[i]);

    [x, y, z]
}

fn mix_numbers(numbers: &[i64], rounds: usize) -> Vec<i64> {
    let mut map_from_mixed_index_to_original_index: Vec<usize> = (0..).take(numbers.len()).collect();

    for _ in 0..rounds {
        for (original_index, n) in numbers.iter().enumerate() {
            let mixed_index = map_from_mixed_index_to_original_index
                .iter()
                .enumerate()
                .find(|&(_, &orig_idx)| original_index == orig_idx)
                .map(|(idx, _)| idx)
                .unwrap();
            let new_mixed_index = (mixed_index as i64 + n).rem_euclid(numbers.len() as i64 - 1);
            debug_assert!(new_mixed_index >= 0);

            let i = map_from_mixed_index_to_original_index.remove(mixed_index);
            debug_assert_eq!(original_index, i);
            map_from_mixed_index_to_original_index.insert(new_mixed_index as usize, i);
        }
    }

    let mut mixed = vec![0; numbers.len()];
    for (mixed_index, original_index) in map_from_mixed_index_to_original_index.iter().enumerate() {
        mixed[mixed_index] = numbers[*original_index];
    }
    mixed
}

mod parser {
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<i64> {
        all_consuming(terminated(separated_list1(line_ending, main_parser), multispace0))(s)
            .unwrap()
            .1
    }

    fn main_parser(s: &str) -> IResult<&str, i64> {
        i64(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
1
2
-3
3
-2
0
4
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 4 - 3 + 2);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 1623178306);
}
