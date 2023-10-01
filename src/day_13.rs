//! Day 13: Distress Signal
//!
//! https://adventofcode.com/2022/day/13

pub fn part_1(input: &str) -> usize {
    let pairs = parser::parse(input);
    pairs
        .iter()
        .enumerate()
        .filter(|(_idx, pair)| pair.first_packet < pair.second_packet)
        .map(|(idx, _pair)| idx + 1)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let pairs = parser::parse(input);
    let mut packets: Vec<Value> = pairs.into_iter().flat_map(|p| [p.first_packet, p.second_packet].into_iter()).collect();

    let divider_packet_1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let divider_packet_2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);

    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());

    packets.sort();

    let (divider_packet_1_idx, _) = packets.iter().enumerate().find(|(_, p)| **p == divider_packet_1).unwrap();
    let (divider_packet_2_idx, _) = packets.iter().enumerate().find(|(_, p)| **p == divider_packet_2).unwrap();

    (divider_packet_1_idx + 1) * (divider_packet_2_idx + 1)
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    first_packet: Value,
    second_packet: Value,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Value {
    List(Vec<Self>),
    Integer(u64),
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::List(a), Value::List(b)) => a.cmp(b),
            (Value::List(_), Value::Integer(_)) => {
                let b = Value::List(vec![other.clone()]);
                self.cmp(&b)
            }
            (Value::Integer(_), Value::List(_)) => {
                let a = Value::List(vec![self.clone()]);
                a.cmp(other)
            }
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Pair> {
        all_consuming(separated_list0(line_ending, parse_pair))(s).unwrap().1
    }

    fn parse_pair(s: &str) -> IResult<&str, Pair> {
        let (s, (first_packet, second_packet)) = pair(parse_packet_line, parse_packet_line)(s)?;
        let p = Pair { first_packet, second_packet };
        Ok((s, p))
    }

    pub(super) fn parse_packet_line(s: &str) -> IResult<&str, Value> {
        terminated(parse_value, line_ending)(s)
    }

    fn parse_value(s: &str) -> IResult<&str, Value> {
        let list = map(delimited(char('['), separated_list0(char(','), parse_value), char(']')), Value::List);
        let integer = map(u64, Value::Integer);
        alt((list, integer))(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

#[cfg(test)]
static EXAMPLE_PART_2: &str = "\
[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 13);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 140);
}

#[test]
fn ord() {
    use crate::nom_complete::*;
    let mut packets = all_consuming(many0(parser::parse_packet_line))(EXAMPLE_PART_2).unwrap().1;
    let expected = packets.clone();
    packets.sort();
    assert_eq!(packets, expected);
}
