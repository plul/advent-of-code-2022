//! Day 18: Boiling Boulders
//!
//! https://adventofcode.com/2022/day/18

use crate::lib::graph::Graph;
use crate::lib::graph::GraphEdge;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let lava_cubes = parser::parse(input);
    let droplet = Droplet {
        lava_cubes: lava_cubes.into_iter().collect(),
    };
    droplet.lava_cubes.iter().map(|c| 6 - droplet.edges(c).len()).sum()
}

pub fn part_2(input: &str) -> usize {
    let mut is_air_bubble: HashMap<Cube, bool> = HashMap::new();

    // To determine if an air cube is in an air bubble, check if air cube has already been explored (is in is_air_bubble).
    // If not, explore air with DFS.
    // If DFS reaches a node that is known to be in outside air (is_air_bubble: false), then all the expored air cubes are in outside air.
    // If DFS does not reach a node that is known to be in outside air, then all air cubes explored form their own air bubble.
    // Bounds: (-1, -1, -1) to (max(x) + 1, max(y) + 1, max(z) + 1).

    // (-1, -1, -1) is known to be in outside air.
    is_air_bubble.insert(Cube(-1,-1,-1), false);

    parser::parse(input);
    Default::default()
}

struct Droplet {
    lava_cubes: HashSet<Cube>,
}

impl<'g> Graph<'g> for Droplet {
    type Node = Cube;

    type Edge = Edge;

    fn edges(&'g self, from: &Self::Node) -> Vec<Self::Edge> {
        let possible_neighbors = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];
        possible_neighbors
            .into_iter()
            .map(|d| Cube(from.0 + d.0, from.1 + d.1, from.2 + d.2))
            .filter(|n| self.lava_cubes.contains(n))
            .map(|to| Edge { to })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube(i64, i64, i64);

struct Edge {
    to: Cube,
}

impl<'g> GraphEdge<'g> for Edge {
    type Node = Cube;

    fn to(&self) -> Cow<'g, Self::Node> {
        Cow::Owned(self.to)
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Cube> {
        all_consuming(terminated(separated_list1(line_ending, parse_cube), multispace0))(s)
            .unwrap()
            .1
    }

    fn parse_cube(s: &str) -> IResult<&str, Cube> {
        let (s, x) = i64(s)?;
        let (s, _) = char(',')(s)?;
        let (s, y) = i64(s)?;
        let (s, _) = char(',')(s)?;
        let (s, z) = i64(s)?;
        Ok((s, Cube(x, y, z)))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 64);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 58);
}
