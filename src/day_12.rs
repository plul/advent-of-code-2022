//! Day 12: Hill Climbing Algorithm
//!
//! https://adventofcode.com/2022/day/12

use crate::lib::graph::dijkstra;
use crate::lib::graph::Graph;
use crate::lib::graph::GraphEdge;
use crate::lib::graph::GraphEdgeCost;
use std::borrow::Cow;

pub fn part_1(input: &str) -> usize {
    let heightmap = parser::parse(input);
    let start_node = heightmap.find_start();
    let end_node = heightmap.find_end();
    dijkstra::shortest_path(&heightmap, &start_node, &end_node).unwrap()
}

pub fn part_2(input: &str) -> usize {
    let heightmap = parser::parse(input);
    let end_node = heightmap.find_end();
    heightmap
        .find_coords_with_height('a')
        .iter()
        .flat_map(|start_node| dijkstra::shortest_path(&heightmap, &start_node, &end_node))
        .min()
        .unwrap()
}

type RowIdx = usize;
type ColIdx = usize;
type Coord = (RowIdx, ColIdx);

/// 2D rectangular height map.
struct Heightmap {
    n_rows: usize,
    n_cols: usize,

    /// Nodes, where index of (row, col) from top left is: row * width + col.
    nodes: Vec<Node>,
}
impl<'g> Graph<'g> for Heightmap {
    type Node = Coord;

    type Edge = Edge;

    /// Neighboring nodes that are at most 1 higher than current node.
    fn edges(&self, from: &Self::Node) -> Vec<Self::Edge> {
        let from_node = self.get_node(*from).unwrap();
        let from_height = from_node.height();

        let mut v = vec![];
        if from.0 > 0 {
            v.push((from.0 - 1, from.1));
        }
        v.push((from.0 + 1, from.1));
        if from.1 > 0 {
            v.push((from.0, from.1 - 1));
        }
        v.push((from.0, from.1 + 1));

        v.into_iter()
            .filter(|coord| {
                let Some(node) = self.get_node(*coord) else { return false; };
                from_height + 1 >= node.height()
            })
            .map(|coord| Edge { to: coord })
            .collect()
    }
}

struct Edge {
    to: Coord,
}
impl<'g> GraphEdge<'g> for Edge {
    type Node = Coord;
    fn to(&self) -> Cow<'g, Self::Node> {
        Cow::Owned(self.to)
    }
}
impl GraphEdgeCost for Edge {
    type Cost = usize;
    fn cost(&self) -> Self::Cost {
        1
    }
}

impl Heightmap {
    fn get_node(&self, coord: Coord) -> Option<Node> {
        let (row, col) = coord;
        if row < self.n_rows && col < self.n_cols {
            Some(self.nodes[row * self.n_cols + col])
        } else {
            None
        }
    }

    fn find_coords_with_height(&self, c: char) -> Vec<Coord> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_idx, node)| match node {
                Node::Normal(h) => c == *h,
                Node::Start => c == 'a',
                Node::End => c == 'z',
            })
            .map(|(idx, _node)| {
                let row = idx / self.n_cols;
                let col = idx % self.n_cols;
                (row, col)
            })
            .collect()
    }

    fn find_start(&self) -> Coord {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_idx, node)| matches!(node, Node::Start))
            .map(|(idx, _node)| {
                let row = idx / self.n_cols;
                let col = idx % self.n_cols;
                (row, col)
            })
            .unwrap()
    }

    fn find_end(&self) -> Coord {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_idx, node)| matches!(node, Node::End))
            .map(|(idx, _node)| {
                let row = idx / self.n_cols;
                let col = idx % self.n_cols;
                (row, col)
            })
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Normal(char),
    Start,
    End,
}
impl Node {
    fn height(&self) -> usize {
        let c: char = match self {
            Node::Normal(char) => *char,
            Node::Start => 'a',
            Node::End => 'z',
        };
        c as usize
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Heightmap {
        all_consuming(parse_heightmap)(s).unwrap().1
    }

    fn parse_heightmap(s: &str) -> IResult<&str, Heightmap> {
        let (s, rows) = many1(parse_row)(s)?;

        let heightmap = Heightmap {
            n_rows: rows.len(),
            n_cols: rows.first().unwrap().len(),
            nodes: rows.into_iter().flat_map(|v| v.into_iter()).collect(),
        };

        Ok((s, heightmap))
    }

    fn parse_row(s: &str) -> IResult<&str, Vec<Node>> {
        let start = value(Node::Start, char('S'));
        let end = value(Node::End, char('E'));
        let normal = map(satisfy(|c| ('a'..='z').contains(&c)), Node::Normal);
        terminated(many1(alt((start, end, normal))), line_ending)(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 31);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 29);
}
