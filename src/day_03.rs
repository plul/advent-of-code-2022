//! Day 3: Rucksack Reorganization
//!
//! https://adventofcode.com/2022/day/3

use std::collections::BTreeSet;

/// Part 1
pub fn part_1(input: &str) -> usize {
    let rucksacks = parser::parse(input);

    rucksacks
        .into_iter()
        .flat_map(|group| group.rucksacks.into_iter())
        .map(|r| {
            let compartment_1_set = r.compartment_1.into_iter().collect::<BTreeSet<char>>();
            let compartment_2_set = r.compartment_2.into_iter().collect::<BTreeSet<char>>();

            let mut intersection = compartment_1_set.intersection(&compartment_2_set);
            let item = intersection.next().unwrap();
            assert!(intersection.next().is_none());

            priority(*item)
        })
        .sum()
}

/// Part 2
pub fn part_2(input: &str) -> usize {
    let groups = parser::parse(input);

    groups
        .into_iter()
        .map(|group| {
            let mut intersection = group
                .rucksacks
                .into_iter()
                .map(|r| {
                    let mut set = BTreeSet::new();
                    set.extend(r.compartment_1);
                    set.extend(r.compartment_2);
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

            assert_eq!(intersection.len(), 1);
            let badge: char = intersection.pop_first().unwrap();

            priority(badge)
        })
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

struct Group {
    rucksacks: [Rucksack; 3],
}

mod parser {
    #[allow(unused_imports)]
    use super::*;
    use nom::character::complete::*;
    use nom::combinator::*;
    use nom::multi::*;
    use nom::IResult;

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
