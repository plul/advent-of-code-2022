//! Day 3: Rucksack Reorganization
//!
//! https://adventofcode.com/2022/day/3

use std::collections::BTreeSet;

pub fn part_1(input: &str) -> usize {
    let groups: Vec<Group> = parser::parse(input);

    groups
        .iter()
        .flat_map(|group| group.rucksacks.iter())
        .map(Rucksack::common_item_between_compartments)
        .map(priority)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let groups: Vec<Group> = parser::parse(input);

    groups
        .iter()
        .map(Group::common_item_between_rucksacks)
        .map(priority)
        .sum()
}

fn priority(c: char) -> usize {
    if ('a'..='z').contains(&c) {
        let a = 'a' as usize;
        return c as usize - a + 1;
    }
    if ('A'..='Z').contains(&c) {
        let a = 'A' as usize;
        return c as usize - a + 27;
    }
    panic!()
}

#[derive(Debug)]
struct Rucksack {
    compartment_1: Vec<char>,
    compartment_2: Vec<char>,
}
impl Rucksack {
    fn common_item_between_compartments(&self) -> char {
        let compartment_1_set = self.compartment_1.iter().copied().collect::<BTreeSet<char>>();

        let mut intersection = self
            .compartment_2
            .iter()
            .copied()
            .filter(|item| compartment_1_set.contains(item))
            .collect::<BTreeSet<char>>()
            .into_iter();

        let common_item = intersection.next().unwrap();
        assert!(intersection.next().is_none());

        common_item
    }
}

struct Group {
    rucksacks: [Rucksack; 3],
}
impl Group {
    fn common_item_between_rucksacks(&self) -> char {
        let common_items = self
            .rucksacks
            .iter()
            .map(|r| {
                let mut set = BTreeSet::new();
                set.extend(&r.compartment_1);
                set.extend(&r.compartment_2);
                set
            })
            .fold::<Option<BTreeSet<char>>, _>(None, |intersection, set| {
                if let Some(intersection) = intersection {
                    Some(intersection.intersection(&set).cloned().collect())
                } else {
                    Some(set)
                }
            })
            .unwrap();

        assert_eq!(common_items.len(), 1);
        common_items.into_iter().next().unwrap()
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Group> {
        let main_parser = parse_groups;
        all_consuming(main_parser)(s).unwrap().1
    }

    fn parse_groups(s: &str) -> IResult<&str, Vec<Group>> {
        many0(parse_group)(s)
    }

    fn parse_group(s: &str) -> IResult<&str, Group> {
        let (s, rucksacks) = count(parse_rucksack, 3)(s)?;
        let group = Group {
            rucksacks: rucksacks.try_into().unwrap(),
        };
        Ok((s, group))
    }

    fn parse_rucksack(s: &str) -> IResult<&str, Rucksack> {
        let (s, chars) = verify(alpha1, |s: &str| {
            // is even:
            s.len() & 1 == 0
        })(s)?;
        let (s, _) = line_ending(s)?;

        let compartment_1 = &chars[0..chars.len() / 2];
        let compartment_2 = &chars[chars.len() / 2..];
        debug_assert_eq!(compartment_1.len(), compartment_2.len());

        let rucksack = Rucksack {
            compartment_1: compartment_1.chars().collect(),
            compartment_2: compartment_2.chars().collect(),
        };

        Ok((s, rucksack))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 157);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 70);
}
