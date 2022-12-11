//! Day 11: Monkey in the Middle
//!
//! https://adventofcode.com/2022/day/11

use std::collections::VecDeque;

pub fn part_1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = parser::parse(input);

    for _round in 1..=20 {
        for monkey_idx in 0..monkeys.len() {
            loop {
                let monkey = &mut monkeys[monkey_idx];
                let Some(mut item) = monkey.items.pop_front() else { break; };
                monkey.inspected_items += 1;

                let rhs = match monkey.operation.rhs {
                    OperationRhs::Value(v) => v,
                    OperationRhs::Self_ => item.worry_level,
                };
                match monkey.operation.operation_type {
                    OperationType::AddAssign => {
                        item.worry_level += rhs;
                    }
                    OperationType::MulAssign => {
                        item.worry_level *= rhs;
                    }
                }

                item.worry_level /= 3;

                let throw_to = if is_divisible_by(item.worry_level, monkey.test_divisible_by) {
                    monkey.if_true_throw_to
                } else {
                    monkey.if_false_throw_to
                };

                let other_monkey = &mut monkeys[throw_to as usize];
                other_monkey.items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|m| std::cmp::Reverse(m.inspected_items));
    monkeys
        .iter()
        .take(2)
        .map(|m| m.inspected_items)
        .reduce(std::ops::Mul::mul)
        .unwrap()
}

pub fn part_2(input: &str) -> u64 {
    parser::parse(input);
    Default::default()
}

fn is_divisible_by(value: u64, divider: u64) -> bool {
    let x = value / divider;
    x * divider == value
}

struct Monkey {
    items: VecDeque<Item>,
    inspected_items: u64,
    operation: Operation,
    test_divisible_by: u64,
    if_true_throw_to: u64,
    if_false_throw_to: u64,
}

#[derive(Clone, Debug, Copy)]
struct Operation {
    operation_type: OperationType,
    rhs: OperationRhs,
}

#[derive(Clone, Debug, Copy)]
enum OperationType {
    AddAssign,
    MulAssign,
}

#[derive(Clone, Debug, Copy)]
enum OperationRhs {
    Value(u64),
    Self_,
}

#[derive(Clone, Debug, Copy)]
struct Item {
    worry_level: u64,
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Monkey> {
        all_consuming(separated_list0(line_ending, parse_monkey))(s)
            .unwrap()
            .1
    }

    fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
        let (s, _) = parse_monkey_line(s)?;
        let (s, starting_items) = parse_starting_items(s)?;
        let (s, operation) = parse_operation(s)?;
        let (s, _) = tag("  Test: divisible by ")(s)?;
        let (s, test_divisible_by) = terminated(u64, line_ending)(s)?;

        let (s, if_true_throw_to) =
            terminated(preceded(tag("    If true: throw to monkey "), u64), line_ending)(s)?;
        let (s, if_false_throw_to) =
            terminated(preceded(tag("    If false: throw to monkey "), u64), line_ending)(s)?;

        let monkey = Monkey {
            items: starting_items
                .into_iter()
                .map(|worry_level| Item { worry_level })
                .collect(),
            inspected_items: 0,
            operation,
            test_divisible_by,
            if_true_throw_to,
            if_false_throw_to,
        };
        Ok((s, monkey))
    }

    fn parse_monkey_line(s: &str) -> IResult<&str, ()> {
        let (s, _) = tag("Monkey ")(s)?;
        let (s, _) = digit1(s)?;
        let (s, _) = char(':')(s)?;
        let (s, _) = line_ending(s)?;
        Ok((s, ()))
    }
    #[test]
    fn test_parse_monkey_line() {
        parse_monkey_line("Monkey 0:\n").unwrap();
    }

    fn parse_starting_items(s: &str) -> IResult<&str, Vec<u64>> {
        let (s, _) = tag("  Starting items:")(s)?;
        terminated(preceded(char(' '), separated_list0(tag(", "), u64)), line_ending)(s)
    }
    #[test]
    fn test_parse_starting_items() {
        parse_starting_items("  Starting items: 79, 98\n").unwrap();
        parse_starting_items("  Starting items: 74\n").unwrap();
    }

    fn parse_operation(s: &str) -> IResult<&str, Operation> {
        let (s, _) = tag("  Operation: new = old ")(s)?;
        let (s, op) = one_of("+*")(s)?;
        let operation_type = match op {
            '+' => OperationType::AddAssign,
            '*' => OperationType::MulAssign,
            _ => unreachable!(),
        };
        let (s, _) = char(' ')(s)?;

        let rhs_value = map(u64, OperationRhs::Value);
        let rhs_self = value(OperationRhs::Self_, tag("old"));
        let (s, rhs) = alt((rhs_value, rhs_self))(s)?;

        let (s, _) = line_ending(s)?;

        let operation = Operation { operation_type, rhs };

        Ok((s, operation))
    }
    #[test]
    fn test_parse_operation() {
        parse_operation("  Operation: new = old * 19\n").unwrap();
        parse_operation("  Operation: new = old * old\n").unwrap();
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 10605);
}

// #[test]
// fn part_2_example() {
//     assert_eq!(part_2(EXAMPLE), 0);
// }
