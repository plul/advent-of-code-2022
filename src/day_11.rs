//! Day 11: Monkey in the Middle
//!
//! https://adventofcode.com/2022/day/11

use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Div;
use std::ops::Mul;

pub fn part_1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = parser::parse(input);

    for _round in 1..=20 {
        for monkey_idx in 0..monkeys.len() {
            loop {
                let monkey = &mut monkeys[monkey_idx];
                let Some(mut item) = monkey.items.pop_front() else {
                    break;
                };
                monkey.inspected_items += 1;

                let rhs = match monkey.expression.operand {
                    Operand::Value(v) => v,
                    Operand::Old => item.worry_level.current_value,
                };
                match monkey.expression.operation {
                    Operation::AddAssign => {
                        item.worry_level.current_value += rhs;
                    }
                    Operation::MulAssign => {
                        item.worry_level.current_value *= rhs;
                    }
                }

                // Relief
                item.worry_level.current_value /= 3;

                let throw_to = if is_divisible_by(&item.worry_level.current_value, &monkey.test_divisible_by) {
                    monkey.if_true_throw_to
                } else {
                    monkey.if_false_throw_to
                };

                let other_monkey = &mut monkeys[throw_to as usize];
                other_monkey.items.push_back(item);
            }
        }
    }

    monkey_business(monkeys)
}

pub fn part_2(input: &str) -> u64 {
    let mut monkeys = parser::parse(input);

    let mut dividers: Vec<u64> = monkeys.iter().map(|m| m.test_divisible_by).collect();
    dividers.sort();
    dividers.dedup();

    for item in monkeys.iter_mut().flat_map(|m| m.items.iter_mut()) {
        for divider in dividers.iter().copied() {
            let remainder_if_divided = item.worry_level.current_value % divider;
            item.worry_level.remainder_if_divided_by.insert(divider, remainder_if_divided);
        }
    }

    for _round in 1..=10_000 {
        for monkey_idx in 0..monkeys.len() {
            loop {
                let monkey = &mut monkeys[monkey_idx];
                let Some(mut item) = monkey.items.pop_front() else {
                    break;
                };
                monkey.inspected_items += 1;

                for (divider, remainder) in item.worry_level.remainder_if_divided_by.iter_mut() {
                    let new_remainder = match (monkey.expression.operation, monkey.expression.operand) {
                        (Operation::AddAssign, Operand::Value(v)) => (*remainder + v) % divider,
                        (Operation::AddAssign, Operand::Old) => {
                            // Adding old value is the same as multiplying by 2.
                            (*remainder * 2) % divider
                        }
                        (Operation::MulAssign, Operand::Value(v)) => (*remainder * v) % divider,
                        (Operation::MulAssign, Operand::Old) => {
                            // This is the math-heavy case.
                            //
                            // w: current worry-level, that is unknown.
                            // d: divider (given in the input, per monkey).
                            // r: the remainder of w divided by d, i.e. `x  mod d`. We know this value.
                            //
                            // Goal: Find `w^2  mod d`, the new remainder to be stored.
                            //
                            // First, note that
                            //   w = r + n×d
                            // for some natural n.
                            // Therefore,
                            //   w^2                                mod d
                            //   = (r + n × d)^2                    mod d
                            //   = r^2 + n^2 × d^2 + 2 × r × n × d  mod d
                            //   = r^2 + 0         + 0              mod d
                            //   = r^2                              mod d
                            let r_squared = *remainder * *remainder;
                            r_squared % divider
                        }
                    };

                    *remainder = new_remainder;
                }

                let test: bool = *item.worry_level.remainder_if_divided_by.get(&monkey.test_divisible_by).unwrap() == 0;
                let throw_to = if test { monkey.if_true_throw_to } else { monkey.if_false_throw_to };

                let other_monkey = &mut monkeys[throw_to as usize];
                other_monkey.items.push_back(item);
            }
        }
    }

    monkey_business(monkeys)
}

fn monkey_business(mut monkeys: Vec<Monkey>) -> u64 {
    monkeys.sort_by_key(|m| std::cmp::Reverse(m.inspected_items));
    monkeys.iter().take(2).map(|m| m.inspected_items).reduce(std::ops::Mul::mul).unwrap()
}

fn is_divisible_by<'a, T>(value: &'a T, divider: &'a T) -> bool
where
    &'a T: Div<&'a T>,
    <&'a T as Div<&'a T>>::Output: Mul<&'a T>,
    <<&'a T as Div<&'a T>>::Output as Mul<&'a T>>::Output: PartialEq<T>,
{
    let x = value / divider;
    x * divider == *value
}

#[derive(Clone, Debug)]
struct WorryLevel {
    /// Current value.
    ///
    /// This is unsed for part 2!
    current_value: u64,

    /// Map from denominators of potential divisions, and the remainder should that division by carried out.
    ///
    /// This is unused for part 1!
    remainder_if_divided_by: HashMap<u64, u64>,
}

struct Monkey {
    items: VecDeque<Item>,
    inspected_items: u64,
    expression: ArithmeticExpression,
    test_divisible_by: u64,
    if_true_throw_to: u64,
    if_false_throw_to: u64,
}

/// All of them start with new = old, so that part is omitted.
#[derive(Clone, Debug, Copy)]
struct ArithmeticExpression {
    operation: Operation,
    operand: Operand,
}

#[derive(Clone, Debug, Copy)]
enum Operation {
    AddAssign,
    MulAssign,
}

#[derive(Clone, Debug, Copy)]
enum Operand {
    Value(u64),
    Old,
}

#[derive(Clone, Debug)]
struct Item {
    worry_level: WorryLevel,
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Monkey> {
        all_consuming(separated_list0(line_ending, parse_monkey))(s).unwrap().1
    }

    fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
        let (s, _) = parse_monkey_line(s)?;
        let (s, starting_items) = parse_starting_items(s)?;
        let (s, expression) = parse_arithmetic_expression(s)?;
        let (s, _) = tag("  Test: divisible by ")(s)?;
        let (s, test_divisible_by) = terminated(u64, line_ending)(s)?;

        let (s, if_true_throw_to) = terminated(preceded(tag("    If true: throw to monkey "), u64), line_ending)(s)?;
        let (s, if_false_throw_to) = terminated(preceded(tag("    If false: throw to monkey "), u64), line_ending)(s)?;

        let items = starting_items
            .into_iter()
            .map(|current_value| Item {
                worry_level: WorryLevel {
                    current_value,
                    remainder_if_divided_by: HashMap::default(),
                },
            })
            .collect();

        let monkey = Monkey {
            items,
            inspected_items: 0,
            expression,
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

    fn parse_arithmetic_expression(s: &str) -> IResult<&str, ArithmeticExpression> {
        let (s, _) = tag("  Operation: new = old ")(s)?;
        let (s, op) = one_of("+*")(s)?;
        let operation = match op {
            '+' => Operation::AddAssign,
            '*' => Operation::MulAssign,
            _ => unreachable!(),
        };
        let (s, _) = char(' ')(s)?;

        let rhs_value = map(u64, Operand::Value);
        let rhs_self = value(Operand::Old, tag("old"));
        let (s, rhs) = alt((rhs_value, rhs_self))(s)?;

        let (s, _) = line_ending(s)?;

        let expression = ArithmeticExpression { operation, operand: rhs };

        Ok((s, expression))
    }
    #[test]
    fn test_parse_operation() {
        parse_arithmetic_expression("  Operation: new = old * 19\n").unwrap();
        parse_arithmetic_expression("  Operation: new = old * old\n").unwrap();
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

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 2713310158);
}
