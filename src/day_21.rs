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
    let monkey_map: HashMap<String, Monkey> =
        monkeys.into_iter().map(|monkey| (monkey.name.clone(), monkey)).collect();
    let monkey_graph = MonkeyGraph { monkey_map };
    let root = monkey_graph.monkey_map.get("root").unwrap();

    let topologically_sorted_monkeys = graph::topological_sort::topological_sort(&monkey_graph, root).unwrap();
    debug_assert_eq!(topologically_sorted_monkeys.last().unwrap().name, "root");

    let mut monkey_yell = HashMap::<&str, i64>::new();
    for monkey in topologically_sorted_monkeys.iter() {
        let n = match &monkey.job {
            Job::SpecificNumber(n) => *n,
            Job::MathOperation { monkey_1, operator, monkey_2 } => {
                let monkey_1_yell = monkey_yell.get(monkey_1.as_str()).unwrap();
                let monkey_2_yell = monkey_yell.get(monkey_2.as_str()).unwrap();
                match operator {
                    Operator::Add => monkey_1_yell + monkey_2_yell,
                    Operator::Subtract => monkey_1_yell - monkey_2_yell,
                    Operator::Multiply => monkey_1_yell * monkey_2_yell,
                    Operator::Divide => monkey_1_yell / monkey_2_yell,
                }
            },
        };
        monkey_yell.insert(&monkey.name, n);
    }

    *monkey_yell.get("root").unwrap()
}

pub fn part_2(input: &str) -> usize {
    parser::parse(input);
    Default::default()
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
            } => vec![
                &self.monkey_map[monkey_1],
                &self.monkey_map[monkey_2],
            ],
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

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 152);
}

// #[test]
// fn part_2_example() {
//     assert_eq!(part_2(EXAMPLE), 0);
// }
