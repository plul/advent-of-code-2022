//! Day 12: Hill Climbing Algorithm
//!
//! https://adventofcode.com/2022/day/12

pub fn part_1(input: &str) -> usize {
    let heightmap = parser::parse(input);
    let start = heightmap.find_start();
    shortest_path::shortest_path(&heightmap, start).unwrap()
}

pub fn part_2(input: &str) -> usize {
    let heightmap = parser::parse(input);
    heightmap
        .find_coords_with_height('a')
        .iter()
        .flat_map(|start| shortest_path::shortest_path(&heightmap, *start))
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
        let start: Coord = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_idx, node)| matches!(node, Node::Start))
            .map(|(idx, _node)| {
                let row = idx / self.n_cols;
                let col = idx % self.n_cols;
                (row, col)
            })
            .unwrap();

        debug_assert_eq!(self.get_node(start), Some(Node::Start));

        start
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

/// Neighboring nodes that are at most 1 higher than current node.
fn reachable(heightmap: &Heightmap, from: Coord) -> Vec<Coord> {
    let from_node = heightmap.get_node(from).unwrap();
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
            let Some(node) = heightmap.get_node(*coord) else { return false; };
            from_height + 1 >= node.height()
        })
        .collect()
}

mod shortest_path {
    use super::reachable;
    use super::Coord;
    use super::Heightmap;
    use super::Node;
    use std::cmp::Ordering;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

    /// Dijkstra's algo
    pub(super) fn shortest_path(heightmap: &Heightmap, start: Coord) -> Option<usize> {
        // Min-heap of edges by cumulative cost.
        let mut edges: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
        let mut visited = HashSet::<Coord>::new();

        for coord in reachable(heightmap, start) {
            let edge = Edge {
                to: coord,
                cumulative_cost: 1,
            };
            edges.push(Reverse(edge));
        }
        visited.insert(start);

        while let Some(Reverse(edge)) = edges.pop() {
            if visited.contains(&edge.to) {
                continue;
            }
            visited.insert(edge.to);

            let node = heightmap.get_node(edge.to).unwrap();
            match node {
                Node::Start | Node::Normal(_) => {
                    for coord in reachable(heightmap, edge.to) {
                        if visited.contains(&coord) {
                            continue;
                        }

                        let e = Edge {
                            to: coord,
                            cumulative_cost: edge.cumulative_cost + 1,
                        };
                        edges.push(Reverse(e));
                    }
                }
                Node::End => {
                    return Some(edge.cumulative_cost);
                }
            }
        }

        None
    }

    /// Edge in shortest-path search.
    ///
    /// Implements PartialOrd/Ord by comparing cumulative cost.
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    struct Edge {
        /// Destination node.
        to: Coord,

        /// Cumulative cost from start node.
        cumulative_cost: usize,
    }
    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cumulative_cost.cmp(&other.cumulative_cost)
        }
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
