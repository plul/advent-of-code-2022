//! Day 21: Monkey Math
//!
//! https://adventofcode.com/2022/day/21

use crate::lib::graph;
use crate::lib::graph::Graph;
use crate::lib::graph::GraphEdge;
use std::borrow::Cow;
use std::collections::HashMap;

pub fn part_1(input: &str) -> i64 {
    let monkeys = parser::parse(input);

    // Map name of monkey to its job
    let monkey_map: HashMap<String, Monkey> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name.clone(), monkey))
        .collect();
    let monkey_graph = MonkeyGraph { monkey_map };
    let root = monkey_graph.monkey_map.get("root").unwrap();

    let topologically_sorted_monkeys =
        graph::topological_sort::topological_sort(&monkey_graph, root).unwrap();
    debug_assert_eq!(topologically_sorted_monkeys.last().unwrap().name, "root");

    let mut monkey_yell = HashMap::<&str, i64>::new();
    for monkey in topologically_sorted_monkeys.iter() {
        let n = match &monkey.job {
            Job::SpecificNumber(n) => *n,
            Job::MathOperation {
                monkey_1,
                operator,
                monkey_2,
            } => {
                let monkey_1_yell = monkey_yell.get(monkey_1.as_str()).unwrap();
                let monkey_2_yell = monkey_yell.get(monkey_2.as_str()).unwrap();
                operator.monkey_math(*monkey_1_yell, *monkey_2_yell)
            }
        };
        monkey_yell.insert(&monkey.name, n);
    }

    *monkey_yell.get("root").unwrap()
}

pub fn part_2(input: &str) -> i64 {
    let monkeys = parser::parse(input);

    let monkeys: HashMap<String, Monkey> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name.clone(), monkey))
        .collect();

    let root = &monkeys["root"];
    let Job::MathOperation { monkey_1, monkey_2, .. } = &root.job else { panic!(); };
    let monkey_1 = &monkeys[monkey_1];
    let monkey_2 = &monkeys[monkey_2];

    let mut cache = HashMap::<&Monkey, i64>::new();
    let monkey_1_n: Option<i64> = cached_descend(&monkeys, monkey_1, &mut cache, Some("humn"));
    let monkey_2_n: Option<i64> = cached_descend(&monkeys, monkey_2, &mut cache, Some("humn"));
    let expected_result: i64 = monkey_1_n.or(monkey_2_n).unwrap();

    let indeterminate_monkey = if monkey_1_n.is_none() { monkey_1 } else { monkey_2 };

    let humn_yell = what_should_humn_yell_to_make_this_monkey_yell_n(
        &monkeys,
        &cache,
        indeterminate_monkey,
        expected_result,
    );

    #[cfg(debug_assertions)]
    {
        let mut monkeys = monkeys.clone();
        let humn = monkeys.get_mut("humn").unwrap();
        humn.job = Job::SpecificNumber(humn_yell);
        let monkey_1 = &monkeys[&monkey_1.name];
        let monkey_2 = &monkeys[&monkey_2.name];

        let yell_override = ("humn", humn_yell);
        let monkey_1_yell = brute_descend(&monkeys, monkey_1, Some(yell_override)).unwrap();
        let monkey_2_yell = brute_descend(&monkeys, monkey_2, Some(yell_override)).unwrap();

        debug_assert_eq!(monkey_1_yell, monkey_2_yell);
    }

    humn_yell
}

fn what_should_humn_yell_to_make_this_monkey_yell_n(
    monkeys: &HashMap<String, Monkey>,
    cache: &HashMap<&Monkey, i64>,
    monkey: &Monkey,
    n: i64,
) -> i64 {
    if monkey.name == "humn" {
        return n;
    }

    let humn_should_yell = match &monkey.job {
        Job::SpecificNumber(_) => panic!(),
        Job::MathOperation {
            monkey_1,
            operator,
            monkey_2,
        } => {
            let monkey_1 = &monkeys[monkey_1];
            let monkey_2 = &monkeys[monkey_2];
            let monkey_1_yell = cache.get(monkey_1).copied();
            let monkey_2_yell = cache.get(monkey_2).copied();

            let humn_should_yell = match (monkey_1_yell, operator, monkey_2_yell) {
                (None, Operator::Add, Some(x)) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_1, n - x)
                }
                (None, Operator::Subtract, Some(x)) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_1, n + x)
                }
                (None, Operator::Multiply, Some(x)) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_1, n / x)
                }
                (None, Operator::Divide, Some(x)) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_1, n * x)
                }
                (Some(x), Operator::Add, None) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_2, n - x)
                }
                (Some(x), Operator::Subtract, None) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_2, x - n)
                }
                (Some(x), Operator::Multiply, None) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_2, n / x)
                }
                (Some(x), Operator::Divide, None) => {
                    what_should_humn_yell_to_make_this_monkey_yell_n(monkeys, cache, monkey_2, x / n)
                }

                (Some(_), _, Some(_)) => panic!(),
                (None, _, None) => {
                    todo!("This might be pretty hard to figure out, as neither branch is known")
                }
            };

            #[cfg(debug_assertions)]
            {
                let yell_override = ("humn", humn_should_yell);
                let monkey_1_yell = brute_descend(monkeys, monkey_1, Some(yell_override)).unwrap();
                let monkey_2_yell = brute_descend(monkeys, monkey_2, Some(yell_override)).unwrap();
                let what_would_this_monkey_yell = operator.monkey_math(monkey_1_yell, monkey_2_yell);
                debug_assert_eq!(what_would_this_monkey_yell, n);
            }

            humn_should_yell
        }
    };

    humn_should_yell
}

