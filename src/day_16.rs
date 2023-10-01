//! Day 16: Proboscidea Volcanium
//!
//! https://adventofcode.com/2022/day/16
//!
//! This is not the prettiest or most performant of solutions.
//! Could definitely be cleaned up!

use crate::lib::graph::dijkstra;
use crate::lib::graph::Graph;
use crate::lib::graph::GraphEdge;
use crate::lib::graph::GraphEdgeCost;
use std::borrow::Cow;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_1(input: &str) -> i64 {
    let network_of_pipes = parser::parse(input);
    let minutes_to_move_from_a_valve_to_any_other_valve = compute_minutes_to_move_from_a_valve_to_any_other_valve(&network_of_pipes);
    let mut opened_valves: HashSet<&str> = HashSet::new();

    let mut actors = vec![Actor {
        minutes: 30,
        name_of_current_valve: "AA",
    }];

    dfs(
        &network_of_pipes,
        &minutes_to_move_from_a_valve_to_any_other_valve,
        &mut actors,
        &mut opened_valves,
    )
}

pub fn part_2(input: &str) -> i64 {
    let network_of_pipes = parser::parse(input);
    let minutes_to_move_from_a_valve_to_any_other_valve = compute_minutes_to_move_from_a_valve_to_any_other_valve(&network_of_pipes);
    let mut opened_valves: HashSet<&str> = HashSet::new();

    let mut actors = vec![
        Actor {
            minutes: 26,
            name_of_current_valve: "AA",
        },
        Actor {
            minutes: 26,
            name_of_current_valve: "AA",
        },
    ];

    dfs(
        &network_of_pipes,
        &minutes_to_move_from_a_valve_to_any_other_valve,
        &mut actors,
        &mut opened_valves,
    )
}

fn compute_minutes_to_move_from_a_valve_to_any_other_valve<'a>(network_of_pipes: &NetworkOfPipes<'a>) -> HashMap<(&'a str, &'a str), u64> {
    let mut minutes_to_move_from_a_valve_to_any_other_valve: HashMap<(&str, &str), u64> = HashMap::new();
    for valve in network_of_pipes.valves.values() {
        for other in network_of_pipes.valves.values() {
            if valve == other {
                continue;
            }
            let minutes = dijkstra::shortest_path(network_of_pipes, valve, other).expect("no path");
            minutes_to_move_from_a_valve_to_any_other_valve.insert((valve.name, other.name), minutes);
        }
    }
    minutes_to_move_from_a_valve_to_any_other_valve
}

fn dfs<'a>(
    network_of_pipes: &'a NetworkOfPipes<'_>,
    minutes_to_move_from_a_valve_to_any_other_valve: &HashMap<(&'a str, &'a str), u64>,
    actors: &mut Vec<Actor<'a>>,
    opened_valves: &mut HashSet<&'a str>,
) -> i64 {
    if actors.is_empty() {
        return 0;
    }

    let actor = actors.pop().unwrap();

    // Max score by not going further with present actor.
    let score_1 = dfs(network_of_pipes, minutes_to_move_from_a_valve_to_any_other_valve, actors, opened_valves);

    // Max score by still going further with present actor.
    let score_2 = network_of_pipes
        .valves
        .values()
        .filter(|v| v.flow_rate > 0)
        .filter(|v| v.name != actor.name_of_current_valve)
        .filter_map(|valve| {
            if opened_valves.contains(valve.name) {
                return None;
            }

            let mut minutes = actor.minutes;

            // Move to valve
            minutes -= minutes_to_move_from_a_valve_to_any_other_valve[&(actor.name_of_current_valve, valve.name)] as i64;
            // Open valve
            minutes -= 1;
            if minutes < 0 {
                return None;
            }

            opened_valves.insert(valve.name);
            actors.push(Actor {
                minutes,
                name_of_current_valve: valve.name,
            });
            let total_pressure_release =
                minutes * valve.flow_rate as i64 + dfs(network_of_pipes, minutes_to_move_from_a_valve_to_any_other_valve, actors, opened_valves);
            actors.pop();
            opened_valves.remove(valve.name);

            Some(total_pressure_release)
        })
        .max()
        .unwrap_or(0);
    actors.push(actor);

    max(score_1, score_2)
}

#[derive(Debug)]
struct Actor<'a> {
    minutes: i64,
    name_of_current_valve: &'a str,
}

struct NetworkOfPipes<'i> {
    /// Valves indexed by name.
    valves: HashMap<&'i str, Valve<'i>>,
}

impl<'s, 'i> Graph<'s> for NetworkOfPipes<'i>
where
    'i: 's,
{
    type Node = Valve<'i>;
    type Edge = Edge<'s, 'i>;

    fn edges(&'s self, from: &Self::Node) -> Vec<Self::Edge> {
        from.tunnels_to
            .iter()
            .map(|name| Edge {
                to: self.valves.get(name).unwrap(),
            })
            .collect()
    }
}

struct Edge<'r, 'i> {
    to: &'r Valve<'i>,
}
impl<'r, 'i> GraphEdge<'r> for Edge<'r, 'i> {
    type Node = Valve<'i>;

    fn to(&self) -> Cow<'r, Self::Node> {
        Cow::Borrowed(self.to)
    }
}
impl<'r, 'i> GraphEdgeCost for Edge<'r, 'i> {
    type Cost = u64;

    fn cost(&self) -> Self::Cost {
        1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u64,
    tunnels_to: Vec<&'a str>,
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> NetworkOfPipes<'_> {
        let p = map(many1(parse_valve_line), |valves| NetworkOfPipes {
            valves: valves.into_iter().map(|v| (v.name, v)).collect(),
        });
        all_consuming(terminated(p, multispace0))(s).unwrap().1
    }

    fn parse_valve_line(s: &str) -> IResult<&str, Valve<'_>> {
        let (s, _) = tag("Valve ")(s)?;
        let (s, name) = parse_valve_name(s)?;
        let (s, _) = tag(" has flow rate=")(s)?;
        let (s, flow_rate) = u64(s)?;
        let (s, _) = tag("; tunnel")(s)?;
        let (s, _) = opt(char('s'))(s)?;
        let (s, _) = tag(" lead")(s)?;
        let (s, _) = opt(char('s'))(s)?;
        let (s, _) = tag(" to valve")(s)?;
        let (s, _) = opt(char('s'))(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, tunnels_to) = separated_list1(tag(", "), parse_valve_name)(s)?;
        let (s, _) = line_ending(s)?;

        let valve = Valve { name, flow_rate, tunnels_to };

        Ok((s, valve))
    }

    fn parse_valve_name(s: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_ascii_uppercase())(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 1651);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 1707);
}
