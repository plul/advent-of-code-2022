//! Day 6: Tuning Trouble
//!
//! https://adventofcode.com/2022/day/6

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub fn part_1(input: &str) -> usize {
    find_unique_window(input, 4)
}

pub fn part_2(input: &str) -> usize {
    find_unique_window(input, 14)
}

fn find_unique_window(input: &str, window_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut iter_front = chars.iter();
    let iter_tail = chars.iter();

    let mut window: BTreeMap<char, usize> = BTreeMap::new();

    for _ in 0..window_size {
        let c = iter_front.next().unwrap();
        *window.entry(*c).or_default() += 1;
    }
    if window.len() == window_size {
        return window_size;
    }

    for (front, (iter_tail_idx, tail)) in iter_front.zip(iter_tail.enumerate()) {
        match window.entry(*tail) {
            Entry::Vacant(_) => {
                unreachable!()
            }
            Entry::Occupied(mut e) => {
                if *e.get() == 1 {
                    e.remove();
                } else {
                    *e.get_mut() -= 1;
                }
            }
        }
        *window.entry(*front).or_default() += 1;

        if window.len() == window_size {
            return window_size + (iter_tail_idx + 1);
        }
    }

    panic!()
}

#[test]
fn part_1_examples() {
    assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn part_2_examples() {
    assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