fn cached_descend<'g>(
    monkeys: &'g HashMap<String, Monkey>,
    monkey: &'g Monkey,
    cache: &mut HashMap<&'g Monkey, i64>,
    abort_if_name: Option<&str>,
) -> Option<i64> {
    if Some(monkey.name.as_str()) == abort_if_name {
        return None;
    }
    let yell = match &monkey.job {
        Job::SpecificNumber(n) => *n,
        Job::MathOperation {
            monkey_1,
            operator,
            monkey_2,
        } => {
            let monkey_1 = &monkeys[monkey_1];
            let monkey_2 = &monkeys[monkey_2];

            let monkey_1_yell = cache
                .get(monkey_1)
                .copied()
                .or_else(|| cached_descend(monkeys, monkey_1, cache, abort_if_name));
            let monkey_2_yell = cache
                .get(monkey_2)
                .copied()
                .or_else(|| cached_descend(monkeys, monkey_2, cache, abort_if_name));

            let monkey_1_yell = monkey_1_yell?;
            let monkey_2_yell = monkey_2_yell?;

            operator.monkey_math(monkey_1_yell, monkey_2_yell)
        }
    };
    cache.insert(monkey, yell);
    Some(yell)
}

// No caching, and ability to override
fn brute_descend<'g>(
    monkeys: &'g HashMap<String, Monkey>,
    monkey: &'g Monkey,
    yell_override: Option<(&str, i64)>,
) -> Option<i64> {
    if let Some((name, yell)) = yell_override {
        if name == monkey.name {
            return Some(yell);
        }
    }

    let yell = match &monkey.job {
        Job::SpecificNumber(n) => *n,
        Job::MathOperation {
            monkey_1,
            operator,
            monkey_2,
        } => {
            let monkey_1 = &monkeys[monkey_1];
            let monkey_2 = &monkeys[monkey_2];

            let monkey_1_yell = brute_descend(monkeys, monkey_1, yell_override);
            let monkey_2_yell = brute_descend(monkeys, monkey_2, yell_override);

            let monkey_1_yell = monkey_1_yell?;
            let monkey_2_yell = monkey_2_yell?;

            operator.monkey_math(monkey_1_yell, monkey_2_yell)
        }
    };
    Some(yell)
}

struct MonkeyGraph {
    monkey_map: HashMap<String, Monkey>,
}

impl<'g> Graph<'g> for MonkeyGraph {
    type Node = Monkey;
    type Edge = &'g Monkey;

    fn edges(&'g self, from: &Self::Node) -> Vec<Self::Edge> {
        match &from.job {
            Job::SpecificNumber(_) => vec![],
            Job::MathOperation {
                monkey_1,
                operator: _,
                monkey_2,
            } => vec![&self.monkey_map[monkey_1], &self.monkey_map[monkey_2]],
        }
    }
}

impl<'g> GraphEdge<'g> for &'g Monkey {
    type Node = Monkey;

    fn to(&self) -> std::borrow::Cow<'g, Self::Node> {
        Cow::Borrowed(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Monkey {
    name: String,
    job: Job,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Job {
    SpecificNumber(i64),
    MathOperation {
        monkey_1: String,
        operator: Operator,
        monkey_2: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl Operator {
    fn monkey_math(&self, monkey_1_yell: i64, monkey_2_yell: i64) -> i64 {
        match self {
            Operator::Add => monkey_1_yell + monkey_2_yell,
            Operator::Subtract => monkey_1_yell - monkey_2_yell,
            Operator::Multiply => monkey_1_yell * monkey_2_yell,
            Operator::Divide => monkey_1_yell / monkey_2_yell,
        }
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Monkey> {
        all_consuming(terminated(
            separated_list1(line_ending, parse_monkey),
            multispace0,
        ))(s)
        .unwrap()
        .1
    }

    fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
        let (s, name) = terminated(alpha1, tag(": "))(s)?;

        let (s, job) = alt((parse_job_specific, parse_job_operation))(s)?;

        let monkey = Monkey {
            name: name.to_owned(),
            job,
        };
        Ok((s, monkey))
    }

    fn parse_job_specific(s: &str) -> IResult<&str, Job> {
        map(i64, Job::SpecificNumber)(s)
    }

    fn parse_job_operation(s: &str) -> IResult<&str, Job> {
        let (s, monkey_1) = alpha1(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, operator) = parse_operator(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, monkey_2) = alpha1(s)?;

        let job = Job::MathOperation {
            monkey_1: monkey_1.to_owned(),
            operator,
            monkey_2: monkey_2.to_owned(),
        };
        Ok((s, job))
    }

    fn parse_operator(s: &str) -> IResult<&str, Operator> {
        let add = value(Operator::Add, char('+'));
        let subtract = value(Operator::Subtract, char('-'));
        let multiply = value(Operator::Multiply, char('*'));
        let divide = value(Operator::Divide, char('/'));
        alt((add, subtract, multiply, divide))(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

static ADDITIONAL_EXAMPLE_PART_2: &str = "\
root: juli + josi
juli: amee + alex
amee: buki * abby
buki: 5
abby: 4
alex: 4
josi: benj / mark
benj: 360
mark: emly - humn
emly: 34
humn: 0
";

#[test]
fn part_1_example() {
    env_logger::builder().is_test(true).parse_default_env().init();
    assert_eq!(part_1(EXAMPLE), 152);
}

#[test]
fn part_2_example() {
    env_logger::builder().is_test(true).parse_default_env().init();
    assert_eq!(part_2(EXAMPLE), 301);
}

#[test]
fn part_2_additional_example() {
    env_logger::builder().is_test(true).parse_default_env().init();
    assert_eq!(part_2(ADDITIONAL_EXAMPLE_PART_2), 19);
}
