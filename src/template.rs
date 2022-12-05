//! Day n: <title>
//!
//! https://adventofcode.com/2022/day/n

pub fn part_1(input: &str) -> usize {
    parser::parse(input);
    Default::default()
}

pub fn part_2(input: &str) -> usize {
    parser::parse(input);
    Default::default()
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<()> {
        all_consuming(many0(main_parser))(s).unwrap().1
    }

    fn main_parser(s: &str) -> IResult<&str, ()> {
        Ok((s, ()))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\

";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 0);
}

// #[test]
// fn part_2_example() {
//     assert_eq!(part_2(EXAMPLE), 0);
// }
