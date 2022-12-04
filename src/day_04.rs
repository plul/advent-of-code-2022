//! Day 4: Camp Cleanup
//!
//! https://adventofcode.com/2022/day/4

use std::ops::RangeInclusive;

pub fn part_1(input: &str) -> usize {
    parser::parse(input)
        .into_iter()
        .filter(|[range_1, range_2]| range_1.fully_contains(range_2) || range_2.fully_contains(range_1))
        .count()
}

pub fn part_2(input: &str) -> usize {
    parser::parse(input)
        .into_iter()
        .filter(|[range_1, range_2]| range_1.overlap_at_all(range_2))
        .count()
}

trait RangeFullyContains {
    fn fully_contains(&self, other: &Self) -> bool;
    fn overlap_at_all(&self, other: &Self) -> bool;
}
impl<T> RangeFullyContains for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn fully_contains(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlap_at_all(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end()) || other.fully_contains(self)
    }
}

mod parser {
    #[allow(unused_imports)]
    use super::*;
    use nom::character::complete::*;
    use nom::combinator::*;
    use nom::multi::*;
    use nom::sequence::*;
    use nom::IResult;
    use std::ops::RangeInclusive;

    pub(super) fn parse(s: &str) -> Vec<[RangeInclusive<u64>; 2]> {
        let main_parser = many0(terminated(parse_range_pair, line_ending));
        all_consuming(main_parser)(s).unwrap().1
    }

    fn parse_range_pair(s: &str) -> IResult<&str, [RangeInclusive<u64>; 2]> {
        let (s, (range_1, range_2)) = separated_pair(parse_range, char(','), parse_range)(s)?;
        Ok((s, [range_1, range_2]))
    }

    fn parse_range(s: &str) -> IResult<&str, RangeInclusive<u64>> {
        let (s, (from, to)) = separated_pair(u64, char('-'), u64)(s)?;
        Ok((s, from..=to))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 2);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 4);
}
